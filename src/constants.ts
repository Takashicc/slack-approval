export const Inputs = {
	BotToken: "bot-token",
	SigningSecret: "signing-secret",
	AppToken: "app-token",
	ChannelId: "channel-id",
	MentionToUser: "mention-to-user",
	MentionToGroup: "mention-to-group",
	AuthorizedUsers: "authorized-users",
	AuthorizedGroups: "authorized-groups",
} as const;

export type Inputs = (typeof Inputs)[keyof typeof Inputs];
