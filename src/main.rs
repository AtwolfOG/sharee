use axum::Router;
use tower_http::services::ServeDir;



#[tokio::main]
async fn main() {
    let app = Router::new().route_service("/", ServeDir::new("./static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
