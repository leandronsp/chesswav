# ChessWAV

Transform chess games into audio. Each move becomes a note.

## Usage

```bash
# Pipe moves to stdout
echo "e4 e5 Nf3 Nc6" | ./chesswav > game.wav

# Play directly
echo "e4 e5 Nf3 Nc6" | ./chesswav --play

# Or with process substitution
afplay <(echo "e4 e5 Nf3 Nc6" | ./chesswav)
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
chesswav              # Main executable
lib/
├── board.sh          # Board representation
├── notation.sh       # Algebraic notation parser
├── freq.sh           # Square to frequency mapping
├── synth.sh          # Sine wave generator
└── wav.sh            # WAV file output
tests/
├── test_*.sh         # Unit tests per module
└── run_all.sh        # Test runner
docs/
├── prd/mvp.md        # Product requirements
└── spec/mvp.md       # Technical specification
```

## Testing

```bash
./tests/run_all.sh
```

## Requirements

- Bash 3.2+ (macOS default works)
- No external dependencies

## Roadmap

See [ROADMAP.md](ROADMAP.md) for future features:
- Multiple waveforms per piece type
- Castling and en passant
- PGN file parsing
- ADSR envelope
- Interactive mode

## License

MIT
