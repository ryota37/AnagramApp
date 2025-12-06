#!/bin/bash

# Release build
echo "Building release..."
cargo build --release

# Create distribution directory
DIST_DIR="AnagramApp-release"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Copy files
echo "Copying files..."
cp target/release/AnagramApp.exe "$DIST_DIR/"
cp -r assets "$DIST_DIR/"

# Create ZIP
echo "Creating ZIP archive..."
ZIP_NAME="AnagramApp-windows-x64.zip"
rm -f "$ZIP_NAME"

if command -v zip &> /dev/null; then
    zip -r "$ZIP_NAME" "$DIST_DIR"
elif command -v 7z &> /dev/null; then
    7z a "$ZIP_NAME" "$DIST_DIR"
else
    echo "Warning: Neither 'zip' nor '7z' found. Please install one to create archive."
    echo "Files are ready in $DIST_DIR directory"
    exit 0
fi

echo "Done! Created $ZIP_NAME"
echo "Contents:"
echo "  AnagramApp.exe"
echo "  assets/"
echo "    japanese_dictionary.csv"
echo "    NotoSansJP-Regular.ttf"
