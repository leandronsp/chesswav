#!/usr/bin/env bash

NOTATION_PIECE=""
NOTATION_DEST=""
NOTATION_CAPTURE=0

notation_parse() {
    local move="$1"

    # Reset
    NOTATION_PIECE=""
    NOTATION_DEST=""
    NOTATION_CAPTURE=0

    # Remove annotations (+, #, !, ?)
    move="${move//+/}"
    move="${move//#/}"
    move="${move//!/}"
    move="${move//\?/}"

    # Check for capture
    if [[ "$move" == *"x"* ]]; then
        NOTATION_CAPTURE=1
        move="${move//x/}"
    fi

    # Determine piece and destination
    local first_char="${move:0:1}"

    case "$first_char" in
        K|Q|R|B|N)
            NOTATION_PIECE="$first_char"
            # Destination is last two characters
            NOTATION_DEST="${move: -2}"
            ;;
        *)
            # Pawn move
            NOTATION_PIECE="P"
            # Destination is last two characters
            NOTATION_DEST="${move: -2}"
            ;;
    esac

    return 0
}
