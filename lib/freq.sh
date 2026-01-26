#!/usr/bin/env bash

# Base frequencies for octave 4 (C4 through C5)
# Columns a-h map to notes C, D, E, F, G, A, B, C
declare -a FREQ_BASE
FREQ_BASE[0]=262   # a -> C
FREQ_BASE[1]=294   # b -> D
FREQ_BASE[2]=330   # c -> E
FREQ_BASE[3]=349   # d -> F
FREQ_BASE[4]=392   # e -> G
FREQ_BASE[5]=440   # f -> A
FREQ_BASE[6]=494   # g -> B
FREQ_BASE[7]=523   # h -> C (octave up)

_col_to_freq_index() {
    case "$1" in
        a) echo 0 ;; b) echo 1 ;; c) echo 2 ;; d) echo 3 ;;
        e) echo 4 ;; f) echo 5 ;; g) echo 6 ;; h) echo 7 ;;
    esac
}

freq_from_square() {
    local square="$1"
    local col="${square:0:1}"
    local rank="${square:1:1}"

    local col_idx=$(_col_to_freq_index "$col")
    local base_freq="${FREQ_BASE[$col_idx]}"

    # Rank 4 is base octave
    # Each rank up doubles frequency, each rank down halves it
    local octave_diff=$(( rank - 4 ))

    local freq="$base_freq"
    if [[ $octave_diff -gt 0 ]]; then
        # Shift up by octave_diff octaves
        for ((i=0; i<octave_diff; i++)); do
            freq=$(( freq * 2 ))
        done
    elif [[ $octave_diff -lt 0 ]]; then
        # Shift down by abs(octave_diff) octaves
        local abs_diff=$(( -octave_diff ))
        for ((i=0; i<abs_diff; i++)); do
            freq=$(( freq / 2 ))
        done
    fi

    echo "$freq"
}
