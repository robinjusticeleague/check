import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { Check, type ProjectKey } from "../dist/nodejs";

describe("Check for Node.js", () => {
	let check: Check;
	let projectKey: ProjectKey;
	beforeEach(() => {
		check = new Check();
		const result = check.openProject();
		projectKey = result.projectKey;
	});

	afterEach(() => {
		check.shutdown();
	});

	it("should format content", () => {
		const result = check.formatContent(projectKey, "let foo  = 'bar'", {
			filePath: "example.js",
		});

		expect(result.content).toEqual('let foo = "bar";\n');
		expect(result.diagnostics).toEqual([]);
	});

	it("should emit diagnostics", () => {
		const result = check.lintContent(projectKey, "a { font-color: red }", {
			filePath: "example.css",
		});
		expect(result.diagnostics).toHaveLength(1);
		expect(result.diagnostics[0].description).toEqual(
			"Unknown property is not allowed.",
		);
	});
});
