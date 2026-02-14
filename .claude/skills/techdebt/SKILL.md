---
name: techdebt
description: Analyze current changes for potential tech debt before review or merge. Use when preparing code for review, checking for code smells, or auditing changes before PR.
---

# Tech Debt Auditor - Rust & ChessWAV

## Purpose

**Staff Engineer perspective**: Catch tech debt BEFORE it reaches main. Better to flag and discuss than let debt accumulate silently.

Anything that deviates from established patterns is a **red flag**.

## What to Audit

1. Diff current branch vs main
2. Check all changed files match codebase patterns
3. Flag deviations as tech debt

```bash
git diff main...HEAD --name-only
git diff main...HEAD
```

---

## Rust Patterns

### Critical - Ownership & Safety

| Red Flag | Problem | Correct Pattern |
|----------|---------|-----------------|
| `.clone()` to fix borrow errors | Hides ownership issues | Restructure borrows, use references |
| `.unwrap()` in production code | Panics on None/Err | Use `?`, `if let`, `match`, combinators |
| `unsafe` without justification | Unnecessary risk | Safe alternatives first |
| `_ =>` on own enums | Hides missing arms | Exhaustive match, list all variants |
| `mut` when not needed | Violates immutability default | Remove unnecessary `mut` |
| Boolean parameters | Unclear call sites | Replace with enums |

### Naming - Must Convey Intent

```rust
// RED FLAG - Single-letter or abbreviated variables
let c = piece.color();
let p = board.get(sq);

// CORRECT
let color = piece.color();
let piece_at_target = board.get(square);

// RED FLAG - Magic expressions inline
if move_index % 2 == 0 { ... }

// CORRECT - Extract into named function
fn is_white_turn(move_index: usize) -> bool { move_index % 2 == 0 }

// RED FLAG - None literal for domain ops
self.set(square, None);

// CORRECT - Semantic method
self.clear_square(square);
```

### Getters & Conversions (Rust API Guidelines)

```rust
// RED FLAG
fn get_name(&self) -> &str
fn convert_to_string(&self) -> String

// CORRECT
fn name(&self) -> &str          // C-GETTER
fn to_string(&self) -> String   // C-CONV: to_ may allocate
fn as_str(&self) -> &str        // C-CONV: as_ free view
fn into_inner(self) -> T        // C-CONV: into_ consumes
```

### Error Handling

```rust
// RED FLAG - unwrap chains
let piece = board.piece_at(square).unwrap();
let freq = square_to_freq(&notation).unwrap();

// CORRECT - propagate with ?
let piece = board.piece_at(square)?;
let freq = square_to_freq(&notation)?;

// RED FLAG - Generic error types
fn parse(input: &str) -> Result<Move, String>

// CORRECT - Module-specific error enums
fn parse(input: &str) -> Result<Move, ParseMoveError>
```

### Architecture - Separation of Concerns

```rust
// RED FLAG - REPL/CLI doing engine work
// main.rs or repl.rs resolving castling, computing hints, finding origins

// CORRECT - Thin CLI, engine in domain modules
// main.rs: parse args, delegate
// board.rs: move resolution, disambiguation, state
// notation.rs: parsing
```

### Self vs Self::

```rust
// RED FLAG - Inconsistent usage
impl Board {
    fn piece_at(&self, sq: Square) -> Option<&Piece> {
        Self::lookup(&self.squares, sq)  // Don't use Self:: for methods taking &self
    }
}

// CORRECT
impl Board {
    fn new() -> Self { ... }            // Self:: for constructors
    fn piece_at(&self, sq: Square) { self.lookup(sq) }  // self. for instance methods
}
```

---

## Audio Patterns

### Critical - Correctness

| Red Flag | Problem | Correct Pattern |
|----------|---------|-----------------|
| Hardcoded frequencies | Drift from equal temperament | Calculate from A4=440Hz formula |
| `as i16` without clamping | Overflow wraps silently | Clamp to `i16::MIN..=i16::MAX` |
| Wrong byte order in WAV | Corrupted audio | Little-endian via `to_le_bytes()` |
| Inconsistent sample rate | Clicks/distortion | Always 44100 Hz |
| Magic numbers for durations | Unclear intent | Named constants |

