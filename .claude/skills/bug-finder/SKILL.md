---
name: bug-finder
description: Hunt bugs through edge cases, corrupted input, stress testing. Use when: find bugs, break this, edge cases, stress test, QA this, what could go wrong, chaos test, find issues, hunt bugs, debug this, why is this failing, not working.
---

# Bug Finder

## Philosophy

**Reproduce first, then fix.** Never assume a bug exists until you see it fail.

## Bug Hunting Strategy

### 1. Input Fuzzing

```rust
// Empty input
parse_move("")
parse_pgn("")

// Invalid notation
parse_move("Zz9")
parse_move("!@#$")

// Unicode
parse_move("e4\u{266E}e5")

// Extremely long input
let long_game = "e4 e5 ".repeat(10000);
parse_pgn(&long_game)
```

### 2. Boundary Testing

```rust
// Board edges
board.piece_at(Square::new(File::A, Rank::One))   // corner
board.piece_at(Square::new(File::H, Rank::Eight)) // opposite corner

// Frequency boundaries
square_to_freq(Square::A1) // lowest frequency
square_to_freq(Square::H8) // highest frequency

// Sample count boundaries
generate_samples(freq, 0)       // zero samples
generate_samples(freq, 1)       // single sample
generate_samples(freq, 1 << 24) // large sample count

// Amplitude boundaries: clipping at i16::MIN and i16::MAX
```

### 3. State Corruption

```rust
// Board state after illegal sequences
// Move on empty square
// Capture non-existent piece
// Castle when king or rook already moved
// En passant when not available
// Promotion on wrong rank
```

### 4. Arithmetic Edge Cases

```rust
// Integer overflow in sample calculation
// Frequency at extreme octaves
// Division by zero in envelope calculations
// Negative duration values
```

## Bug Report Format

```markdown
## Bug Report

### Summary
One-line description

### Reproduction
Exact code/commands to reproduce

### Expected
What should happen

### Actual
What actually happens (include error output)

### Root Cause
Analysis of why it fails

### Fix
Code change with failing test
```

## Edge Case Checklist

- [ ] Empty input
- [ ] Invalid chess notation
- [ ] Out-of-range squares
- [ ] Illegal moves
- [ ] Very long games (100+ moves)
- [ ] Special characters in PGN comments
- [ ] All special moves (castling, en passant, promotion)
- [ ] Disambiguation edge cases (multiple pieces can reach same square)
- [ ] Integer overflow in audio math
- [ ] Zero-length audio generation

## Workflow

1. **Identify target** - What function/module to stress?
2. **List assumptions** - What does the code assume about input?
3. **Violate assumptions** - Create inputs that break those assumptions
4. **Write failing test** - Capture the bug as `#[test]`
5. **Fix** - Minimum change to pass the test
6. **Run full suite** - `cargo test`
