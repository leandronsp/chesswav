# ChessWAV

Transform chess games into audio. Each move becomes a note, each piece has its timbre, each capture has its drama.

## Usage

```bash
# From PGN file
./chesswav game.pgn -o output.wav

# From STDIN
echo "e4 e5 Nf3 Nc6" | ./chesswav > game.wav

# Interactive mode
./chesswav --interactive
```

## How it works

- Columns (a-h) map to notes (C, D, E, F, G, A, B, C)
- Ranks (1-8) map to octaves (low to high)
- Each piece type has a unique waveform

| Piece | Sound |
|-------|-------|
| Pawn | Sine wave |
| Knight | Triangle wave |
| Rook | Square wave |
| Bishop | Sawtooth wave |
| Queen | Composite wave |

## Requirements

- Bash 4.0+
- No external dependencies

## License

MIT
