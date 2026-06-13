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
echo "=== Running tests ==="
cargo test 2>&1 || EXIT_CODE=$?

exit $EXIT_CODE
