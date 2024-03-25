import * as core from "@actions/core";
import { App, BlockAction, LogLevel } from "@slack/bolt";
import { WebClient } from "@slack/web-api";
import { Option, isNone, isSome, none, some } from "fp-ts/lib/Option";
import { getGitHubInfo } from "./helper/github_info_helper";
import { SlackApprovalInputs, getInputs } from "./helper/input_helper";

async function run(inputs: SlackApprovalInputs, app: App): Promise<void> {
	try {
		const web = new WebClient(inputs.botToken);

		const githubInfo = getGitHubInfo();

		let title = "";
		if (isSome(inputs.mentionToUser)) {
			title += `<@${inputs.mentionToUser.value}>\n`;
		}
		if (isSome(inputs.mentionToGroup)) {
			title += `<!subteam^${inputs.mentionToGroup.value}>\n`;
		}
		title += "*GitHub Action Approval request*";

		let authorizedGroupMembers: Option<string[]> = none;
		if (isSome(inputs.authorizedGroups)) {
			const members: string[] = [];
			for (const group of inputs.authorizedGroups.value) {
				try {
					const usersInGroup = await web.usergroups.users.list({
						usergroup: group,
					});
					if (!usersInGroup.ok) {
						throw new Error(`Failed to get users in user group: ${group}`);
					}
					if (usersInGroup.users) {
						members.push(...usersInGroup.users);
					}
				} catch (error) {
					console.error(error);
					process.exit(1);
				}
			}
			authorizedGroupMembers = some(members);
		}

		(async () => {
			await web.chat.postMessage({
				channel: inputs.channelId,
				blocks: [
					{
						type: "section",
						text: {
							type: "mrkdwn",
							text: title,
						},
					},
					{
						type: "section",
						fields: [
							{
								type: "mrkdwn",
								text: `*GitHub Actor:*\n${githubInfo.actor}`,
							},
							{
								type: "mrkdwn",
								text: `*Repos:*\n${githubInfo.serverUrl}/${githubInfo.repo}`,
							},
							{
								type: "mrkdwn",
								text: `*Actions URL:*\n${githubInfo.actionUrl}`,
							},
							{
								type: "mrkdwn",
								text: `*GITHUB_RUN_ID:*\n${githubInfo.runId}`,
							},
							{
								type: "mrkdwn",
								text: `*Workflow:*\n${githubInfo.workflow}`,
							},
							{
								type: "mrkdwn",
								text: `*RunnerOS:*\n${githubInfo.runnerOS}`,
							},
						],
					},
					{
						type: "actions",
						elements: [
							{
								type: "button",
								text: {
									type: "plain_text",
									emoji: true,
									text: "Approve",
								},
								style: "primary",
								value: "approve",
								action_id: "slack-approval-approve",
							},
							{
								type: "button",
								text: {
									type: "plain_text",
									emoji: true,
									text: "Reject",
								},
								style: "danger",
								value: "reject",
								action_id: "slack-approval-reject",
							},
						],
					},
				],
			});
		})();

		app.action(
			"slack-approval-approve",
			async ({ ack, client, body, logger }) => {
				await ack();

				const blockAction = <BlockAction>body;
				const userId = blockAction.user.id;
				const ts = blockAction.message?.ts || "";

				if (
					!isAuthorizedUser(
						userId,
						inputs.authorizedUsers,
						authorizedGroupMembers,
					)
				) {
					await client.chat.postMessage({
						channel: inputs.channelId,
						thread_ts: ts,
						text: `You are not authorized to approve this action: <@${userId}>`,
					});
					return;
				}

				try {
					const response_blocks = blockAction.message?.blocks;
					response_blocks.pop();
					response_blocks.push({
						type: "section",
						text: {
							type: "mrkdwn",
							text: `Approved by <@${userId}> `,
						},
					});

					await client.chat.update({
						channel: inputs.channelId,
						ts: ts,
						blocks: response_blocks,
					});
				} catch (error) {
					logger.error(error);
				}

				process.exit(0);
			},
		);

		app.action(
			"slack-approval-reject",
			async ({ ack, client, body, logger }) => {
				await ack();

				const blockAction = <BlockAction>body;
				const userId = blockAction.user.id;
				const ts = blockAction.message?.ts || "";

				if (
					!isAuthorizedUser(
						userId,
						inputs.authorizedUsers,
						authorizedGroupMembers,
					)
				) {
					await client.chat.postMessage({
						channel: inputs.channelId,
						thread_ts: ts,
						text: `You are not authorized to reject this action: <@${userId}>`,
					});
					return;
				}

				try {
					const response_blocks = blockAction.message?.blocks;
					response_blocks.pop();
					response_blocks.push({
						type: "section",
						text: {
							type: "mrkdwn",
							text: `Rejected by <@${userId}>`,
						},
					});

					await client.chat.update({
						channel: inputs.channelId,
						ts: ts,
						blocks: response_blocks,
					});
				} catch (error) {
					logger.error(error);
				}

				process.exit(1);
			},
		);

		(async () => {
			await app.start(3000);
			console.log("Waiting Approval reaction.....");
		})();
	} catch (error) {
		if (error instanceof Error) core.setFailed(error.message);
	}
}

function isAuthorizedUser(
	userId: string,
	authorizedUsers: Option<string[]>,
	authorizedGroupMembers: Option<string[]>,
): boolean {
	if (isNone(authorizedUsers) && isNone(authorizedGroupMembers)) {
		return true;
	}

	if (isSome(authorizedUsers)) {
		if (authorizedUsers.value.includes(userId)) {
			return true;
		}
	}

	if (isSome(authorizedGroupMembers)) {
		if (authorizedGroupMembers.value.includes(userId)) {
			return true;
		}
	}

	return false;
}

async function main() {
	const inputs = getInputs();

	const app = new App({
		token: inputs.botToken,
		signingSecret: inputs.signingSecret,
		appToken: inputs.appToken,
		socketMode: true,
		port: 3000,
		logLevel: LogLevel.DEBUG,
	});

	run(inputs, app);
}

main();
