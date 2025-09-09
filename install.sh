#!/bin/bash

# A script to automatically build and install the 'check' CLI tool.
# 'dev' mode is now the default for faster iteration.
#
# Usage:
#   ./install_check.sh           (for a fast, unoptimized debug build)
#   ./install_check.sh release   (for a slow, optimized release build)

# Exit immediately if a command exits with a non-zero status.
set -e

# --- Speed-up Configuration ---
# Attempt to use the 'lld' linker for faster link times, a common bottleneck.
if command -v lld &> /dev/null; then
    echo "⚡ Found 'lld' linker. Configuring for faster builds."
    export RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=lld"
else
    echo "💡 Tip: Install 'lld' for faster Rust build times (e.g., 'sudo apt-get install lld')."
fi


# --- Configuration ---
# The directory where your 'check-cli' crate is located.
PROJECT_DIR="crates/check_cli"
BINARY_NAME="check"
INSTALL_DIR="$HOME/.cargo/bin"

# --- Script Logic ---

# Determine build mode. Default is 'debug' for speed.
BUILD_MODE="debug"
if [[ "$1" == "release" ]]; then
  BUILD_MODE="release"
fi

echo "🚀 Starting installation for '$BINARY_NAME' (Mode: $BUILD_MODE)..."

# 1. Check if the project directory exists
if [ ! -d "$PROJECT_DIR" ]; then
  echo "❌ Error: Project directory not found at './${PROJECT_DIR}'"
  echo "Please make sure you are in the correct root directory and the path is correct."
  exit 1
fi

# 2. Navigate into the project directory
echo "➡️  Changing directory to '$PROJECT_DIR'..."
cd "$PROJECT_DIR"

# 3. Build and install based on the mode
if [[ "$BUILD_MODE" == "release" ]]; then
  echo "🛠️  Building and installing in release mode (slower compile, fast runtime)..."
  cargo install --path .
else
  echo "🛠️  Building in debug mode (faster compile, slow runtime)..."
  cargo build
  echo "📥 Copying debug binary to '$INSTALL_DIR'..."
  # Ensure the installation directory exists
  mkdir -p "$INSTALL_DIR"
  # The source binary is named after the crate ('check_cli'),
  # but we copy and rename it to the desired command name ('check').
  cp "target/debug/check_cli" "$INSTALL_DIR/$BINARY_NAME"
fi


# 4. Final success message
echo "✅ Success! The '$BINARY_NAME' command is now installed."
echo "Go build something 10x faster, manfromexistence! 🔥"
