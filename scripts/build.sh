#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$REPO_ROOT"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is required but not found. Install Rust: https://rustup.rs" >&2
  exit 1
fi

cargo build --release
