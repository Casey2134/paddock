use axum::{
    Router,
    routing::post,
};
use axum::body::Bytes;
use axum::http::{
    HeaderMap,
    StatusCode,
};
mod webhook;
#[tokio::main]
async fn main() {
    let app= Router::new().route("/webhook", post(handle_webhook));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on port 3000");
    axum::serve(listner, app).await.unwrap();
} 

async fn handle_webhook(headers: HeaderMap, body: Bytes) -> StatusCode {
    let text = String::from_utf8_lossy(&body);
    let event = headers.get("X-Github-Event");
    let uid = headers.get("x-github-delivery");
    println!{"{}", text};
    if let Some(value) = uid && let Some(e_value) = event{
        if let Ok(uid_str) = value.to_str() && let Ok(event_str) = e_value.to_str(){
            let filename = format!("tests/fixtures/{}-{}.json", uid_str, event_str);
        
            if let Err(e) = std::fs::write(filename, &body){
                println!("{}", e);
            }
        }
    }
    if let Some(value) = event {
        if let Ok(event_name) = value.to_str() {
           println!("github event: {}", event_name);
            return StatusCode::OK;
        }else {
            return StatusCode::BAD_REQUEST;
        }
    }else{
        return StatusCode::BAD_REQUEST;
    }
}
