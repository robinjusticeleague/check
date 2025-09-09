---
"@checkjs/check": patch
---

Fixed [#7212](https://github.com/checkjs/check/issues/7212), now the [`useOptionalChain`](https://checkjs.dev/linter/rules/use-optional-chain/) rule recognizes optional chaining using `typeof` (e.g., `typeof foo !== 'undefined' && foo.bar`).
