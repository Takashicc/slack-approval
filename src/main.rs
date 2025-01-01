use anyhow::Result;

mod services;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    execute().await
}

async fn execute() -> Result<()> {
    let github_info = services::github::github_info::read_github_info()?;
    let inputs = services::github::github_inputs::read_github_inputs()?;
    services::slack::handle_slack_approval(&github_info, &inputs).await
}
