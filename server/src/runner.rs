use reqwest::Client;

use crate::github::get_jit_config;

pub async fn run_job(
    client: &Client,
    token: &str,
    owner: &str,
    repo: &str,
    job_dir: &str,
    job_id: u64,
    template_dir: &str,
) -> anyhow::Result<()> {
    let start_time = std::time::Instant::now();

    let runner = format!("paddock-{}", job_id);
    let config = get_jit_config(client, token, owner, repo, &runner).await?;
    let location = format!("{}/{}", job_dir, job_id);
    let cp_status = tokio::process::Command::new("cp")
        .arg("-a")
        .arg(template_dir)
        .arg(&location)
        .status()
        .await?;

    if !cp_status.success() {
        let err = cp_status.to_string();
        tokio::fs::remove_dir_all(location).await?;
        anyhow::bail!(err)
    }
    let output = tokio::process::Command::new("./run.sh")
        .arg("--jitconfig")
        .arg(config.encoded_jit_config)
        .current_dir(&location)
        .status()
        .await?;
    if !output.success() {
        let err = output.to_string();
        tokio::fs::remove_dir_all(location).await?;
        anyhow::bail!(err)
    }
    tokio::fs::remove_dir_all(location).await?;
    println!(
        "job: {} finished {} in {:?}",
        job_id,
        output,
        start_time.elapsed()
    );
    Ok(())
}
