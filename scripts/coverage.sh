#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if ! cargo llvm-cov --version >/dev/null 2>&1; then
  echo "error: cargo-llvm-cov is not installed" >&2
  echo "install: cargo install cargo-llvm-cov --locked" >&2
  echo "also need: rustup component add llvm-tools-preview" >&2
  exit 1
fi

OUT_DIR="${ODM_COVERAGE_DIR:-target/coverage}"
mkdir -p "$OUT_DIR"

echo "Running workspace coverage → ${OUT_DIR}/ ..."
cargo llvm-cov --workspace --html --output-dir "$OUT_DIR"
cargo llvm-cov report --lcov --output-path "$OUT_DIR/lcov.info"

echo "HTML: ${OUT_DIR}/index.html"
echo "LCOV: ${OUT_DIR}/lcov.info"
