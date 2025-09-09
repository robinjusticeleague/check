set -eu

! cargo run --bin check -- lint . 2>&1 | grep -q debugger
