#!/bin/bash

# A script to automatically build and install the 'check' CLI tool.
# 'dev' mode is now the default for a fast, unoptimized build.
#
# Usage:
#   ./install_check.sh          (for a fast, unoptimized debug build)
#   ./install_check.sh release  (for a slow, optimized release build)

set -e  # Exit immediately if a command exits with a non-zero status.

# --- Speed-up Configuration ---
if command -v lld &> /dev/null; then
    echo "⚡ Found 'lld' linker. Configuring for faster builds."
    export RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=lld"
else
    echo "💡 Tip: Install 'lld' for faster Rust build times (e.g., 'sudo apt-get install lld')."
fi

# --- Configuration ---
PROJECT_DIR="crates/check_cli"
BINARY_NAME="check"  # This is the actual binary name Cargo produces

# --- Script Logic ---
BUILD_MODE="debug"
if [[ "$1" == "release" ]]; then
  BUILD_MODE="release"
fi

echo "🚀 Starting installation for '$BINARY_NAME' (Mode: $BUILD_MODE)..."

# 1. Check if the project directory exists
if [ ! -d "$PROJECT_DIR" ]; then
  echo "❌ Error: Project directory not found at './${PROJECT_DIR}'"
  exit 1
fi

# 2. Navigate into the project directory
echo "➡️  Changing directory to '$PROJECT_DIR'..."
cd "$PROJECT_DIR"

# 3. Build and install
if [[ "$BUILD_MODE" == "release" ]]; then
  echo "🛠️  Building in release mode (slower compile, fast runtime)..."
  cargo build --release

  SOURCE_BINARY="../../target/release/$BINARY_NAME"
  echo "🔍 Verifying that the binary ('$SOURCE_BINARY') was built successfully..."
  if [ -f "$SOURCE_BINARY" ]; then
    echo "📥 Copying release binary to /usr/local/bin/ with sudo..."
    sudo cp "$SOURCE_BINARY" "/usr/local/bin/$BINARY_NAME"
  else
    echo "❌ Error: Build failed. The binary '$SOURCE_BINARY' was not found."
    exit 1
  fi
else
  echo "🛠️  Building in debug mode (faster compile, slow runtime)..."
  cargo build

  SOURCE_BINARY="../../target/debug/$BINARY_NAME"
  echo "🔍 Verifying that the binary ('$SOURCE_BINARY') was built successfully..."
  if [ -f "$SOURCE_BINARY" ]; then
    echo "📥 Copying debug binary to /usr/local/bin/ with sudo..."
    sudo cp "$SOURCE_BINARY" "/usr/local/bin/$BINARY_NAME"
  else
    echo "❌ Error: Build failed. The binary '$SOURCE_BINARY' was not found."
    exit 1
  fi
fi

# 4. Final success message
echo "✅ Success! The '$BINARY_NAME' command is now installed."
echo "Go build something 10x faster, manfromexistence! 🔥"
