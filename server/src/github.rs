use std::str;

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct JitRequestBody {
    pub name: String,
    pub runner_group_id: u64,
    pub labels: Vec<String>,
    pub work_folder: String,
}
#[derive(Deserialize, Debug)]
pub struct JitConfig {
    pub encoded_jit_config: String,
    pub runner: Runner,
}
#[derive(Deserialize, Debug)]
pub struct Runner {
    pub id: u64,
}

pub async fn get_jit_config(
    client: &Client,
    token: &str,
    owner: &str,
    repo: &str,
    runner: &str,
) -> anyhow::Result<JitConfig> {
    let req_url = format!(
        "https://api.github.com/repos/{}/{}/actions/runners/generate-jitconfig",
        owner, repo
    );
    println!("{}", req_url);
    let body = JitRequestBody {
        name: String::from(runner),
        runner_group_id: 1,
        labels: vec!["self-hosted".to_string(), "paddock".to_string()],
        work_folder: String::from("_work"),
    };
    let response = client
        .post(req_url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    if !response.status().is_success() {
        let err = response.text().await?;
        anyhow::bail!(err)
    }
    Ok(response.json().await?)
}

pub async fn get_runners(
    client: &reqwest::Client,
    token: &String,
    owner: &String,
    repo: &String,
) -> anyhow::Result<String> {
    let req_url = format!(
        "https://api.github.com/repos/{}/{}/actions/runners",
        owner, repo
    );
    let response = client.get(&req_url).bearer_auth(token).send().await?;
    if !response.status().is_success() {
        let err = response.text().await?;
        anyhow::bail!(err)
    }
    Ok(response.text().await?)
}
