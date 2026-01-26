#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CHESSWAV="$SCRIPT_DIR/../chesswav"

fail() {
    echo "FAIL: $1"
    ((FAIL++))
}

pass() {
    echo "PASS: $1"
    ((PASS++))
}

test_basic_pipeline() {
    local tmpfile=$(mktemp).wav
    echo "e4 e5 Nf3 Nc6" | "$CHESSWAV" > "$tmpfile" 2>/dev/null
    local size=$(wc -c < "$tmpfile")
    rm -f "$tmpfile"
    # Should have header (44 bytes) + sample data
    if [[ $size -gt 44 ]]; then
        pass "basic_pipeline"
    else
        fail "basic_pipeline: output size is $size bytes (expected > 44)"
    fi
}

test_wav_valid() {
    local tmpfile=$(mktemp).wav
    echo "e4 e5" | "$CHESSWAV" > "$tmpfile" 2>/dev/null
    local file_type=$(file "$tmpfile" 2>/dev/null)
    rm -f "$tmpfile"
    if echo "$file_type" | grep -qi "audio\|RIFF\|wav"; then
        pass "wav_valid"
    else
        fail "wav_valid: file type is '$file_type'"
    fi
}

test_four_moves() {
    local tmpfile=$(mktemp).wav
    echo "e4 e5 Nf3 Nc6" | "$CHESSWAV" > "$tmpfile" 2>/dev/null
    local size=$(wc -c < "$tmpfile")
    rm -f "$tmpfile"
    # 4 moves * 300ms + 4 silences * 50ms = 1400ms
    # At 44100 Hz, 16-bit mono = 1400ms * 44100 * 2 = ~123,480 bytes + 44 header
    local min_expected=100000
    if [[ $size -gt $min_expected ]]; then
        pass "four_moves (size: $size bytes)"
    else
        fail "four_moves: expected > $min_expected bytes, got $size"
    fi
}

test_empty_input() {
    local tmpfile=$(mktemp).wav
    echo "" | "$CHESSWAV" > "$tmpfile" 2>/dev/null
    local size=$(wc -c < "$tmpfile")
    rm -f "$tmpfile"
    # Should at least have valid header
    if [[ $size -ge 44 ]]; then
        pass "empty_input"
    else
        fail "empty_input: expected >= 44 bytes, got $size"
    fi
}

test_single_move() {
    local tmpfile=$(mktemp).wav
    echo "e4" | "$CHESSWAV" > "$tmpfile" 2>/dev/null
    local size=$(wc -c < "$tmpfile")
    rm -f "$tmpfile"
    # 1 move = 300ms at 44100 Hz * 2 bytes = 26,460 bytes + header + silence
    if [[ $size -gt 20000 ]]; then
        pass "single_move"
    else
        fail "single_move: expected > 20000 bytes, got $size"
    fi
}

test_capture_move() {
    local tmpfile=$(mktemp).wav
    echo "Bxc6" | "$CHESSWAV" > "$tmpfile" 2>/dev/null
    local size=$(wc -c < "$tmpfile")
    rm -f "$tmpfile"
    if [[ $size -gt 20000 ]]; then
        pass "capture_move"
    else
        fail "capture_move: expected > 20000 bytes, got $size"
    fi
}

PASS=0
FAIL=0

chmod +x "$CHESSWAV"

test_basic_pipeline
test_wav_valid
test_four_moves
test_empty_input
test_single_move
test_capture_move

echo "Integration tests: $PASS passed, $FAIL failed"
[[ $FAIL -eq 0 ]]
