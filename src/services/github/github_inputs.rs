use anyhow::Result;
use slack_morphism::{SlackApiTokenValue, SlackChannelId, SlackSigningSecret};

use super::input_utils::{get_list_input, get_required_input};

#[derive(PartialEq, Debug)]
pub struct GitHubInputs {
    pub bot_token: SlackApiTokenValue,
    pub signing_secret: SlackSigningSecret,
    pub app_token: SlackApiTokenValue,
    pub channel_id: SlackChannelId,
    pub mention_to_users: Vec<String>,
    pub mention_to_groups: Vec<String>,
    pub authorized_users: Vec<String>,
    pub authorized_groups: Vec<String>,
}

pub fn read_github_inputs() -> Result<GitHubInputs> {
    let v = GitHubInputs {
        bot_token: get_required_input("bot_token")?.into(),
        app_token: get_required_input("app_token")?.into(),
        signing_secret: get_required_input("signing_secret")?.into(),
        channel_id: get_required_input("channel_id")?.into(),
        mention_to_users: get_list_input("mention_to_users")?,
        mention_to_groups: get_list_input("mention_to_groups")?,
        authorized_users: get_list_input("authorized_users")?,
        authorized_groups: get_list_input("authorized_groups")?,
    };
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_github_inputs() {
        std::env::set_var("INPUT_BOT_TOKEN", "xoxb-bot-token");
        std::env::set_var("INPUT_APP_TOKEN", "xapp-app-token");
        std::env::set_var("INPUT_SIGNING_SECRET", "super_secret");
        std::env::set_var("INPUT_CHANNEL_ID", "C1234567890");
        std::env::set_var("INPUT_MENTION_TO_USERS", "U000001, U000002");
        std::env::set_var("INPUT_MENTION_TO_GROUPS", "G000001, G000002, G000003");
        std::env::set_var("INPUT_AUTHORIZED_USERS", "U000010, U000011");
        std::env::set_var("INPUT_AUTHORIZED_GROUPS", "G000031, G000032");

        let actual = read_github_inputs().unwrap();
        let expected = GitHubInputs {
            bot_token: "xoxb-bot-token".into(),
            app_token: "xapp-app-token".into(),
            signing_secret: "super_secret".into(),
            channel_id: "C1234567890".into(),
            mention_to_users: vec!["U000001".into(), "U000002".into()],
            mention_to_groups: vec!["G000001".into(), "G000002".into(), "G000003".into()],
            authorized_users: vec!["U000010".into(), "U000011".into()],
            authorized_groups: vec!["G000031".into(), "G000032".into()],
        };

        assert_eq!(actual, expected);
    }
}
