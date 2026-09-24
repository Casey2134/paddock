use axum::{
    Router,
    routing::post,
};
use axum::body::Bytes;
use axum::http::HeaderMap;
#[tokio::main]
async fn main() {
    let app= Router::new().route("/webhook", post(handle_webhook));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listner, app).await.unwrap();
} 

async fn handle_webhook(headers: HeaderMap, body: Bytes) {
    let text = String::from_utf8_lossy(&body);
    let event = headers.get("X-Github-Event");
    println!{"{:?}", header};
    println!{"{}", text};
}
