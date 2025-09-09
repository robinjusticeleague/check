# `check_cli`

The main binary distribution, exposes the command line interface defined in this crate,
and the language server interface defined in `check_lsp` and used by the `check` VSCode extension

## Logs

When the server is run in daemon mode,
it will output logs to a file created in the `check-logs` directory inside the check cache directory.
This directory is typically `~/.cache/check` on Linux,
`C:\Users<UserName>\AppData\Local\checkjs\check\cache` on Windows,
`/Users/<UserName>/Library/Caches/dev.checkjs.check` on macOS,
and the system's temporary directory on other platforms.
To obtain the precise path, execute the command `check __print_cache_dir`.
The log file will be rotated on a hourly basis.
