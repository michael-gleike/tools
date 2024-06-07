#!/bin/bash
blue=$'\e[38;2;12;158;144m'
end=$'\e[0m'

# Define source and destination directories
BOOK_DIR="./aquascope-book"
SRC_DIR="./src"
DEST_DIR="$BOOK_DIR/src"
SUMMARY_FILE="$DEST_DIR/SUMMARY.md"

rm -f $DEST_DIR/*md

# Check if destination directory exists
if [ ! -d "$DEST_DIR" ]; then
  echo "Destination directory '$DEST_DIR' does not exist. Creating it."
  mkdir -p "$DEST_DIR"
fi

# Create or clear SUMMARY.md file
echo "# Summary\n\n" > "$SUMMARY_FILE"  # Clear or create SUMMARY.md

# Find .md files and store them in the array
find "$SRC_DIR" -name "*.md" -print | while IFS= read -r file; do
  filename=$(basename "$file")
  base_filename="${filename%.*}"
  extension="${filename##*.}"
  dest_file="$DEST_DIR/$filename"
  suffix_count=1

  # Check if file already exists
  while [[ -f "$dest_file" ]]; do
    dest_file="$DEST_DIR/$base_filename-$suffix_count.$extension"
    suffix_count=$((suffix_count + 1))
  done
  cp "$file" "$dest_file"
  echo "$file will be shown as ${blue}$(basename "$dest_file")${end}"

  # Write to SUMMARY.md
  filename=$(basename "$dest_file")
  echo "  - [$filename](./$filename)" >> "$SUMMARY_FILE"
done

cd $BOOK_DIR
echo Serving on: http://localhost:3000/
mdbook serve -n 0.0.0.0 > output.log 2>&1

tail -f /dev/null