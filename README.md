# ChessWAV

Transform chess games into audio. Each move becomes a note.

## Installation

```bash
cargo build --release
```

## Usage

```bash
# Pipe moves to stdout
echo "e4 e5 Nf3 Nc6" | ./target/release/chesswav > game.wav

# Play directly
echo "e4 e5 Nf3 Nc6" | ./target/release/chesswav --play

# From file
./target/release/chesswav < moves.txt > output.wav
```

## How it works

- Columns (a-h) map to notes (C, D, E, F, G, A, B, C)
- Ranks (1-8) map to octaves (low to high)
- Each move produces a 300ms sine wave tone

### Musical Mapping

| Square | Note | Frequency |
|--------|------|-----------|
| a4 | C4 | 262 Hz |
| b4 | D4 | 294 Hz |
| c4 | E4 | 330 Hz |
| d4 | F4 | 349 Hz |
| e4 | G4 | 392 Hz |
| f4 | A4 | 440 Hz |
| g4 | B4 | 494 Hz |
| h4 | C5 | 523 Hz |

Higher ranks = higher octaves. `e5` is an octave above `e4`.

## Project Structure

```
src/
├── main.rs        # CLI entry point
├── lib.rs         # Library exports
├── types.rs       # Shared domain types (PieceKind, Color, Square)
├── audio.rs       # Audio constants and generation
├── board.rs       # Board representation
├── notation.rs    # Algebraic notation parser
├── freq.rs        # Square to frequency mapping
├── synth.rs       # Sine wave generator
└── wav.rs         # WAV file output
tests/
└── integration.rs # End-to-end tests
```

## Testing

```bash
cargo test
```

## Requirements

- Rust 1.70+
- No external dependencies

## Roadmap

See [ROADMAP.md](ROADMAP.md) for future features:
- Multiple waveforms per piece type
- Castling and en passant
- PGN file parsing
- ADSR envelope

## License

MIT
