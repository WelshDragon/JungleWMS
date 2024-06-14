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

RUN cargo build --release -vv -p gorillaauth_service

FROM rustlang/rust:nightly-alpine as runner

WORKDIR /app

COPY --from=builder /work/target/release/gorillaauth_service /app/
COPY --from=builder /work/Cargo.toml /app/

EXPOSE $PORT

CMD ["/app/gorillaauth_service"]