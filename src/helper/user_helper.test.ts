import { Option, none, some } from "fp-ts/lib/Option";
import { isAuthorizedUser } from "./user_helper";

describe("isAuthorizedUser", () => {
	const defaultUserId = "U123456";
	const noneAuthorizedUsers: Option<string[]> = none;
	const noneAuthorizedGroupMembers: Option<string[]> = none;

	describe("authorized user", () => {
		test("when authorized users and authorized group members are not specified", () => {
			const actual = isAuthorizedUser(
				defaultUserId,
				noneAuthorizedUsers,
				noneAuthorizedGroupMembers,
			);
			expect(actual).toBeTruthy();
		});

		test("when the user is only in the authorized users", () => {
			const authorizedUsers: Option<string[]> = some(["U123456"]);
			const authorizedGroupMembers: Option<string[]> = some(["U111111"]);
			const actual = isAuthorizedUser(
				defaultUserId,
				authorizedUsers,
				authorizedGroupMembers,
			);
			expect(actual).toBeTruthy();
		});

		test("when the user is only in the authorized group members", () => {
			const authorizedUsers: Option<string[]> = some(["U222222"]);
			const authorizedGroupMembers: Option<string[]> = some(["U123456"]);
			const actual = isAuthorizedUser(
				defaultUserId,
				authorizedUsers,
				authorizedGroupMembers,
			);
			expect(actual).toBeTruthy();
		});

		test("when the user is in both the authorized users and authorized group members", () => {
			const authorizedUsers: Option<string[]> = some(["U111111", "U123456"]);
			const authorizedGroupMembers: Option<string[]> = some([
				"U123456",
				"U222222",
			]);
			const actual = isAuthorizedUser(
				defaultUserId,
				authorizedUsers,
				authorizedGroupMembers,
			);
			expect(actual).toBeTruthy();
		});
	});

	describe("unauthorized user", () => {
		test("when the user is not in the authorized users and authorized group members", () => {
			const authorizedUsers: Option<string[]> = some(["U111111"]);
			const authorizedGroupMembers: Option<string[]> = some(["U222222"]);
			const actual = isAuthorizedUser(
				defaultUserId,
				authorizedUsers,
				authorizedGroupMembers,
			);
			expect(actual).toBeFalsy();
		});
	});
});
