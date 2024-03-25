import { Option, isNone, isSome } from "fp-ts/lib/Option";

export function isAuthorizedUser(
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
