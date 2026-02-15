# ChessWAV

Transform chess games into audio. Each move becomes a note.


https://github.com/user-attachments/assets/a1496a92-6b52-4888-84ab-c3219914cb35



## Quick Start

```bash
# Generate WAV file
echo "e4 Nf6 Bb5 Qd8 Rad1 O-O" | cargo run --release > game.wav

# Play audio directly (macOS/Linux)
echo "e4 Nf6 Bb5 Qd8 Rad1 O-O" | cargo run --release -- --play
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

## Display

Interactive mode supports three display modes:

| Mode | Description |
|------|-------------|
| `sprite` | Half-block pixel art with ANSI colors (default) |
| `unicode` | Unicode chess symbols (♔♕♖♗♘♙ / ♚♛♜♝♞♟) |
| `ascii` | Plain text letters (K Q R B N P / k q r b n p) |

### Setting the display mode

At startup with `--display` (or `-d`):

```bash
chesswav --interactive --display sprite
chesswav --interactive -d unicode
chesswav --interactive -d ascii
```

Or switch at any time during the REPL:

```
display unicode
display ascii
display sprite
```

### Color support

The `sprite` and `unicode` modes use ANSI colors. Color depth is auto-detected from the `COLORTERM` environment variable:

| `COLORTERM` value | Color mode |
|-------------------|------------|
| `truecolor` | 24-bit RGB |
| `24bit` | 24-bit RGB |
| *(anything else)* | 256-color xterm palette |

Most modern terminals (iTerm2, Ghostty, WezTerm, Windows Terminal, GNOME Terminal) set `COLORTERM=truecolor` automatically. If colors look wrong, you can override it:

```bash
COLORTERM=truecolor chesswav --interactive
```

The `ascii` mode uses no colors and works in any terminal.

## How it works

- Columns (a-h) map to notes (C, D, E, F, G, A, B, C)
- Ranks (1-8) map to octaves (low to high)
- Each piece has a distinct timbre (waveform)
- Castling (`O-O`, `O-O-O`) is supported

### Piece Timbres

| Piece | Waveform | Character |
|-------|----------|-----------|
| Pawn | Sine | Pure, simple |
| Knight | Triangle | Mellow, soft |
| Rook | Square | Hollow, woody |
| Bishop | Sawtooth | Bright, buzzy |
| Queen | Composite (5 harmonics) | Rich, full |
| King | Harmonics | Warm, noble |

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
├── main.rs              # CLI entry point
├── lib.rs               # Library exports
├── engine/
│   ├── mod.rs           # Engine module exports
│   ├── chess.rs         # Domain types (Piece, Square, Move, parser)
│   ├── board.rs         # Board representation & move execution
│   └── hint.rs          # Move disambiguation hints
├── audio/
│   ├── mod.rs           # Audio module exports
│   ├── freq.rs          # Square to frequency mapping
│   ├── synth.rs         # Note synthesis & orchestration
│   ├── wav.rs           # WAV file encoder
│   ├── waveform.rs      # Waveform generators (sine, triangle, square, saw)
│   └── blend.rs         # Waveform blending for composite timbres
└── tui/
    ├── mod.rs           # TUI module exports
    ├── repl.rs          # Interactive REPL
    └── display/
        ├── mod.rs       # Display mode abstraction
        ├── sprite.rs    # Half-block pixel art renderer
        ├── unicode.rs   # Unicode chess symbol renderer
        ├── ascii.rs     # Plain text renderer
        └── colors.rs    # ANSI color support (truecolor/256)
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
- ~~Multiple waveforms per piece type~~ done
- ~~Castling~~ done
- En passant
- PGN file parsing
- ADSR envelope

## License

MIT
