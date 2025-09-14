export type CheckPath = string;
export type ProjectKey = number;

interface UpdateSettingsParams<Configuration> {
	configuration: Configuration;
	projectKey: ProjectKey;
	workspaceDirectory?: CheckPath;
}

type TextRange = [TextSize, TextSize];

type TextSize = number;

interface OpenProjectParams {
	openUninitialized: boolean;
	path: CheckPath;
}
export interface OpenProjectResult {
	/**
	 * A unique identifier for this project
	 */
	projectKey: ProjectKey;
}
interface OpenFileParams {
	content: FileContent;
	path: CheckPath;
	projectKey: ProjectKey;
}
type FileContent = { content: string; type: "fromClient"; version: number };

interface CloseFileParams {
	path: CheckPath;
	projectKey: ProjectKey;
}

interface GetFormatterIRParams {
	path: CheckPath;
	projectKey: ProjectKey;
}

interface PullDiagnosticsParams {
	categories: RuleCategories;
	path: CheckPath;
	projectKey: ProjectKey;
	/**
	 * When `false` the diagnostics, don't have code frames of the code actions
	 * (fixes, suppressions, etc.)
	 */
	pullCodeActions: boolean;
}
type RuleCategories = RuleCategory[];
type RuleCategory = "syntax" | "lint" | "action" | "transformation";
interface PullDiagnosticsResult<Diagnostic> {
	diagnostics: Diagnostic[];
	errors: number;
}

interface FormatFileParams {
	path: CheckPath;
	projectKey: ProjectKey;
}

interface FormatRangeParams {
	path: CheckPath;
	projectKey: ProjectKey;
	range: TextRange;
}

interface FixFileParams {
	fixFileMode: FixFileMode;
	path: CheckPath;
	projectKey: ProjectKey;
	ruleCategories: RuleCategories;
	shouldFormat: boolean;
}
export type FixFileMode =
	| "safeFixes"
	| "safeAndUnsafeFixes"
	| "applySuppressions";
interface FixFileResult {
	/**
	 * New source code for the file with all fixes applied
	 */
	code: string;
}

export interface DiagnosticPrinter<Diagnostic> {
	free(): void;
	print_simple(diagnostic: Diagnostic): void;
	print_verbose(diagnostic: Diagnostic): void;
	finish(): string;
}
export interface Workspace<Configuration, Diagnostic> {
	free(): void;
	updateSettings(params: UpdateSettingsParams<Configuration>): void;
	openProject(params: OpenProjectParams): OpenProjectResult;
	openFile(params: OpenFileParams): void;
	closeFile(params: CloseFileParams): void;
	pullDiagnostics(
		params: PullDiagnosticsParams,
	): PullDiagnosticsResult<Diagnostic>;
	// check-ignore lint: code generation is broken
	formatRange(params: FormatRangeParams): any;
	// check-ignore lint: code generation is broken
	formatFile(params: FormatFileParams): any;
	getFormatterIr(params: GetFormatterIRParams): string;
	fixFile(params: FixFileParams): FixFileResult;
}

export interface Module<Configuration, Diagnostic> {
	main: () => void;
	DiagnosticPrinter: new (
		fileName: string,
		fileSource: string,
	) => DiagnosticPrinter<Diagnostic>;
	Workspace: new () => Workspace<Configuration, Diagnostic>;
}

/**
 * The error generated when communicating with WebAssembly
 */
class WasmError extends Error {
	/**
	 * The stack trace of the error.
	 *
	 * It might be useful, but the first like of the stack trace contains the
	 * error
	 */
	stackTrace: string;
	constructor(stackTrace: string) {
		super();
		this.stackTrace = stackTrace;
	}
}

/**
 * Catch a WebAssembly error and wrap into a native JS Error
 *
 * @param func The function to execute
 */
export function tryCatchWrapper<T>(func: () => T): T {
	try {
		return func();
	} catch (err) {
		throw new WasmError(err as string);
	}
}
