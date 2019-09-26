#!/usr/bin/env bash

set -euo pipefail

WORK_DIR="$(pwd)"

(
  cd "$WORK_DIR/rust"
  cargo build --release
  cargo build
)

source="$WORK_DIR/rust/target/release/librustlib.so"

cp -v $source "$WORK_DIR/unity3d/Assets/Plugins/lib"
cp -v $source "$WORK_DIR/build/01_Data/Plugins/librustlib.so"
