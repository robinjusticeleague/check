import type { Configuration, Diagnostic } from "@checkjs/wasm-nodejs";
import * as moduleNodeJs from "@checkjs/wasm-nodejs";
import { CheckCommon } from "./common";

export type * from "./common";
export type { Configuration, Diagnostic };

export class Check extends CheckCommon<Configuration, Diagnostic> {
	constructor() {
		super(moduleNodeJs);
	}
}
