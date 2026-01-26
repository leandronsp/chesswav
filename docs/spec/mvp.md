# ChessWAV MVP - Technical Specification

## Overview

This document specifies the technical implementation of ChessWAV MVP. The system converts algebraic chess notation to WAV audio using pure bash.

## Architecture

```
STDIN → notation_parse → freq_from_square → synth_sine → wav_output → STDOUT
              ↓
          board_update
```

## Module Specifications

### 1. lib/board.sh - Board Representation

#### Data Structure
```bash
declare -A BOARD  # Associative array: BOARD["e4"]="P"
```

#### Piece Encoding
| Character | Piece |
|-----------|-------|
| K | White King |
| Q | White Queen |
| R | White Rook |
| B | White Bishop |
| N | White Knight |
| P | White Pawn |
| k | Black King |
| q | Black Queen |
| r | Black Rook |
| b | Black Bishop |
| n | Black Knight |
| p | Black Pawn |
| (empty) | Empty square |

#### Functions

```bash
board_init()
# Initializes BOARD with standard chess starting position

board_get(square)
# Returns: piece at square or empty string
# Example: board_get "e1" → "K"

board_set(square, piece)
# Sets piece at square
# Example: board_set "e4" "P"

board_print()
# Debug: prints ASCII board to stderr
```

### 2. lib/notation.sh - Move Parser

#### Output Variables
```bash
NOTATION_PIECE=""       # K, Q, R, B, N, P
NOTATION_DEST=""        # e4, d5, etc.
NOTATION_CAPTURE=0      # 1 if capture
```

#### Functions

```bash
notation_parse(move)
# Parses algebraic notation and sets output variables
# Returns: 0 on success, 1 on invalid

# Examples:
# notation_parse "e4"    → PIECE=P, DEST=e4, CAPTURE=0
# notation_parse "Nf3"   → PIECE=N, DEST=f3, CAPTURE=0
# notation_parse "Bxc6"  → PIECE=B, DEST=c6, CAPTURE=1
# notation_parse "exd5"  → PIECE=P, DEST=d5, CAPTURE=1
```

#### Parsing Rules
1. If first char is uppercase K/Q/R/B/N → that's the piece
2. Otherwise → pawn move
3. If contains 'x' → capture
4. Last two chars → destination square

### 3. lib/freq.sh - Frequency Mapping

#### Musical Mapping
| Column | Note | Semitones from A |
|--------|------|------------------|
| a | C | -9 |
| b | D | -7 |
| c | E | -5 |
| d | F | -4 |
| e | G | -2 |
| f | A | 0 |
| g | B | 2 |
| h | C | 3 |

Ranks 1-8 add octave offsets: rank 4 = base octave.

#### Frequency Formula
```
freq = 440 * 2^(semitones/12) * 2^(octave_offset)
```

For integer math, we scale by 1000:
```
FREQ_SCALED = freq * 1000
```

#### Functions

```bash
freq_from_square(square)
# Returns: frequency in Hz (integer, unscaled)
# Example: freq_from_square "f4" → 440
# Example: freq_from_square "a4" → 262
```

#### Lookup Table (precomputed for octave 4)
```bash
declare -A FREQ_BASE=(
    [a]=262 [b]=294 [c]=330 [d]=349
    [e]=392 [f]=440 [g]=494 [h]=523
)
```

Octave adjustment: multiply/divide by 2 per octave.

### 4. lib/synth.sh - Waveform Generator

#### Parameters
- SAMPLE_RATE=44100
- DURATION_MS=300
- AMPLITUDE=32000 (max 32767 for 16-bit signed)

#### Sine Wave Algorithm

Without floating point, we use a lookup table for sin values scaled by 10000.

```bash
# Precomputed sin table (0-359 degrees, scaled by 10000)
declare -a SIN_TABLE=(
    0 175 349 523 698 872 1045 1219 ...
)

synth_sine(frequency, duration_ms)
# Outputs: raw 16-bit signed PCM samples to stdout
# Sample count = SAMPLE_RATE * duration_ms / 1000
```

#### Sample Generation Loop
```bash
samples=$((SAMPLE_RATE * duration_ms / 1000))
phase_increment=$((frequency * 360 * 10000 / SAMPLE_RATE))
phase=0

for ((i=0; i<samples; i++)); do
    degree=$((phase / 10000 % 360))
    sample=$((SIN_TABLE[degree] * AMPLITUDE / 10000))
    # Output sample as 2 bytes, little-endian
    phase=$((phase + phase_increment))
done
```

