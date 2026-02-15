---
name: review
description: Deep code review - Rust idioms, audio correctness, chess logic, safety. Builds a plan to address findings. Use when: review, review this, code review, check this code, review my changes, is this good, what do you think.
---

# Code Review - Rust, Audio & Chess Expert

**Reviews current changes, runs tech debt audit, and builds a plan to address findings.**

## Workflow

1. **Diff the branch** against main to understand all changes
2. **Review** using the priorities below
3. **Run `/techdebt`** to audit for pattern deviations and code smells
4. **Enter plan mode** with a fix plan if there are Critical or Important findings
5. **Implement fixes** after user approves the plan
6. **Re-run `cargo test` and `cargo clippy`** to confirm everything is clean

If the review finds nothing actionable, skip the plan and report the verdict.

## Review Priorities

### 0. Documentation
- Does `README.md` reflect the current project structure, features, and usage?
- Does `CLAUDE.md` "File Structure" and module list match the actual codebase?
- Are `//!` doc comments in `main.rs` and `lib.rs` up to date with CLI flags and usage?
- Are `///` doc comments on changed public types/functions accurate?
- If the changes add/remove/rename modules, files, CLI flags, or features, **all affected docs must be updated**

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

### 3. Safety
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

### Verdict
[ ] Clean - ready for `/pr`
[ ] Needs fixes - see plan below
```

## Checklists

### Documentation
- [ ] `README.md` "Project Structure" matches actual `src/` layout
- [ ] `CLAUDE.md` "File Structure" matches actual `src/` layout
- [ ] `main.rs` doc comment reflects all CLI flags and usage
- [ ] New features/flags documented in README
- [ ] Removed features/flags cleaned from README and doc comments

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

## Pipeline

```
/tdd <issue_url> -> /review (+ /techdebt) -> /pr
```
