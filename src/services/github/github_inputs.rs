use anyhow::Result;
use slack_morphism::{SlackApiTokenValue, SlackChannelId, SlackUserGroupId, SlackUserId};

use super::input_utils::{get_list_input, get_required_input};

#[derive(PartialEq, Debug)]
pub struct GitHubInputs {
    pub bot_token: SlackApiTokenValue,
    pub app_token: SlackApiTokenValue,
    pub channel_id: SlackChannelId,
    pub mention_to_users: Vec<SlackUserId>,
    pub mention_to_groups: Vec<SlackUserGroupId>,
    pub authorized_users: Vec<SlackUserId>,
    pub authorized_groups: Vec<SlackUserGroupId>,
}

pub fn read_github_inputs() -> Result<GitHubInputs> {
    Ok(GitHubInputs {
        bot_token: get_required_input("bot-token")?.into(),
        app_token: get_required_input("app-token")?.into(),
        channel_id: get_required_input("channel-id")?.into(),
        mention_to_users: to_slack_user_id(get_list_input("mention-to-users")?),
        mention_to_groups: to_slack_user_group_id(get_list_input("mention-to-groups")?),
        authorized_users: to_slack_user_id(get_list_input("authorized-users")?),
        authorized_groups: to_slack_user_group_id(get_list_input("authorized-groups")?),
    })
}

fn to_slack_user_id(v: Vec<String>) -> Vec<SlackUserId> {
    v.into_iter().map(|v| v.into()).collect()
}

fn to_slack_user_group_id(v: Vec<String>) -> Vec<SlackUserGroupId> {
    v.into_iter().map(|v| v.into()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_github_inputs() {
        std::env::set_var("INPUT_BOT-TOKEN", "xoxb-bot-token");
        std::env::set_var("INPUT_APP-TOKEN", "xapp-app-token");
        std::env::set_var("INPUT_CHANNEL-ID", "C1234567890");
        std::env::set_var("INPUT_MENTION-TO-USERS", "U000001, U000002");
        std::env::set_var("INPUT_MENTION-TO-GROUPS", "G000001, G000002, G000003");
        std::env::set_var("INPUT_AUTHORIZED-USERS", "U000010, U000011");
        std::env::set_var("INPUT_AUTHORIZED-GROUPS", "G000031, G000032");

        let actual = read_github_inputs().unwrap();
        let expected = GitHubInputs {
            bot_token: "xoxb-bot-token".into(),
            app_token: "xapp-app-token".into(),
            channel_id: "C1234567890".into(),
            mention_to_users: vec!["U000001".into(), "U000002".into()],
            mention_to_groups: vec!["G000001".into(), "G000002".into(), "G000003".into()],
            authorized_users: vec!["U000010".into(), "U000011".into()],
            authorized_groups: vec!["G000031".into(), "G000032".into()],
        };

        assert_eq!(actual, expected);
    }
}
