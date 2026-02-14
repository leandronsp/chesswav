---
name: review
description: Deep code review - Rust idioms, audio correctness, chess logic, safety. Use when: review, review this, code review, check this code, review my changes, analyze this, critique this, is this good, what do you think, check my implementation.
---

# Code Review - Rust, Audio & Chess Expert

## Review Priorities

### 1. Correctness
- Does the math work? (frequencies, waveforms, byte calculations)
- Is the chess logic right? (legal moves, board state, disambiguation)
- Are edge cases handled?

### 2. Rust Idioms

```rust
// BAD: clone to silence borrow checker
let piece = board.piece_at(square).clone();

// GOOD: borrow or restructure
let piece = board.piece_at(square);

// BAD: unwrap in production
let value = result.unwrap();

// GOOD: propagate or handle
let value = result?;

// BAD: wildcard on own enums
match piece {
    King => ...,
    _ => ...,
}

// GOOD: exhaustive match
match piece {
    King => ...,
    Queen => ...,
    Rook => ...,
    Bishop => ...,
    Knight => ...,
    Pawn => ...,
}
```

### 3. Safety (replaces standalone /security)
- No `unwrap()` in production code
- No `unsafe` without justification
- Integer overflow handled (use `checked_*` or `saturating_*`)
- No panics on invalid input (return `Result`)
- Input validated at system boundaries (CLI args, PGN parsing)

### 4. Architecture
- Separation of concerns (REPL/CLI thin, engine in domain modules)
- `main.rs` stays thin
- Naming favors meaning over brevity
- Functions decomposed if not understandable at a glance

### 5. Performance
- Prefer borrowing over cloning
- Avoid unnecessary allocations in hot paths
- Pre-calculate constants

## Review Output Format

```markdown
### Critical
1) **Issue**: description
   **Fix**: solution

### Important
A) **Issue**: description
   **Suggestion**: approach

### Minor
* Nitpick or suggestion
```

## Checklists

### Rust
- [ ] No `unwrap()` in production code
- [ ] No unnecessary `.clone()`
- [ ] Exhaustive match on own enums
- [ ] `cargo clippy` passes
- [ ] `cargo test` passes
- [ ] Error types with `Display` impl

### Audio
- [ ] Equal temperament frequency calculations
- [ ] Sample rate consistent (44100 Hz)
- [ ] 16-bit signed samples clamped to `i16` range
- [ ] Little-endian byte order for WAV
- [ ] Correct RIFF chunk sizes

### Chess
- [ ] Algebraic notation correctly parsed
- [ ] Board state updated after each move
- [ ] Disambiguation handled (Rad1 vs Rfd1)
- [ ] Special moves: castling, en passant, promotion

## When to Review

- After `/tdd` completes a task
- Before merging feature branches
- When refactoring existing code
- After bug fixes
