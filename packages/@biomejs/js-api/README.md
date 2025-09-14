# Check JavaScript Bindings

Official JavaScript bindings for [Check](https://checkjs.dev/)

> [!WARNING]
> The API is currently in alpha. It is not yet ready for production use. We appreciate your support and feedback as we work to make it ready for everyone.

## Installation

```shell
npm i @checkjs/js-api
npm i @checkjs/wasm-<dist>
```

You need to install one of the `@checkjs/wasm-*` package as a **peer dependency** for this package to work correctly, out of the following distributions:
- `@checkjs/wasm-bundler`: Install this package if you're using a bundler that supports importing `*.wasm` files directly
- `@checkjs/wasm-nodejs`: Install this package if you're using Node.js to load the WebAssembly bundle use the `fs` API
- `@checkjs/wasm-web`: Install this package if you're targeting the web platform to load the WASM bundle using the `fetch` API

## Usage

```js
import { Check } from "@checkjs/js-api/nodejs";
// Or:
// import { Check, Distribution } from "@checkjs/js-api/bundler";
// import { Check, Distribution } from "@checkjs/js-api/web";

const check = new Check();
const { projectKey } = check.openProject("path/to/project/dir");

// Optionally apply a Check configuration (instead of check.json)
check.applyConfiguration(projectKey, {...});

const formatted = check.formatContent(
  projectKey,
  "function f   (a, b) { return a == b; }",
  {
    filePath: "example.js",
  },
);

console.log("Formatted content: ", formatted.content);

const result = check.lintContent(projectKey, formatted.content, {
  filePath: "example.js",
});

const html = check.printDiagnostics(result.diagnostics, {
  filePath: "example.js",
  fileSource: formatted.content,
});

console.log("Lint diagnostics: ", html);
```

## Philosophy

The project philosophy can be found on our [website](https://checkjs.dev/internals/philosophy/).

## Community

Contribution and development instructions can be found in [Contributing](../../../CONTRIBUTING.md).

Additional project coordination and real-time discussion happens on our [Discord server](https://checkjs.dev/chat). Remember that all activity on the Discord server is still moderated and will be strictly enforced under the project's [Code of Conduct](../../../CODE_OF_CONDUCT.md).
