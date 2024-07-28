use tower_http::services::{ServeFile, ServeDir};

#[tokio::main]
async fn main() {

    let static_files = axum::Router::new()
        .route_service("/", ServeFile::new("../templates/layout.html"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, static_files).await.unwrap();

}