# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine as builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen
    # protoc openssl-dev protobuf-dev gcc git g++ libc-dev make binaryen

RUN npm install -g sass

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN cargo install --locked cargo-leptos
RUN cargo leptos build --release -vv -p gorillaauth_website

FROM rustlang/rust:nightly-alpine as runner

WORKDIR /app

COPY --from=builder /work/target/release/gorillaauth_website /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

EXPOSE $PORT
ENV LEPTOS_SITE_ROOT=./site

CMD ["/app/gorillaauth_website"]