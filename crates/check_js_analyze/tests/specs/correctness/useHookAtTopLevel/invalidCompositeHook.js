function useFoo() {
    return useBar();
}

function Component() {
    if (condition) {
        // This call should be reported just once.
        // See https://github.com/checkjs/check/issues/6393
        return useFoo();
    }
}
