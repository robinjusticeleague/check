/* should not generate diagnostics */
// Issue https://github.com/checkjs/check/issues/2637
declare module 'test-module' {
    const foo: any;
    export { foo };
}