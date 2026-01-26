#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

TOTAL_PASS=0
TOTAL_FAIL=0

run_test() {
    local test_file="$1"
    local test_name=$(basename "$test_file" .sh)

    echo "=== Running $test_name ==="
    if "$test_file"; then
        echo ""
    else
        echo "^^^ FAILURES in $test_name ^^^"
        echo ""
    fi
}

for test_file in "$SCRIPT_DIR"/test_*.sh; do
    if [[ -f "$test_file" ]]; then
        chmod +x "$test_file"
        run_test "$test_file"
    fi
done

echo "==================================="
echo "All test suites completed"
