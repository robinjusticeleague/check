#!/bin/bash

# This script automates the global search and replace for rebranding.
# It targets different casings and the NPM scope, ignoring build artifacts and dependencies.

echo "Starting global search and replace..."
echo "This might take a few moments depending on the project size."

# Note: These commands modify files in place. Make sure you have a backup or are on a git branch.

# 1. Replace snake_case: check_ -> check_
echo "Replacing 'check_' with 'check_'..."
find . -type f -not -path "./.git/*" -not -path "./target/*" -not -path "*/node_modules/*" -exec sed -i 's/check_/check_/g' {} +

# 2. Replace lowercase: check -> check
echo "Replacing 'check' with 'check'..."
find . -type f -not -path "./.git/*" -not -path "./target/*" -not -path "*/node_modules/*" -exec sed -i 's/check/check/g' {} +

# 3. Replace PascalCase: Check -> Check
echo "Replacing 'Check' with 'Check'..."
find . -type f -not -path "./.git/*" -not -path "./target/*" -not -path "*/node_modules/*" -exec sed -i 's/Check/Check/g' {} +

# 4. Replace SCREAMING_SNAKE_CASE: CHECK -> CHECK
echo "Replacing 'CHECK' with 'CHECK'..."
find . -type f -not -path "./.git/*" -not -path "./target/*" -not -path "*/node_modules/*" -exec sed -i 's/CHECK/CHECK/g' {} +

# 5. Replace the NPM scope: @checkjs -> @check
echo "Replacing '@checkjs' with '@check'..."
find . -type f -not -path "./.git/*" -not -path "./target/*" -not -path "*/node_modules/*" -exec sed -i 's/@checkjs/@check/g' {} +

echo "Global search and replace complete."
echo "Please review the changes and manually fix any unintended replacements."

