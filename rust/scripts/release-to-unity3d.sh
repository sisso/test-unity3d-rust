#!/usr/bin/env bash

set -euo pipefail

(
  cd rust
  cargo build --release
)

source=rust/target/release/librustlib.so

cp -v $source ./Assets/Plugins/lib
cp -v $source ./build/01_Data/Plugins/librustlib.so
