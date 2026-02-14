# PRD: Streaming/Interactive Mode

> Source: ROADMAP.md → Epic 4 → Feature 4.2

## Problem Statement

Currently, ChessWAV operates in batch mode only: the user pipes all moves at once via stdin, and the entire game is synthesized into a single WAV output. There's no way to play a game move-by-move and hear each move as it happens. This eliminates the real-time feedback loop that makes the audio experience engaging — hearing a knight land on f3 right after typing `Nf3`.

## Goal

A REPL mode where the user types one move at a time and hears it immediately. The board state advances with each move, an ASCII board is displayed, and the audio plays through the system speaker. The session is a living game that can be reset or exited.

## Success Criteria

- `chesswav --interactive` launches a REPL prompt
- Typing `e4` plays the pawn-to-e4 tone and shows the board
- Typing `quit` exits cleanly
- Typing `reset` starts a fresh game
- Invalid moves show an error without crashing
- All existing batch-mode behavior is unaffected
- No external dependencies added

## Current State

```
echo "e4 e5 Nf3" | chesswav --play
                        ↑
          all moves must be known upfront
          audio plays AFTER full synthesis
```

The CLI reads stdin to EOF, generates all audio at once, then either writes WAV to stdout or plays via system player. No interactivity.

## Target State

```
$ chesswav --interactive

  ♟ ChessWAV Interactive Mode
  Type moves in algebraic notation. Commands: reset, quit

  [Move 1 - White] > e4
  ♪ Playing...
  8 | r n b q k b n r
  7 | p p p p p p p p
  6 | . . . . . . . .
  5 | . . . . . . . .
  4 | . . . . P . . .
  3 | . . . . . . . .
  2 | P P P P . P P P
  1 | R N B Q K B N R
    +----------------
      a b c d e f g h

  [Move 2 - Black] > e5
  ♪ Playing...
  ...
```

## Requirements

### Functional

- [ ] FR-1: `--interactive` or `-i` flag activates REPL mode
- [ ] FR-2: Prompt displays move number and side to move (White/Black)
- [ ] FR-3: Each valid move is parsed, synthesized, and played immediately
- [ ] FR-4: ASCII board is displayed after each move
- [ ] FR-5: Move number increments correctly (1 for White+Black, 2 for next White+Black, etc.)
- [ ] FR-6: `quit` command exits the REPL with exit code 0
- [ ] FR-7: `reset` command clears the board to starting position and resets move counter
- [ ] FR-8: Invalid input prints an error message and re-prompts (does not crash)
- [ ] FR-9: Empty input (just Enter) re-prompts silently
- [ ] FR-10: Ctrl+D (EOF) exits cleanly, same as `quit`

### Non-Functional

- [ ] NFR-1: Audio plays with no perceptible delay after pressing Enter (<100ms to start playback)
- [ ] NFR-2: REPL remains responsive while audio is playing (audio playback should not block the next prompt for longer than the note duration)

## Technical Approach

### New Module: `repl.rs`

The interactive loop lives in a dedicated module to keep `main.rs` thin.

```
main.rs
  ↓ --interactive flag detected
repl::run()
  ↓ loop
  read line from stdin
  ↓ match command
  "quit" / EOF → break
  "reset"      → reset board, reset move counter
  move string  → parse → synthesize → play → display board
```

### Board State Tracking

The current batch mode is stateless — it parses each move independently using only the move index to determine white/black starting rank. Interactive mode needs actual board state to:

1. Display the ASCII board after each move
2. Validate that the typed move makes sense in context (future enhancement)

For v1, we need a minimal board representation:

```rust
pub struct Board {
    squares: [[Option<(Piece, Color)>; 8]; 8],
}
```

This is **Feature 1.1 (Board Representation)** from the roadmap. The interactive mode is the first consumer that actually needs it. The board tracks piece positions and updates them after each move.

**Color enum**: `White` / `Black` — needed to display uppercase (White) vs lowercase (Black) pieces on the ASCII board.

### Move Execution

After parsing a move, the board needs to be updated:

- Move piece from origin to destination
- Remove captured piece (if capture)
- Handle castling (move both king and rook)
- Handle promotion (replace pawn with promoted piece)

This is **Feature 1.4 (Move Executor)** from the roadmap, scoped minimally for interactive mode.

**Key limitation**: The current parser (`Move::parse`) does not determine the origin square — it only knows the destination. For board updates, we need to infer the origin. This can be done by scanning the board for a matching piece that can reach the destination. This is essentially **Feature 1.3 (Move Resolver)** scoped to the minimum needed.

### Audio Playback

Reuse the existing `play()` function from `main.rs`, extracted into a shared location. Each move generates samples for a single move (not the full game), converts to WAV, writes to temp file, and plays via system player.

### ASCII Board Display

A `Display` implementation on `Board` that outputs:

```
8 | r n b q k b n r
7 | p p p p p p p p
6 | . . . . . . . .
  ...
1 | R N B Q K B N R
  +----------------
    a b c d e f g h
```

