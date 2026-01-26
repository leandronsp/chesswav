#!/usr/bin/env bash

# Board represented as 64-element array
# Index = (rank-1)*8 + col_index where col_index: a=0, b=1, ... h=7
declare -a BOARD

_col_to_index() {
    case "$1" in
        a) echo 0 ;; b) echo 1 ;; c) echo 2 ;; d) echo 3 ;;
        e) echo 4 ;; f) echo 5 ;; g) echo 6 ;; h) echo 7 ;;
    esac
}

_square_to_index() {
    local square="$1"
    local col="${square:0:1}"
    local rank="${square:1:1}"
    local col_idx=$(_col_to_index "$col")
    echo $(( (rank - 1) * 8 + col_idx ))
}

board_init() {
    BOARD=()

    # Initialize 64 empty squares
    for ((i=0; i<64; i++)); do
        BOARD[$i]=""
    done

    # White pieces (rank 1)
    BOARD[$(_square_to_index "a1")]="R"
    BOARD[$(_square_to_index "b1")]="N"
    BOARD[$(_square_to_index "c1")]="B"
    BOARD[$(_square_to_index "d1")]="Q"
    BOARD[$(_square_to_index "e1")]="K"
    BOARD[$(_square_to_index "f1")]="B"
    BOARD[$(_square_to_index "g1")]="N"
    BOARD[$(_square_to_index "h1")]="R"

    # White pawns (rank 2)
    for col in a b c d e f g h; do
        BOARD[$(_square_to_index "${col}2")]="P"
    done

    # Black pieces (rank 8)
    BOARD[$(_square_to_index "a8")]="r"
    BOARD[$(_square_to_index "b8")]="n"
    BOARD[$(_square_to_index "c8")]="b"
    BOARD[$(_square_to_index "d8")]="q"
    BOARD[$(_square_to_index "e8")]="k"
    BOARD[$(_square_to_index "f8")]="b"
    BOARD[$(_square_to_index "g8")]="n"
    BOARD[$(_square_to_index "h8")]="r"

    # Black pawns (rank 7)
    for col in a b c d e f g h; do
        BOARD[$(_square_to_index "${col}7")]="p"
    done
}

board_get() {
    local square="$1"
    local idx=$(_square_to_index "$square")
    echo "${BOARD[$idx]}"
}

board_set() {
    local square="$1"
    local piece="$2"
    local idx=$(_square_to_index "$square")
    BOARD[$idx]="$piece"
}

board_print() {
    echo "  a b c d e f g h" >&2
    for rank in 8 7 6 5 4 3 2 1; do
        printf "%d " "$rank" >&2
        for col in a b c d e f g h; do
            local piece=$(board_get "${col}${rank}")
            if [[ -z "$piece" ]]; then
                printf ". " >&2
            else
                printf "%s " "$piece" >&2
            fi
        done
        echo >&2
    done
}
