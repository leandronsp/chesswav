#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/notation.sh"

fail() {
    echo "FAIL: $1"
    ((FAIL++))
}

pass() {
    echo "PASS: $1"
    ((PASS++))
}

test_pawn_move() {
    notation_parse "e4"
    [[ "$NOTATION_PIECE" == "P" ]] || { fail "e4: piece should be P, got $NOTATION_PIECE"; return; }
    [[ "$NOTATION_DEST" == "e4" ]] || { fail "e4: dest should be e4, got $NOTATION_DEST"; return; }
    [[ "$NOTATION_CAPTURE" == "0" ]] || { fail "e4: capture should be 0"; return; }
    pass "pawn_move e4"
}

test_pawn_move_d5() {
    notation_parse "d5"
    [[ "$NOTATION_PIECE" == "P" ]] || { fail "d5: piece should be P"; return; }
    [[ "$NOTATION_DEST" == "d5" ]] || { fail "d5: dest should be d5"; return; }
    [[ "$NOTATION_CAPTURE" == "0" ]] || { fail "d5: capture should be 0"; return; }
    pass "pawn_move d5"
}

test_knight_move() {
    notation_parse "Nf3"
    [[ "$NOTATION_PIECE" == "N" ]] || { fail "Nf3: piece should be N, got $NOTATION_PIECE"; return; }
    [[ "$NOTATION_DEST" == "f3" ]] || { fail "Nf3: dest should be f3, got $NOTATION_DEST"; return; }
    [[ "$NOTATION_CAPTURE" == "0" ]] || { fail "Nf3: capture should be 0"; return; }
    pass "knight_move Nf3"
}

test_bishop_move() {
    notation_parse "Bb5"
    [[ "$NOTATION_PIECE" == "B" ]] || { fail "Bb5: piece should be B"; return; }
    [[ "$NOTATION_DEST" == "b5" ]] || { fail "Bb5: dest should be b5"; return; }
    pass "bishop_move Bb5"
}

test_queen_move() {
    notation_parse "Qh4"
    [[ "$NOTATION_PIECE" == "Q" ]] || { fail "Qh4: piece should be Q"; return; }
    [[ "$NOTATION_DEST" == "h4" ]] || { fail "Qh4: dest should be h4"; return; }
    pass "queen_move Qh4"
}

test_rook_move() {
    notation_parse "Ra1"
    [[ "$NOTATION_PIECE" == "R" ]] || { fail "Ra1: piece should be R"; return; }
    [[ "$NOTATION_DEST" == "a1" ]] || { fail "Ra1: dest should be a1"; return; }
    pass "rook_move Ra1"
}

test_king_move() {
    notation_parse "Ke2"
    [[ "$NOTATION_PIECE" == "K" ]] || { fail "Ke2: piece should be K"; return; }
    [[ "$NOTATION_DEST" == "e2" ]] || { fail "Ke2: dest should be e2"; return; }
    pass "king_move Ke2"
}

test_piece_capture() {
    notation_parse "Bxc6"
    [[ "$NOTATION_PIECE" == "B" ]] || { fail "Bxc6: piece should be B"; return; }
    [[ "$NOTATION_DEST" == "c6" ]] || { fail "Bxc6: dest should be c6"; return; }
    [[ "$NOTATION_CAPTURE" == "1" ]] || { fail "Bxc6: capture should be 1"; return; }
    pass "piece_capture Bxc6"
}

test_pawn_capture() {
    notation_parse "exd5"
    [[ "$NOTATION_PIECE" == "P" ]] || { fail "exd5: piece should be P"; return; }
    [[ "$NOTATION_DEST" == "d5" ]] || { fail "exd5: dest should be d5, got $NOTATION_DEST"; return; }
    [[ "$NOTATION_CAPTURE" == "1" ]] || { fail "exd5: capture should be 1"; return; }
    pass "pawn_capture exd5"
}

test_queen_capture() {
    notation_parse "Qxf7"
    [[ "$NOTATION_PIECE" == "Q" ]] || { fail "Qxf7: piece should be Q"; return; }
    [[ "$NOTATION_DEST" == "f7" ]] || { fail "Qxf7: dest should be f7"; return; }
    [[ "$NOTATION_CAPTURE" == "1" ]] || { fail "Qxf7: capture should be 1"; return; }
    pass "queen_capture Qxf7"
}

test_check_annotation() {
    notation_parse "Qh5+"
    [[ "$NOTATION_PIECE" == "Q" ]] || { fail "Qh5+: piece should be Q"; return; }
    [[ "$NOTATION_DEST" == "h5" ]] || { fail "Qh5+: dest should be h5"; return; }
    pass "check_annotation Qh5+"
}

test_checkmate_annotation() {
    notation_parse "Qxf7#"
    [[ "$NOTATION_PIECE" == "Q" ]] || { fail "Qxf7#: piece should be Q"; return; }
    [[ "$NOTATION_DEST" == "f7" ]] || { fail "Qxf7#: dest should be f7"; return; }
    [[ "$NOTATION_CAPTURE" == "1" ]] || { fail "Qxf7#: capture should be 1"; return; }
    pass "checkmate_annotation Qxf7#"
}

PASS=0
FAIL=0

test_pawn_move
test_pawn_move_d5
test_knight_move
test_bishop_move
test_queen_move
test_rook_move
test_king_move
test_piece_capture
test_pawn_capture
test_queen_capture
test_check_annotation
test_checkmate_annotation

echo "Notation tests: $PASS passed, $FAIL failed"
[[ $FAIL -eq 0 ]]
