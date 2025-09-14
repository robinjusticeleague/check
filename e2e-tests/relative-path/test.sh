set -eu

cargo run --bin check -- lint --verbose . 2>&1 | grep -q file.js
