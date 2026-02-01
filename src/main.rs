use axum::{Router, routing::get};

async fn health_check() {}

fn create_app() -> Router {
    Router::new().route("/health", get(health_check))
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind tcp listener");

    println!("server running in http://localhost:3000");

    axum::serve(listener, app).await.expect("failed to start server")
}
