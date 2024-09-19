
use tokio::net::TcpListener;
use gorillaauth_service::app;
use gorillaauth_service::util;

#[tokio::main]
async fn main() {
    
    let server_address = "0.0.0.0:8080".to_string();
    let listener = TcpListener::bind(server_address)
        .await
        .expect("Unable to connect to server!");

    util::tracing::init();

    let routes = app();

    axum::serve(listener, routes)
        .await
        .expect("Error serving application");
}


