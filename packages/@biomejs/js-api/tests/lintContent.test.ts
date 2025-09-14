import { afterEach, beforeEach, describe, expect, it } from "vitest";
import type { ProjectKey } from "../../backend-jsonrpc/dist";
import { Check, Distribution } from "../dist";

describe("Check WebAssembly lintContent", () => {
	const inputCode = `
	debugger;
	const b = /   /;
	`;

	let check: Check;
	let projectKey: ProjectKey;
	beforeEach(async () => {
		check = await Check.create({
			distribution: Distribution.NODE,
		});
		const result = check.openProject();
		projectKey = result.projectKey;
	});

	afterEach(() => {
		check.shutdown();
	});

	describe("fixFileMode is undefined/omitted", () => {
		it("should emit diagnostics", () => {
			const result = check.lintContent(projectKey, inputCode, {
				filePath: "example.js",
			});
			const categories = result.diagnostics.map((d) => d.category);
			expect(categories).toMatchObject([
				"lint/suspicious/noDebugger",
				"lint/complexity/noAdjacentSpacesInRegex",
				"lint/correctness/noUnusedVariables",
			]);
		});
		it("should not fix the code", () => {
			const result = check.lintContent(projectKey, inputCode, {
				filePath: "example.js",
			});
			expect(result.content).toMatchSnapshot();
		});
	});

	describe("fixFileMode is SafeFixes", () => {
		it("should emit diagnostics", () => {
			const result = check.lintContent(projectKey, inputCode, {
				filePath: "example.js",
				fixFileMode: "safeFixes",
			});
			const categories = result.diagnostics.map((d) => d.category);
			expect(categories).toMatchObject([
				"lint/suspicious/noDebugger",
				"lint/correctness/noUnusedVariables",
			]);
		});
		it("should fix the SafeFixes only", () => {
			const result = check.lintContent(projectKey, inputCode, {
				filePath: "example.js",
				fixFileMode: "safeFixes",
			});
			expect(result.content).toMatchSnapshot();
		});
	});

	describe("fixFileMode is SafeAndUnsafeFixes", () => {
		it("should emit diagnostics", () => {
			const result = check.lintContent(projectKey, inputCode, {
				filePath: "example.js",
				fixFileMode: "safeAndUnsafeFixes",
			});
			expect(result.diagnostics).toHaveLength(0);
		});
		it("should fix the code", () => {
			const result = check.lintContent(projectKey, inputCode, {
				filePath: "example.js",
				fixFileMode: "safeAndUnsafeFixes",
			});
			expect(result.content).toMatchSnapshot();
		});
	});
});
