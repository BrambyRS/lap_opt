#!/bin/bash

# Get the directory of the script to make paths absolute
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="$SCRIPT_DIR/build"

mkdir -p "$BUILD_DIR"

pdflatex -output-directory="$BUILD_DIR" "$SCRIPT_DIR/src/main.tex"
