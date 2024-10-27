import * as core from "@actions/core";
import { Option, none, some } from "fp-ts/lib/Option";
import { Inputs } from "../constants";
import { getOptionalInput, getOptionalListInput } from "./input_helper";

jest.mock("@actions/core");

describe("getOptionalInput", () => {
	const testInput = Inputs.AppToken;

	test("empty string", () => {
		(core.getInput as jest.Mock).mockReturnValue("");
		const actual = getOptionalInput(testInput);
		const expected: Option<string> = none;
		expect(actual).toStrictEqual(expected);
	});

	test("non-empty string", () => {
		(core.getInput as jest.Mock).mockReturnValue("item");
		const actual = getOptionalInput(testInput);
		const expected: Option<string> = some("item");
		expect(actual).toStrictEqual(expected);
	});
});

describe("getOptionalListInput", () => {
	const testInput = Inputs.AppToken;

	test("empty string", () => {
		(core.getInput as jest.Mock).mockReturnValue("");
		const actual = getOptionalListInput(testInput);
		const expected: Option<string[]> = none;
		expect(actual).toStrictEqual(expected);
	});

	test("single value", () => {
		(core.getInput as jest.Mock).mockReturnValue("item1");
		const actual = getOptionalListInput(testInput);
		const expected: Option<string[]> = some(["item1"]);
		expect(actual).toStrictEqual(expected);
	});

	test("multiple value without space", () => {
		(core.getInput as jest.Mock).mockReturnValue("item1,item2,item3");
		const actual = getOptionalListInput(testInput);
		const expected: Option<string[]> = some(["item1", "item2", "item3"]);
		expect(actual).toStrictEqual(expected);
	});

	test("multiple value with space", () => {
		(core.getInput as jest.Mock).mockReturnValue("item1, item2, item3");
		const actual = getOptionalListInput(testInput);
		const expected: Option<string[]> = some(["item1", "item2", "item3"]);
		expect(actual).toStrictEqual(expected);
	});
});