Uppercase = White, lowercase = Black, `.` = empty.

## Data Flow Per Move

```
User types: "Nf3"
  ↓
repl: read line, trim
  ↓
chess::Move::parse("Nf3", move_index)
  ↓ Move { piece: Knight, dest: f3, ... }
board: resolve origin (find knight that can reach f3)
  ↓ origin = g1
board: apply move (g1 → f3)
  ↓
audio: move_to_samples(&move) → Vec<i16>
audio: to_wav(&samples) → Vec<u8>
  ↓
play(&wav) via afplay/aplay
  ↓
board: display ASCII board
  ↓
prompt next move
```

## Scope of Board Implementation

To keep this manageable, the board for v1 needs:

| Capability | Needed? | Notes |
|-----------|---------|-------|
| Initial position setup | Yes | Standard chess starting position |
| Piece placement/removal | Yes | For move execution |
| ASCII display | Yes | Core feature |
| Origin resolution | Yes | Find which piece moves to destination |
| Castling execution | Yes | Move king + rook |
| En passant | No | Defer to v2 |
| Full legality validation | No | Trust the user's notation |
| Check/checkmate detection | No | Trust annotations in notation |
| Move history | No | Defer to v2 |

The board trusts the user — if they type `Nf3`, we find a knight that can reach f3 and move it. We don't verify check legality or pin constraints.

## Commands

| Command | Action |
|---------|--------|
| `quit` | Exit REPL |
| `reset` | New game (starting position, move 1) |
| Any algebraic notation | Parse, play, display |
| Empty line | Re-prompt |

## Error Handling

| Scenario | Behavior |
|----------|----------|
| Unparseable move (`xyz`) | Print "Invalid move: xyz" and re-prompt |
| No matching piece found (`Nf3` but no knight can reach f3) | Print "No piece found for: Nf3" and re-prompt |
| System audio player missing | Print warning once, continue without audio |

## Implementation Order

1. Add `Color` enum to `chess.rs` (White, Black)
2. Add `Board` struct with initial position setup
3. Implement ASCII display for `Board`
4. Implement basic origin resolution (find piece → destination)
5. Implement move execution (update board squares)
6. Handle castling in board execution
7. Handle promotion in board execution
8. Extract `play()` from `main.rs` into shared location
9. Create `repl.rs` with the REPL loop
10. Add `--interactive` / `-i` flag to CLI
11. Integration tests for the REPL

## Test Cases

### Board Tests

```rust
#[test]
fn initial_position_has_white_pawns_on_rank_2() {
    let board = Board::new();
    for file in 0..8 {
        assert_eq!(board.get(file, 1), Some((Piece::Pawn, Color::White)));
    }
}

#[test]
fn apply_move_updates_board() {
    let mut board = Board::new();
    board.apply_move(Square::new(4, 1), Square::new(4, 3)); // e2 → e4
    assert_eq!(board.get(4, 1), None);
    assert_eq!(board.get(4, 3), Some((Piece::Pawn, Color::White)));
}

#[test]
fn ascii_display_initial_position() {
    let board = Board::new();
    let display = format!("{board}");
    assert!(display.contains("r n b q k b n r"));
    assert!(display.contains("P P P P P P P P"));
}
```

### REPL Tests

```rust
#[test]
fn repl_parses_quit() {
    // Simulate stdin with "quit\n"
    // Assert clean exit
}

#[test]
fn repl_handles_invalid_move() {
    // Simulate stdin with "xyz\nquit\n"
    // Assert error message printed, no crash
}
```

## Out of Scope

- En passant tracking and execution
- Full legal move validation (pins, checks)
- Move history / undo
- PGN export from interactive session
- Saving the generated audio to file
- Multiplayer / network play
- Board colors or Unicode piece characters

## Dependencies

- `chess.rs` — existing `Move::parse()`, `Piece`, `Square`
- `audio.rs` — existing `generate()` pipeline, `move_to_samples()` (needs to be made accessible for single-move synthesis)
- `synth.rs`, `freq.rs`, `wav.rs` — used indirectly via audio module

### New Modules

- `board.rs` — Board representation, display, move execution
- `repl.rs` — Interactive REPL loop

### Prerequisite Work

`move_to_samples()` in `audio.rs` is currently private. It needs to be either made public or wrapped in a public function that synthesizes a single move to WAV bytes, so `repl.rs` can call it.

## Open Questions

1. **Origin resolution complexity**: Should we implement full piece movement rules (knight L-shape, bishop diagonals, etc.) or use a simpler heuristic (find the only piece of that type that could plausibly reach the destination based on file/rank)?
2. **Blocking vs async playback**: Should the prompt wait for audio to finish (`afplay` blocks), or should we spawn playback in a background thread so the user can type the next move immediately?
3. **Board module scope**: Should `board.rs` from this feature become the foundation for Feature 1.1, or should we keep it minimal here and build Feature 1.1 separately with a richer API?
