#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/wav.sh"

fail() {
    echo "FAIL: $1"
    ((FAIL++))
}

pass() {
    echo "PASS: $1"
    ((PASS++))
}

test_header_size() {
    local tmpfile=$(mktemp)
    wav_header 1000 > "$tmpfile"
    local size=$(wc -c < "$tmpfile")
    rm -f "$tmpfile"
    if [[ $size -eq 44 ]]; then
        pass "header_size (44 bytes)"
    else
        fail "header_size: expected 44 bytes, got $size"
    fi
}

test_riff_marker() {
    local tmpfile=$(mktemp)
    wav_header 1000 > "$tmpfile"
    local marker=$(head -c 4 "$tmpfile")
    rm -f "$tmpfile"
    if [[ "$marker" == "RIFF" ]]; then
        pass "riff_marker"
    else
        fail "riff_marker: expected 'RIFF', got '$marker'"
    fi
}

test_wave_marker() {
    local tmpfile=$(mktemp)
    wav_header 1000 > "$tmpfile"
    local marker=$(dd if="$tmpfile" bs=1 skip=8 count=4 2>/dev/null)
    rm -f "$tmpfile"
    if [[ "$marker" == "WAVE" ]]; then
        pass "wave_marker"
    else
        fail "wave_marker: expected 'WAVE', got '$marker'"
    fi
}

test_fmt_marker() {
    local tmpfile=$(mktemp)
    wav_header 1000 > "$tmpfile"
    local marker=$(dd if="$tmpfile" bs=1 skip=12 count=4 2>/dev/null)
    rm -f "$tmpfile"
    if [[ "$marker" == "fmt " ]]; then
        pass "fmt_marker"
    else
        fail "fmt_marker: expected 'fmt ', got '$marker'"
    fi
}

test_data_marker() {
    local tmpfile=$(mktemp)
    wav_header 1000 > "$tmpfile"
    local marker=$(dd if="$tmpfile" bs=1 skip=36 count=4 2>/dev/null)
    rm -f "$tmpfile"
    if [[ "$marker" == "data" ]]; then
        pass "data_marker"
    else
        fail "data_marker: expected 'data', got '$marker'"
    fi
}

test_valid_wav_file() {
    local tmpfile=$(mktemp).wav
    wav_header 100 > "$tmpfile"
    # Add some sample data (100 samples * 2 bytes)
    for ((i=0; i<200; i++)); do
        printf '\x00' >> "$tmpfile"
    done
    local file_type=$(file "$tmpfile" 2>/dev/null | grep -i "audio\|RIFF\|wav")
    rm -f "$tmpfile"
    if [[ -n "$file_type" ]]; then
        pass "valid_wav_file"
    else
        fail "valid_wav_file: file command doesn't recognize as audio"
    fi
}

PASS=0
FAIL=0

test_header_size
test_riff_marker
test_wave_marker
test_fmt_marker
test_data_marker
test_valid_wav_file

echo "WAV tests: $PASS passed, $FAIL failed"
[[ $FAIL -eq 0 ]]
