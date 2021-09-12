#!/usr/bin/env bash

set -euo pipefail

WORK_DIR="$(pwd)"

RUST_OPTS=""
RUST_TARGET="debug"

if [[ "${1:-}" == "--release" ]]
then
  echo "building release mode"
  RUST_OPTS="${RUST_OPTS} --release"
  RUST_TARGET="release"
fi

(
  cd "$WORK_DIR/rust"
  cargo build $RUST_OPTS
)

target="$WORK_DIR/unity3d/Assets/Plugins"

echo "copy ff1 library"
cp -v "$WORK_DIR/rust/target/${RUST_TARGET}/libffi_domain.so" "$target"

echo "copy ffi-flap library"
cp -v "$WORK_DIR/rust/target/${RUST_TARGET}/libffi_domain_2.so" "$target/libffi_domain_2_native.so"

echo "copy ffi-flat generated code"
target="$WORK_DIR/unity3d/Assets/src/Ffi2"
cp -r "rust/ffi-flap/generated_dotnet/ffi_domain_2.cs" "$target"