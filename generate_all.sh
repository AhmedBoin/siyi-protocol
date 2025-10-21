#!/bin/bash

# === Configuration ===
XML_FILE="siyi_protocol.xml"
OUTPUT_DIR="src"
SCRIPT="generator.py"

PROTOCOLS=("TCP" "UDP" "TTL")
CAMERAS=("ZT30" "ZT6" "ZR30" "ZR10" "A8mini" "A2mini")

echo "Starting code generation..."
mkdir -p "$OUTPUT_DIR"

for protocol in "${PROTOCOLS[@]}"; do
  for camera in "${CAMERAS[@]}"; do
    lower_camera=$(echo "$camera" | tr '[:upper:]' '[:lower:]')
    lower_protocol=$(echo "$protocol" | tr '[:upper:]' '[:lower:]')

    output_file="${OUTPUT_DIR}/${lower_camera}_${lower_protocol}.rs"
    echo "Generating for protocol '${protocol}' and camera '${camera}' -> ${output_file}"

    python3 "$SCRIPT" "$XML_FILE" --protocol "$protocol" --camera "$camera" -o "$output_file"

    if [ $? -eq 0 ]; then
      echo "Successfully generated ${output_file}"
    else
      echo "Failed for ${protocol} / ${camera}"
    fi
  done
done

echo "✨ All generation complete! Running cargo fmt..."

# Format generated Rust code
if command -v cargo >/dev/null 2>&1; then
  cargo fmt --all
  if [ $? -eq 0 ]; then
    echo "Successfully formatted all Rust code."
  else
    echo "cargo fmt failed — check for formatting issues."
  fi
else
  echo "cargo not found. Skipping formatting."
fi

echo "Done!"