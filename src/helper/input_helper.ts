import * as core from "@actions/core";
import { Option, none, some } from "fp-ts/lib/Option";
import { Inputs } from "../constants";

export type SlackApprovalInputs = {
	botToken: string;
	signingSecret: string;
	appToken: string;
	channelId: string;
	mentionToUser: Option<string>;
	mentionToGroup: Option<string>;
	authorizedUsers: Option<string[]>;
	authorizedGroups: Option<string[]>;
};

export function getInputs(): SlackApprovalInputs {
	const botToken = getRequiredInput(Inputs.BotToken);
	const signingSecret = getRequiredInput(Inputs.SigningSecret);
	const appToken = getRequiredInput(Inputs.AppToken);
	const channelId = getRequiredInput(Inputs.ChannelId);
	const mentionToUser = getOptionalInput(Inputs.MentionToUser);
	const mentionToGroup = getOptionalInput(Inputs.MentionToGroup);
	const authorizedUsers = getOptionalListInput(Inputs.AuthorizedUsers);
	const authorizedGroups = getOptionalListInput(Inputs.AuthorizedGroups);

	return {
		botToken,
		signingSecret,
		appToken,
		channelId,
		mentionToUser,
		mentionToGroup,
		authorizedUsers,
		authorizedGroups,
	};
}

function getRequiredInput(name: Inputs): string {
	return core.getInput(name, { required: true });
}

function getOptionalInput(name: Inputs): Option<string> {
	const value = core.getInput(name);
	if (value === "") {
		return none;
	}

	return some(value);
}

function getOptionalListInput(name: Inputs): Option<string[]> {
	const value = core.getInput(name);
	if (value === "") {
		return none;
	}
	const res: string[] = [];
	const values = value.split(",");
	for (const v of values) {
		res.push(v.trim());
	}
	return some(res);
}
