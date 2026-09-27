use std::time::Duration;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::{Router, routing::post};
use reqwest::header::ACCEPT;
mod events;
use events::WorkflowJobEvent;
mod github;
mod runner;
mod webhook;
use crate::events::label_check;
use webhook::verify_webhook;

#[derive(Clone)]
struct AppState {
    github_token: String,
    secret: String,
    client: reqwest::Client,
    jobs_dir: String,
    runner_template_dir: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert(
        "x-github-api-version",
        HeaderValue::from_static("2022-11-28"),
    );
    let app_state = AppState {
        github_token: std::env::var("GITHUB_TOKEN").expect("set GITHUB_TOKEN"),
        secret: std::env::var("WEBHOOK_SECRET").expect("set WEBHOOK_SECRET"),
        client: reqwest::Client::builder()
            .user_agent("paddock/0.1.0")
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()?,
        jobs_dir: std::env::var("JOBS_DIR").expect("set JOBS_DIR"),
        runner_template_dir: std::env::var("RUNNER_TEMPLATE_DIR").expect("set RUNNER_TEMPLATE_DIR"),
    };
    let app = Router::new()
        .route("/webhook", post(handle_webhook))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("listening on port 3000");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handle_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    let Some(event) = headers.get("x-github-event") else {
        return StatusCode::BAD_REQUEST;
    };
    let Ok(event_str) = event.to_str() else {
        return StatusCode::BAD_REQUEST;
    };
    let Some(s_header) = headers.get("x-hub-signature-256") else {
        return StatusCode::UNAUTHORIZED;
    };
    let Ok(s_header_str) = s_header.to_str() else {
        return StatusCode::UNAUTHORIZED;
    };

    if !verify_webhook(state.secret.as_bytes(), &body, s_header_str) {
        return StatusCode::UNAUTHORIZED;
    }

    if event_str != "workflow_job" {
        return StatusCode::OK;
    }

    // extract json from body
    let Ok(w_event) = serde_json::from_slice::<WorkflowJobEvent>(&body) else {
        return StatusCode::BAD_REQUEST;
    };

    if w_event.action != "queued" {
        return StatusCode::OK;
    }

    if label_check(&w_event) {
        println!("Starting workflow job {}", w_event.workflow_job.id);
        tokio::spawn(async move {
            if let Err(e) = runner::run_job(
                &state.client,
                &state.github_token,
                &w_event.repository.owner.login,
                &w_event.repository.name,
                &state.jobs_dir,
                w_event.workflow_job.id,
                &state.runner_template_dir,
            )
            .await
            {
                println!("{:?}", e);
            } else {
                println!("job {} completed", w_event.workflow_job.id);
            }
        });
    }

    StatusCode::OK
}
