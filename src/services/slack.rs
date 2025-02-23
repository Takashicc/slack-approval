use std::collections::HashSet;
use std::sync::Arc;

use anyhow::{Context, Result};
use slack_morphism::prelude::*;
use tracing::info;

use crate::services::github::github_info::GitHubInfo;
use crate::services::github::github_inputs::GitHubInputs;

const SLACK_APPROVAL_APPROVE_ACTION_ID: &str = "slack-approval-approve";
const SLACK_APPROVAL_REJECT_ACTION_ID: &str = "slack-approval-reject";

pub async fn handle_slack_approval(
    github_info: &GitHubInfo,
    github_inputs: &GitHubInputs,
) -> Result<()> {
    let client = Arc::new(SlackClient::new(
        SlackClientHyperHttpsConnector::new().with_context(|| "Failed to create slack client")?,
    ));
    let token = SlackApiToken::new(github_inputs.bot_token.clone());
    let session = client.open_session(&token);

    let authorized_users = collect_authorized_users(&session, github_inputs)
        .await
        .with_context(|| "Failed to collect authorized users")?;

    post_message(
        &session,
        &github_inputs.channel_id,
        build_content(github_inputs, github_info),
    )
    .await?;

    let listener_environment = Arc::new(
        SlackClientEventsListenerEnvironment::new(client.clone()).with_user_state(
            SlackApprovalActionState {
                channel_id: github_inputs.channel_id.clone(),
                api_token: token,
                authorized_users,
            },
        ),
    );
    let socket_mode_callbacks = SlackSocketModeListenerCallbacks::new()
        .with_interaction_events(handle_slack_interaction_events);
    let socket_mode_listener = SlackClientSocketModeListener::new(
        &SlackClientSocketModeConfig::new(),
        listener_environment,
        socket_mode_callbacks,
    );
    socket_mode_listener
        .listen_for(&SlackApiToken::new(github_inputs.app_token.clone()))
        .await
        .with_context(|| "Failed to listen for slack socket mode. Have you enabled socket mode in your slack app?")?;
    socket_mode_listener.serve().await;

    Ok(())
}

struct SlackApprovalActionState {
    channel_id: SlackChannelId,
    api_token: SlackApiToken,
    authorized_users: Vec<SlackUserId>,
}

async fn handle_slack_interaction_events(
    event: SlackInteractionEvent,
    client: Arc<SlackHyperClient>,
    user_state: SlackClientEventsUserState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match event {
        SlackInteractionEvent::BlockActions(block_actions) => {
            let user_state_read = user_state.read().await;
            let state = user_state_read
                .get_user_state::<SlackApprovalActionState>()
                .with_context(|| "Failed to get slack approval action state")?;

            let user_id = block_actions.user.unwrap().id;
            let ts = block_actions.message.clone().unwrap().origin.ts;
            let blocks = block_actions
                .message
                .clone()
                .unwrap()
                .content
                .blocks
                .unwrap();

            let session = client.open_session(&state.api_token);

            if let Some(action) = block_actions.actions.unwrap().into_iter().next() {
                match action.action_id.0.as_ref() {
                    SLACK_APPROVAL_APPROVE_ACTION_ID => {
                        match approve_action(&session, state, &user_id, &blocks, &ts).await {
                            Err(e) => return Err(e.into()),
                            Ok(should_exit) => {
                                if should_exit {
                                    std::process::exit(0);
                                }
                            }
                        }
                    }
                    SLACK_APPROVAL_REJECT_ACTION_ID => {
                        match reject_action(&session, state, &user_id, &blocks, &ts).await {
                            Err(e) => return Err(e.into()),
                            Ok(should_exit) => {
                                if should_exit {
                                    std::process::exit(1);
                                }
                            }
                        }

                        std::process::exit(1);
                    }
                    _ => unimplemented!("Action not implemented: {:?}", action.action_id.0),
                }
            }
        }
        _ => unimplemented!("Event not implemented: {:?}", event),
    }
    Ok(())
}

