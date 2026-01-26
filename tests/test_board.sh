#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/board.sh"

fail() {
    echo "FAIL: $1"
    ((FAIL++))
}

pass() {
    echo "PASS: $1"
    ((PASS++))
}

test_board_init() {
    board_init
    [[ $(board_get "e1") == "K" ]] || { fail "King should be on e1"; return; }
    [[ $(board_get "d1") == "Q" ]] || { fail "Queen should be on d1"; return; }
    [[ $(board_get "a1") == "R" ]] || { fail "Rook should be on a1"; return; }
    [[ $(board_get "b1") == "N" ]] || { fail "Knight should be on b1"; return; }
    [[ $(board_get "c1") == "B" ]] || { fail "Bishop should be on c1"; return; }
    [[ $(board_get "e2") == "P" ]] || { fail "Pawn should be on e2"; return; }
    pass "board_init"
}

test_board_init_black() {
    board_init
    [[ $(board_get "e8") == "k" ]] || { fail "Black king should be on e8"; return; }
    [[ $(board_get "d8") == "q" ]] || { fail "Black queen should be on d8"; return; }
    [[ $(board_get "a8") == "r" ]] || { fail "Black rook should be on a8"; return; }
    [[ $(board_get "e7") == "p" ]] || { fail "Black pawn should be on e7"; return; }
    pass "board_init_black"
}

test_board_get_empty() {
    board_init
    [[ $(board_get "e4") == "" ]] || { fail "e4 should be empty"; return; }
    [[ $(board_get "d5") == "" ]] || { fail "d5 should be empty"; return; }
    pass "board_get_empty"
}

test_board_set() {
    board_init
    board_set "e4" "P"
    [[ $(board_get "e4") == "P" ]] || { fail "Pawn should be on e4 after set"; return; }
    pass "board_set"
}

test_board_set_clear() {
    board_init
    board_set "e2" ""
    [[ $(board_get "e2") == "" ]] || { fail "e2 should be empty after clearing"; return; }
    pass "board_set_clear"
}

PASS=0
FAIL=0

test_board_init
test_board_init_black
test_board_get_empty
test_board_set
test_board_set_clear

echo "Board tests: $PASS passed, $FAIL failed"
[[ $FAIL -eq 0 ]]
