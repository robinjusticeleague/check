import type {
	Configuration as ConfigurationBundler,
	Diagnostic as DiagnosticBundler,
} from "@checkjs/wasm-bundler";
import type {
	Configuration as ConfigurationNodejs,
	Diagnostic as DiagnosticNodeJs,
} from "@checkjs/wasm-nodejs";
import type {
	Configuration as ConfigurationWeb,
	Diagnostic as DiagnosticWeb,
} from "@checkjs/wasm-web";
import { CheckCommon } from "./common";

export type * from "./common";
export type Configuration =
	| ConfigurationBundler
	| ConfigurationNodejs
	| ConfigurationWeb;
export type Diagnostic = DiagnosticBundler | DiagnosticNodeJs | DiagnosticWeb;

/**
 * What kind of client Check should use to communicate with the binary
 */
export enum Distribution {
	/**
	 * Use this if you want to communicate with the WebAssembly client built for
	 * bundlers
	 */
	BUNDLER = 0,
	/**
	 * Use this if you want to communicate with the WebAssembly client built for
	 * Node.JS
	 */
	NODE = 1,
	/**
	 * Use this if you want to communicate with the WebAssembly client built for
	 * the Web
	 */
	WEB = 2,
}

export interface CheckCreate {
	distribution: Distribution;
}

export class Check extends CheckCommon<Configuration, Diagnostic> {
	/**
	 * It creates a new instance of the class {Check}.
	 */
	static async create({ distribution }: CheckCreate): Promise<Check> {
		switch (distribution) {
			case Distribution.BUNDLER:
				return new Check(await import("@checkjs/wasm-bundler"));
			case Distribution.NODE:
				return new Check(await import("@checkjs/wasm-nodejs"));
			case Distribution.WEB:
				return new Check(await import("@checkjs/wasm-web"));
			default:
				throw new Error(`Unknown distribution: ${distribution}`);
		}
	}
}
