use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WorkflowJobEvent {
    pub action: String,
    pub workflow_job: WorkflowJob,
    pub repository: Repository,
}

#[derive(Deserialize, Debug)]
pub struct WorkflowJob {
    pub id: u64,
    pub run_id: u64,
    pub name: String,
    pub labels: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub name: String,
    pub owner: Owner,
}

#[derive(Deserialize, Debug)]
pub struct Owner {
    pub login: String,
}

pub fn label_check(w_event: &WorkflowJobEvent) -> bool {
    let labels = &w_event.workflow_job.labels;
    labels.iter().any(|label| label == "paddock")
        && labels.iter().any(|label| label == "self-hosted")
}
