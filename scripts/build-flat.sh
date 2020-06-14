#!/usr/bin/env bash

set -euo pipefail

WORK_DIR="$(pwd)"
SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)

# TODO un-hardcode
flatc=/home/sisso/workspace/others/flatbuffers/flatc

for file in $WORK_DIR/data/*; do
  echo "processing ${file}..."
  $flatc --rust -o "$WORK_DIR/rust/src/schemas" "${file}"
  $flatc --csharp -o "$WORK_DIR/ffi-csharp/schemas/" "${file}"
  $flatc --csharp -o "$WORK_DIR/unity3d/Assets/src/Schemas" "${file}"
done

echo "complete"
