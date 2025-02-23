use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct GitHubInfo {
    pub github_server_url: String,
    pub github_repository: String,
    pub github_run_id: String,
    pub github_workflow: String,
    pub runner_os: String,
    pub github_actor: String,
}

impl GitHubInfo {
    pub fn action_url(&self) -> String {
        format!(
            "{}/{}/actions/runs/{}",
            self.github_server_url, self.github_repository, self.github_run_id
        )
    }

    pub fn repository_url(&self) -> String {
        format!("{}/{}", self.github_server_url, self.github_repository)
    }
}

pub fn read_github_info() -> Result<GitHubInfo> {
    envy::from_env::<GitHubInfo>()
        .with_context(|| "Failed to read GitHub info from environment variables")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_github_info() {
        std::env::set_var("GITHUB_SERVER_URL", "https://github.com");
        std::env::set_var("GITHUB_REPOSITORY", "octocat/Hello-World");
        std::env::set_var("GITHUB_RUN_ID", "42");
        std::env::set_var("GITHUB_WORKFLOW", "Hello-World-Workflow");
        std::env::set_var("RUNNER_OS", "Linux");
        std::env::set_var("GITHUB_ACTOR", "octocat");

        let expected = GitHubInfo {
            github_server_url: "https://github.com".into(),
            github_repository: "octocat/Hello-World".into(),
            github_run_id: "42".into(),
            github_workflow: "Hello-World-Workflow".into(),
            runner_os: "Linux".into(),
            github_actor: "octocat".into(),
        };
        let actual = read_github_info().unwrap();
        assert_eq!(actual, expected);
        assert_eq!(
            actual.action_url(),
            "https://github.com/octocat/Hello-World/actions/runs/42"
        );
        assert_eq!(
            actual.repository_url(),
            "https://github.com/octocat/Hello-World"
        );
    }
}
