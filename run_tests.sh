#!/usr/bin/env bash
# Runs tests.
#
# Usage: ./run_tests.sh
set -euo pipefail

git config core.hooksPath .githooks 2>/dev/null || true

echo "Compiling tests..."
cargo test --no-run 2>&1

echo ""
EXIT_CODE=0

if [ -f src/lib.rs ]; then
  echo "=== Unit tests (lib) ==="
  cargo test --lib 2>&1 || EXIT_CODE=$?
fi

echo "=== All tests ==="
cargo test 2>&1 || EXIT_CODE=$?

exit $EXIT_CODE
