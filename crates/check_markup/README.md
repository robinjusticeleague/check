# `check_markup`

The crate contains procedural macros to build `check_console` markup object with a JSX-like syntax

The macro cannot be used alone as it generates code that requires supporting types declared in the
`check_console` crate, so it's re-exported from there and should be used as `check_console::markup`
