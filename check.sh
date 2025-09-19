#!/bin/bash

# A combined script to fully set up the 'check' project.
# It performs the following actions in order:
# 1. Renames all files and directories from 'biome' to 'check'.
# 2. Replaces all instances of 'biome' with 'check' within the files.
# 3. Builds and installs the 'check' CLI.
#
# Usage:
#   ./check.sh          (for a fast, unoptimized debug build)
#   ./check.sh release  (for a slow, optimized release build)

set -e

echo "🚀 Starting the full 'check' setup process..."
echo "This will rename, replace, and install."
echo "----------------------------------------------------"

#==============================================================================
# STEP 1: RENAME FILES AND DIRECTORIES
#==============================================================================
echo "STEP 1: Renaming files and directories from 'biome' to 'check'..."

CRATES_DIR="crates"

if [ ! -d "$CRATES_DIR" ]; then
  echo "❌ Error: Directory '$CRATES_DIR' not found."
  echo "Please make sure you are running this script from the root of your project."
  exit 1
fi

find "$CRATES_DIR" -depth -name "*biome*" | while read -r old_path; do
  if [ -e "$old_path" ]; then
    dir_name=$(dirname "$old_path")
    old_basename=$(basename "$old_path")
    
    new_basename=$(echo "$old_basename" | sed 's/biome/check/g')
    
    new_path="$dir_name/$new_basename"
    
    if [ "$old_path" != "$new_path" ]; then
      echo "   Renaming '$old_path' -> '$new_path'"
      mv "$old_path" "$new_path"
    fi
  fi
done

echo "✅ File and directory renaming complete."
echo "----------------------------------------------------"

#==============================================================================
# STEP 2: REPLACE CODE CONTENT
#==============================================================================
echo "STEP 2: Replacing code content from 'biome' to 'check'..."

FIND_CMD="find . -type f -not -path './.git/*' -not -path './check.sh' -not -path './target/*' -not -path '*/node_modules/*'"

echo "   Replacing 'biome_' with 'check_'..."
eval "$FIND_CMD" -exec sed -i 's/biome_/check_/g' {} +

echo "   Replacing 'biome' with 'check'..."
eval "$FIND_CMD" -exec sed -i 's/biome/check/g' {} +

echo "   Replacing 'Biome' with 'Check'..."
eval "$FIND_CMD" -exec sed -i 's/Biome/Check/g' {} +

echo "   Replacing 'BIOME' with 'CHECK'..."
eval "$FIND_CMD" -exec sed -i 's/BIOME/CHECK/g' {} +

echo "   Replacing '@biomejs' with '@check'..."
eval "$FIND_CMD" -exec sed -i 's/@biomejs/@check/g' {} +

echo "✅ Code content replacement complete."
echo "----------------------------------------------------"

#==============================================================================
# STEP 3: BUILD AND INSTALL THE CLI
#==============================================================================
echo "STEP 3: Building and installing the 'check' CLI..."

if command -v lld &> /dev/null; then
    echo "⚡ Found 'lld' linker. Configuring for faster builds."
    export RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=lld"
else
    echo "💡 Tip: Install 'lld' for faster Rust build times (e.g., 'sudo apt-get install lld')."
fi

PROJECT_DIR="crates/check_cli"
BINARY_NAME="check"
BUILD_MODE="debug"

if [[ "$1" == "release" ]]; then
  BUILD_MODE="release"
fi

echo "🚀 Starting installation for '$BINARY_NAME' (Mode: $BUILD_MODE)..."

if [ ! -d "$PROJECT_DIR" ]; then
  echo "❌ Error: Project directory not found at './${PROJECT_DIR}'"
  echo "This might have failed due to an error in the renaming step."
  exit 1
fi

echo "➡️  Changing directory to '$PROJECT_DIR'..."
cd "$PROJECT_DIR"

if [[ "$BUILD_MODE" == "release" ]]; then
  echo "🛠️  Building in release mode (slower compile, fast runtime)..."
  cargo build --release
  SOURCE_BINARY="../../target/release/$BINARY_NAME"
else
  echo "🛠️  Building in debug mode (faster compile, slow runtime)..."
  cargo build
  SOURCE_BINARY="../../target/debug/$BINARY_NAME"
fi

echo "🔍 Verifying that the binary ('$SOURCE_BINARY') was built successfully..."
if [ -f "$SOURCE_BINARY" ]; then
  echo "📥 Copying binary to /usr/local/bin/ with sudo..."
  sudo cp "$SOURCE_BINARY" "/usr/local/bin/$BINARY_NAME"
else
  echo "❌ Error: Build failed. The binary '$SOURCE_BINARY' was not found."
  exit 1
fi

cd ../..

echo "----------------------------------------------------"
echo "🎉 All steps completed successfully!"
echo "The '$BINARY_NAME' command is now installed."
echo "Go build something 10x faster, manfromexistence! 🔥"
echo "Please review the git changes and manually fix any unintended replacements."