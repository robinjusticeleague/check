import { describe, expect, it } from "vitest";
import { Check } from "../dist/web";

describe("Check for web", () => {
	it("should export Check", () => {
		expect(Check).not.toBeUndefined();
	});
});