### 5. lib/wav.sh - WAV Output

#### WAV Header Format (44 bytes)

| Offset | Size | Description | Value |
|--------|------|-------------|-------|
| 0 | 4 | ChunkID | "RIFF" |
| 4 | 4 | ChunkSize | file_size - 8 |
| 8 | 4 | Format | "WAVE" |
| 12 | 4 | Subchunk1ID | "fmt " |
| 16 | 4 | Subchunk1Size | 16 |
| 20 | 2 | AudioFormat | 1 (PCM) |
| 22 | 2 | NumChannels | 1 (mono) |
| 24 | 4 | SampleRate | 44100 |
| 28 | 4 | ByteRate | 88200 |
| 32 | 2 | BlockAlign | 2 |
| 34 | 2 | BitsPerSample | 16 |
| 36 | 4 | Subchunk2ID | "data" |
| 40 | 4 | Subchunk2Size | num_samples * 2 |

#### Functions

```bash
wav_header(num_samples)
# Outputs: 44-byte WAV header to stdout

wav_write_sample(value)
# Outputs: 16-bit signed integer as 2 bytes, little-endian
# value: -32768 to 32767
```

#### Byte Output
```bash
# Little-endian 16-bit
printf "\\x$(printf '%02x' $((value & 0xFF)))"
printf "\\x$(printf '%02x' $(((value >> 8) & 0xFF)))"

# Little-endian 32-bit
for i in 0 8 16 24; do
    printf "\\x$(printf '%02x' $(((value >> i) & 0xFF)))"
done
```

### 6. chesswav - Main Script

```bash
#!/usr/bin/env bash
set -euo pipefail

source lib/board.sh
source lib/notation.sh
source lib/freq.sh
source lib/synth.sh
source lib/wav.sh

main() {
    board_init

    # Collect all samples
    local samples=()

    while read -r line; do
        for move in $line; do
            notation_parse "$move" || continue
            board_update  # Apply move to board
            freq=$(freq_from_square "$NOTATION_DEST")
            synth_sine "$freq" 300 >> /tmp/samples.raw
        done
    done

    # Count samples and write WAV
    local byte_count=$(wc -c < /tmp/samples.raw)
    local num_samples=$((byte_count / 2))

    wav_header "$num_samples"
    cat /tmp/samples.raw

    rm -f /tmp/samples.raw
}

main
```

## Test Strategy

### Unit Tests per Module

#### tests/test_board.sh
- test_board_init: verify initial position
- test_board_get: verify piece retrieval
- test_board_set: verify piece placement
- test_empty_square: verify empty returns ""

#### tests/test_notation.sh
- test_pawn_move: "e4" → P, e4
- test_piece_move: "Nf3" → N, f3
- test_capture: "Bxc6" → B, c6, capture=1
- test_pawn_capture: "exd5" → P, d5, capture=1

#### tests/test_freq.sh
- test_a4: a4 → 262 Hz
- test_f4: f4 → 440 Hz
- test_octave_up: a5 → 523 Hz
- test_octave_down: a3 → 131 Hz

#### tests/test_synth.sh
- test_sample_count: verify correct number of samples
- test_sample_range: verify samples within -32768 to 32767

#### tests/test_wav.sh
- test_header_size: verify 44 bytes
- test_riff_marker: verify starts with "RIFF"
- test_wave_marker: verify "WAVE" at offset 8

#### tests/test_integration.sh
- test_full_pipeline: "e4 e5 Nf3 Nc6" produces valid WAV
- test_wav_plays: file recognized by `file` command

### Test Runner

```bash
#!/usr/bin/env bash
# tests/run_all.sh

PASS=0
FAIL=0

for test_file in tests/test_*.sh; do
    source "$test_file"
done

echo "Passed: $PASS, Failed: $FAIL"
exit $((FAIL > 0 ? 1 : 0))
```

## Error Handling

- Invalid moves: skip and continue
- Empty input: output silent WAV (header only)
- Parse errors: log to stderr, continue

## Performance Considerations

- Precompute sin lookup table (360 values)
- Use `printf` buffering for byte output
- Minimize subshell spawning
- Target: < 100ms per move synthesis
