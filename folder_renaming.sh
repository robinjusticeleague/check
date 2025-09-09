#!/bin/bash

# This script automates the renaming of directories in the 'crates' directory
# from the 'check_' prefix to the 'check_' prefix.

CRATES_DIR="crates"

# Check if the 'crates' directory exists
if [ ! -d "$CRATES_DIR" ]; then
  echo "Error: Directory '$CRATES_DIR' not found."
  echo "Please make sure you are running this script from the root of your project."
  exit 1
fi

# Navigate into the crates directory
cd "$CRATES_DIR" || exit

echo "Starting the renaming process for crates..."

# Loop through all directories starting with 'check_'
for old_name in check_*; do
  # Check if it is actually a directory to avoid errors
  if [ -d "$old_name" ]; then
    # Create the new name by replacing 'check_' with 'check_'
    new_name=$(echo "$old_name" | sed 's/^check_/check_/')
    
    # Rename the directory and print the action to the console
    echo "Renaming '$old_name' to '$new_name'"
    mv "$old_name" "$new_name"
  fi
done

echo "Crate renaming process complete."
