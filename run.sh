#! /bin/bash

set -euo pipefail

SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)

export LD_LIBRARY_PATH="$SCRIPT_DIR/Assets/Plugins/lib"
echo "register LD_LIBRARY_PATH=${LD_LIBRARY_PATH}" 
ls $LD_LIBRARY_PATH

[[ $(rm -v /tmp/ffi.log) ]] || true

${SCRIPT_DIR}/rust/scripts/release-to-unity3d.sh

unity-editor