// When user clicks on approve button
// Returns true when the right user clicks on the approve button
async fn approve_action<SDHC>(
    session: &SlackClientSession<'_, SDHC>,
    state: &SlackApprovalActionState,
    user_id: &SlackUserId,
    blocks: &Vec<SlackBlock>,
    ts: &SlackTs,
) -> Result<bool>
where
    SDHC: SlackClientHttpConnector + Send,
{
    info!("Approve button clicked by: {}", user_id);

    if !is_authorized_user(&user_id, &state.authorized_users) {
        info!("User is not authorized to approve: {}", user_id);

        let content = SlackMessageContent::new().with_text(format!(
            "You are not authorized to approve this action: {}",
            user_id
        ));
        post_message(session, &state.channel_id, content).await?;

        return Ok(false);
    }

    info!("User is authorized to approve: {}", user_id);
    let mut response_blocks = blocks.clone();
    response_blocks.pop();
    response_blocks
        .push(SlackBlock::Section(SlackSectionBlock::new().with_text(
            md!(format!("Approved by {}", user_id.to_slack_format())),
        )));

    update_message(session, &state.channel_id, response_blocks, ts).await?;

    Ok(true)
}

// When user clicks on reject button
// Returns true when the right user clicks on the reject button
async fn reject_action<SDHC>(
    session: &SlackClientSession<'_, SDHC>,
    state: &SlackApprovalActionState,
    user_id: &SlackUserId,
    blocks: &Vec<SlackBlock>,
    ts: &SlackTs,
) -> Result<bool>
where
    SDHC: SlackClientHttpConnector + Send,
{
    info!("Reject button clicked by: {}", user_id);

    if !is_authorized_user(&user_id, &state.authorized_users) {
        info!("User is not authorized to reject: {}", user_id);

        let content = SlackMessageContent::new().with_text(format!(
            "You are not authorized to reject this action: {}",
            user_id
        ));
        post_message(session, &state.channel_id, content).await?;

        return Ok(false);
    }

    info!("User is authorized to reject: {}", user_id);
    let mut response_blocks = blocks.clone();
    response_blocks.pop();
    response_blocks
        .push(SlackBlock::Section(SlackSectionBlock::new().with_text(
            md!(format!("Rejected by {}", user_id.to_slack_format())),
        )));

    update_message(session, &state.channel_id, response_blocks, ts).await?;

    Ok(true)
}

fn build_header(inputs: &GitHubInputs) -> String {
    let mut header = String::new();
    if !inputs.mention_to_users.is_empty() {
        header.push_str(
            &inputs
                .mention_to_users
                .iter()
                .map(|user| user.to_slack_format())
                .collect::<Vec<String>>()
                .join(" "),
        );
    }

    if !inputs.mention_to_groups.is_empty() {
        header.push_str(
            &inputs
                .mention_to_groups
                .iter()
                .map(|group| format!("<!subteam^{}>", group))
                .collect::<Vec<String>>()
                .join(" "),
        );
    }

    header
}

fn build_content(github_inputs: &GitHubInputs, github_info: &GitHubInfo) -> SlackMessageContent {
    SlackMessageContent::new().with_blocks(slack_blocks![
        some_into(SlackSectionBlock::new().with_text(md!(build_header(github_inputs)))),
        some_into(SlackSectionBlock::new().with_fields(vec![
            md!(format!("*Actor:*\n{}", github_info.github_actor)),
            md!(format!(
                "*Repository:*\n{}",
                github_info.get_repository_url()
            )),
            md!(format!("*Action:*\n{}", github_info.get_action_url())),
            md!(format!("*Run ID:*\n{}", github_info.github_run_id)),
            md!(format!("*Workflow:*\n{}", github_info.github_workflow)),
            md!(format!("*Runner:*\n{}", github_info.runner_os))
        ])),
        some_into(SlackActionsBlock::new(slack_blocks!(
            some_into(
                SlackBlockButtonElement::new(
                    SLACK_APPROVAL_APPROVE_ACTION_ID.into(),
                    pt!("Approve")
                )
                .with_style("primary".into())
                .with_value("approve".into())
            ),
            some_into(
                SlackBlockButtonElement::new(SLACK_APPROVAL_REJECT_ACTION_ID.into(), pt!("Reject"))
                    .with_style("danger".into())
                    .with_value("reject".into())
            )
        )))
    ])
}

