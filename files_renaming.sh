#!/bin/bash

CRATES_DIR="crates"

if [ ! -d "$CRATES_DIR" ]; then
  echo "Error: Directory '$CRATES_DIR' not found."
  echo "Please make sure you are running this script from the root of your project."
  exit 1
fi

echo "Starting the recursive renaming process for directories and files..."

# Use find with -depth to process directory contents before the directory itself.
# This finds any file or directory containing "biome" in its name.
find "$CRATES_DIR" -depth -name "*biome*" | while read -r old_path; do
  # Check if the path still exists, as it might have been part of a parent that was already renamed.
  if [ -e "$old_path" ]; then
    dir_name=$(dirname "$old_path")
    old_basename=$(basename "$old_path")
    
    # Create the new name by replacing all occurrences of "biome" with "check"
    new_basename=$(echo "$old_basename" | sed 's/biome/check/g')
    
    # Construct the full new path
    new_path="$dir_name/$new_basename"
    
    # Only rename if the name has actually changed to avoid unnecessary moves
    if [ "$old_path" != "$new_path" ]; then
      # Perform the rename
      echo "Renaming '$old_path' to '$new_path'"
      mv "$old_path" "$new_path"
    fi
  fi
done

echo "Renaming process complete."

