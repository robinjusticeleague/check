/* should not generate diagnostics */
// https://github.com/checkjs/check/issues/105

const tid = setInterval(() => {
    clearInterval(tid);
});