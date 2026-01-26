#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/synth.sh"

fail() {
    echo "FAIL: $1"
    ((FAIL++))
}

pass() {
    echo "PASS: $1"
    ((PASS++))
}

test_sample_count() {
    # 440 Hz, 100ms at 44100 Hz = 4410 samples = 8820 bytes
    local tmpfile=$(mktemp)
    synth_sine 440 100 > "$tmpfile"
    local byte_count=$(wc -c < "$tmpfile")
    rm -f "$tmpfile"
    local expected_bytes=8820
    local diff=$(( byte_count - expected_bytes ))
    [[ $diff -lt 0 ]] && diff=$(( -diff ))
    if [[ $diff -le 10 ]]; then
        pass "sample_count (100ms = ~4410 samples)"
    else
        fail "sample_count: expected ~$expected_bytes bytes, got $byte_count"
    fi
}

test_sample_count_300ms() {
    # 440 Hz, 300ms at 44100 Hz = 13230 samples = 26460 bytes
    local tmpfile=$(mktemp)
    synth_sine 440 300 > "$tmpfile"
    local byte_count=$(wc -c < "$tmpfile")
    rm -f "$tmpfile"
    local expected_bytes=26460
    local diff=$(( byte_count - expected_bytes ))
    [[ $diff -lt 0 ]] && diff=$(( -diff ))
    if [[ $diff -le 10 ]]; then
        pass "sample_count_300ms (~13230 samples)"
    else
        fail "sample_count_300ms: expected ~$expected_bytes bytes, got $byte_count"
    fi
}

test_output_binary() {
    local tmpfile=$(mktemp)
    synth_sine 440 10 > "$tmpfile"
    local byte_count=$(wc -c < "$tmpfile")
    rm -f "$tmpfile"
    if [[ $byte_count -gt 0 ]]; then
        pass "output_binary (non-empty)"
    else
        fail "output_binary: output is empty"
    fi
}

test_different_frequencies() {
    local tmp1=$(mktemp)
    local tmp2=$(mktemp)
    synth_sine 440 50 > "$tmp1"
    synth_sine 880 50 > "$tmp2"
    local hash1=$(xxd -p "$tmp1" | head -c 100)
    local hash2=$(xxd -p "$tmp2" | head -c 100)
    rm -f "$tmp1" "$tmp2"
    if [[ "$hash1" != "$hash2" ]]; then
        pass "different_frequencies"
    else
        fail "different_frequencies: 440 Hz and 880 Hz produce same output"
    fi
}

PASS=0
FAIL=0

test_sample_count
test_sample_count_300ms
test_output_binary
test_different_frequencies

echo "Synth tests: $PASS passed, $FAIL failed"
[[ $FAIL -eq 0 ]]
