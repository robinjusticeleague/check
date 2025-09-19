#!/bin/bash

echo "Starting global search and replace..."
echo "This might take a few moments depending on the project size."

# 1. Replace snake_case: biome_ -> check_
echo "Replacing 'biome_' with 'check_'..."
find . -type f -not -path "./.git/*" -not -path "./code_renaming.sh" -not -path "./files_renaming.sh*" -not -path "./target/*" -not -path "*/node_modules/*" -exec sed -i 's/biome_/check_/g' {} +

# 2. Replace lowercase: biome -> check
echo "Replacing 'biome' with 'check'..."
find . -type f -not -path "./.git/*" -not -path "./code_renaming.sh" -not -path "./files_renaming.sh*" -not -path "./target/*" -not -path "*/node_modules/*" -exec sed -i 's/biome/check/g' {} +

# 3. Replace PascalCase: Biome -> Check
echo "Replacing 'Biome' with 'Check'..."
find . -type f -not -path "./.git/*" -not -path "./code_renaming.sh" -not -path "./files_renaming.sh*" -not -path "./target/*" -not -path "*/node_modules/*" -exec sed -i 's/Biome/Check/g' {} +

# 4. Replace SCREAMING_SNAKE_CASE: BIOME -> CHECK
echo "Replacing 'BIOME' with 'CHECK'..."
find . -type f -not -path "./.git/*" -not -path "./code_renaming.sh" -not -path "./files_renaming.sh*" -not -path "./target/*" -not -path "*/node_modules/*" -exec sed -i 's/BIOME/CHECK/g' {} +

# 5. Replace the NPM scope: @biomejs -> @check
echo "Replacing '@biomejs' with '@check'..."
find . -type f -not -path "./.git/*" -not -path "./code_renaming.sh" -not -path "./files_renaming.sh*" -not -path "./target/*" -not -path "*/node_modules/*" -exec sed -i 's/@biomejs/@check/g' {} +

echo "Global search and replace complete."
echo "Please review the changes and manually fix any unintended replacements."