async fn fetch_user_ids_from_groups<SCHC>(
    session: &SlackClientSession<'_, SCHC>,
    authorized_groups: &Vec<SlackUserGroupId>,
) -> Result<Vec<SlackUserId>>
where
    SCHC: SlackClientHttpConnector + Send,
{
    let mut user_ids = vec![];

    for group in authorized_groups {
        let res = session
            .usergroups_users_list(&SlackApiUserGroupsUsersListRequest::new(SlackUserGroupId(
                group.to_string(),
            )))
            .await
            .with_context(|| format!("Failed to fetch user IDs from group. group: {}", group))?;
        user_ids.extend(res.users);
    }

    info!("User IDs from groups: {:?}", user_ids);

    Ok(user_ids)
}

async fn collect_authorized_users<SCHC>(
    session: &SlackClientSession<'_, SCHC>,
    github_inputs: &GitHubInputs,
) -> Result<Vec<SlackUserId>>
where
    SCHC: SlackClientHttpConnector + Send,
{
    let mut authorized_users: Vec<SlackUserId> = vec![];
    authorized_users.extend(github_inputs.authorized_users.clone());

    if !github_inputs.authorized_groups.is_empty() {
        let user_ids =
            fetch_user_ids_from_groups(session, &github_inputs.authorized_groups).await?;
        authorized_users.extend_from_slice(&user_ids);
    }

    // Remove duplicates
    let mut hash_set = HashSet::new();
    authorized_users.retain(|e| hash_set.insert(e.clone()));

    info!("Authorized users: {:?}", authorized_users);

    Ok(authorized_users)
}

fn is_authorized_user(user_id: &SlackUserId, authorized_users: &[SlackUserId]) -> bool {
    if authorized_users.is_empty() {
        return true;
    }

    authorized_users.contains(user_id)
}

async fn post_message<SDHC>(
    session: &SlackClientSession<'_, SDHC>,
    channel_id: &SlackChannelId,
    content: SlackMessageContent,
) -> Result<()>
where
    SDHC: SlackClientHttpConnector + Send,
{
    session
        .chat_post_message(&SlackApiChatPostMessageRequest::new(
            channel_id.clone(),
            content,
        ))
        .await
        .with_context(|| format!("Failed to post message. channel_id: {}", channel_id))?;

    Ok(())
}

async fn update_message<SDHC>(
    session: &SlackClientSession<'_, SDHC>,
    channel_id: &SlackChannelId,
    blocks: Vec<SlackBlock>,
    ts: &SlackTs,
) -> Result<()>
where
    SDHC: SlackClientHttpConnector + Send,
{
    session
        .chat_update(&SlackApiChatUpdateRequest::new(
            channel_id.clone(),
            SlackMessageContent::new().with_blocks(blocks),
            ts.clone(),
        ))
        .await
        .with_context(|| {
            format!(
                "Failed to update message. channel_id: {}, ts: {}",
                channel_id, ts,
            )
        })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::is_authorized_user;
    use rstest::rstest;
    use slack_morphism::SlackUserId;

    #[rstest]
    #[case("U1", vec![], true)]
    #[case("U1", vec!["U2".into(), "U3".into()], false)]
    #[case("U1", vec!["U2".into(), "U1".into()], true)]
    fn test_is_authorized_user(
        #[case] user_id: &str,
        #[case] authorized_users: Vec<SlackUserId>,
        #[case] expected: bool,
    ) {
        let actual = is_authorized_user(&SlackUserId::new(user_id.into()), &authorized_users);
        assert_eq!(actual, expected);
    }
}
