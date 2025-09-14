/* should not generate diagnostics */
// Issue https://github.com/checkjs/check/issues/2659
type Test<T> = T extends Array<infer U> ? true : false