#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/freq.sh"

fail() {
    echo "FAIL: $1"
    ((FAIL++))
}

pass() {
    echo "PASS: $1"
    ((PASS++))
}

# Test frequency is within acceptable range (within 5%)
assert_freq_range() {
    local actual="$1"
    local expected="$2"
    local name="$3"
    local tolerance=$(( expected / 20 ))  # 5%
    local diff=$(( actual - expected ))
    [[ $diff -lt 0 ]] && diff=$(( -diff ))
    if [[ $diff -le $tolerance ]]; then
        pass "$name"
    else
        fail "$name: expected ~$expected Hz, got $actual Hz"
    fi
}

test_a4_c() {
    local freq=$(freq_from_square "a4")
    # a4 = C4 = 262 Hz
    assert_freq_range "$freq" 262 "a4 -> C4 (262 Hz)"
}

test_f4_a() {
    local freq=$(freq_from_square "f4")
    # f4 = A4 = 440 Hz
    assert_freq_range "$freq" 440 "f4 -> A4 (440 Hz)"
}

test_e4_g() {
    local freq=$(freq_from_square "e4")
    # e4 = G4 = 392 Hz
    assert_freq_range "$freq" 392 "e4 -> G4 (392 Hz)"
}

test_b4_d() {
    local freq=$(freq_from_square "b4")
    # b4 = D4 = 294 Hz
    assert_freq_range "$freq" 294 "b4 -> D4 (294 Hz)"
}

test_octave_up() {
    local freq=$(freq_from_square "a5")
    # a5 = C5 = 523 Hz
    assert_freq_range "$freq" 523 "a5 -> C5 (523 Hz)"
}

test_octave_down() {
    local freq=$(freq_from_square "a3")
    # a3 = C3 = 131 Hz
    assert_freq_range "$freq" 131 "a3 -> C3 (131 Hz)"
}

test_h4_c_high() {
    local freq=$(freq_from_square "h4")
    # h4 = C5 = 523 Hz
    assert_freq_range "$freq" 523 "h4 -> C5 (523 Hz)"
}

test_g4_b() {
    local freq=$(freq_from_square "g4")
    # g4 = B4 = 494 Hz
    assert_freq_range "$freq" 494 "g4 -> B4 (494 Hz)"
}

PASS=0
FAIL=0

test_a4_c
test_f4_a
test_e4_g
test_b4_d
test_octave_up
test_octave_down
test_h4_c_high
test_g4_b

echo "Freq tests: $PASS passed, $FAIL failed"
[[ $FAIL -eq 0 ]]
