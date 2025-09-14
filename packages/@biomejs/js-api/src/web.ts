import type { Configuration, Diagnostic } from "@checkjs/wasm-web";
import * as moduleWeb from "@checkjs/wasm-web";
import { CheckCommon } from "./common";

export type * from "./common";
export type { Configuration, Diagnostic };

export class Check extends CheckCommon<Configuration, Diagnostic> {
	constructor() {
		super(moduleWeb);
	}
}