```rust
// RED FLAG - Hardcoded frequency
let freq = 440.0;

// CORRECT - Calculate from formula
fn note_frequency(semitones_from_a4: i32) -> f64 {
    440.0 * 2.0_f64.powf(semitones_from_a4 as f64 / 12.0)
}

// RED FLAG - Unclamped cast
let sample = (amplitude * wave_value) as i16;

// CORRECT
let sample = (amplitude * wave_value).clamp(i16::MIN as f64, i16::MAX as f64) as i16;
```

---

## Chess Patterns

### Critical - Logic Correctness

| Red Flag | Problem | Correct Pattern |
|----------|---------|-----------------|
| Missing disambiguation | Wrong piece moves | Check file/rank hints |
| En passant without history | Illegal captures | Track last move |
| Castling without validation | Move through check | Validate path clear + no check |
| Ignoring check state | Illegal moves allowed | Validate king safety |
| Hardcoded starting position | Inflexible | Build from FEN or setup function |

---

## Code Smells (AI-Generated)

### Over-commenting

```rust
// RED FLAG
// This function calculates the frequency for a given square
// It takes a square parameter and returns an f64
// The frequency is based on equal temperament tuning
fn square_frequency(square: Square) -> f64 { ... }

// CORRECT - Self-documenting name, no obvious comments
fn square_frequency(square: Square) -> f64 { ... }
```

### Unnecessary abstraction

```rust
// RED FLAG - Single-use helper
fn validate_not_empty(input: &str) -> Result<(), ParseError> {
    if input.is_empty() { Err(ParseError::EmptyInput) } else { Ok(()) }
}

// CORRECT - Inline when used once
if input.is_empty() { return Err(ParseError::EmptyInput); }
```

### Defensive overkill

```rust
// RED FLAG - Internal code doesn't need this
fn process(board: &Board, square: Square) -> Option<Piece> {
    if square.file() > 7 { return None; }
    if square.rank() > 7 { return None; }
    // Square is already validated by its type...
    board.piece_at(square)
}
```

### Breaking established patterns

```rust
// RED FLAG - New pattern when codebase uses another
// Check how similar features are implemented first
```

---

## Output Format

```markdown
## Tech Debt Audit - [Branch Name]

### Summary
- Files changed: X
- Debt items found: N

### Critical (Must Fix)

1. **`.unwrap()` in production** - `src/notation.rs:45`
   **Issue**: Panics on malformed input
   **Fix**: Return `Result` with `ParseMoveError`

2. **Unclamped sample cast** - `src/synth.rs:78`
   **Issue**: `as i16` wraps on overflow
   **Fix**: `.clamp(i16::MIN as f64, i16::MAX as f64) as i16`

### Should Fix

A) **Single-letter variable** - `src/board.rs:23`
   **Issue**: `let p = ...` conveys no intent
   **Fix**: `let piece = ...`

B) **Wildcard match on own enum** - `src/notation.rs:60`
   **Issue**: `_ =>` hides missing arms
   **Fix**: List all `PieceKind` variants

### Consider

* Could extract magic number `300.0` into `const NOTE_DURATION_MS`
* Missing `Display` impl on `BoardError`

### Positive Notes
- Good use of borrowing over cloning
- Proper test coverage
- Consistent naming in synth module

### Verdict
[ ] Ready for review
[ ] Needs fixes - see Critical
```

## Remember

- **Pattern deviation = red flag** - If it doesn't match codebase, flag it
- **Reference existing code** - Show where the correct pattern is used
- **Better to over-flag** - Discussion is better than silent debt
- **Run clippy** - `cargo clippy -- -D warnings` catches mechanical issues
- **Check the tables above** - Common issues specific to Rust/audio/chess
