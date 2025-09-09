import type { Configuration, Diagnostic } from "@checkjs/wasm-bundler";
import * as moduleBundler from "@checkjs/wasm-bundler";
import { CheckCommon } from "./common";

export type * from "./common";
export type { Configuration, Diagnostic };

export class Check extends CheckCommon<Configuration, Diagnostic> {
	constructor() {
		super(moduleBundler);
	}
}
