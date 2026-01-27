# ChessWAV

Transform chess games into audio. Each move becomes a note.



https://github.com/user-attachments/assets/837ae999-e6b3-4f3d-a724-1cf0ae06d68e





## Quick Start

```bash
# Generate WAV file
echo "e4 e5 Nf3 Nc6" | cargo run --release > game.wav

# Play audio directly (macOS/Linux)
echo "e4 e5 Nf3 Nc6" | cargo run --release -- --play
```

## Installation

```bash
# Build only
cargo build --release

# Install globally (adds `chesswav` to PATH)
cargo install --path .
```

## Usage

### With cargo run

```bash
# Generate WAV to file
echo "e4 e5 Nf3 Nc6" | cargo run --release > game.wav

# Play directly
echo "e4 e5 Nf3 Nc6" | cargo run --release -- --play
echo "e4 e5 Nf3 Nc6" | cargo run --release -- -p

# From file
cargo run --release < moves.txt > output.wav
```

### After installation

```bash
# Generate WAV to file
echo "e4 e5 Nf3 Nc6" | chesswav > game.wav

# Play directly
echo "e4 e5 Nf3 Nc6" | chesswav --play

# From file
chesswav < moves.txt > output.wav
```

### Using binary directly

```bash
echo "e4 e5 Nf3 Nc6" | ./target/release/chesswav > game.wav
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
├── main.rs      # CLI entry point
├── lib.rs       # Library exports
├── chess.rs     # Domain types (Piece, Square, Move, parser)
├── board.rs     # Board representation
├── freq.rs      # Square to frequency mapping
├── synth.rs     # Sine wave generator
├── wav.rs       # WAV file encoder
└── audio.rs     # Orchestration (notation → WAV)
tests/
└── integration.rs
```

## Testing

```bash
cargo test
```

## Requirements

- Rust 2024 edition (1.85+)
- No external dependencies

## Roadmap

See [ROADMAP.md](ROADMAP.md) for future features:
- Multiple waveforms per piece type
- Castling and en passant
- PGN file parsing
- ADSR envelope

## License

MIT
