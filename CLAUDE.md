# ChessWAV Project Guidelines

## Project Context

ChessWAV transforms chess games into audio. Each move becomes a note, each piece has its timbre, each capture has its drama.

## Domain Expertise

### Chess Notation
- **Algebraic notation**: `e4`, `Nf3`, `Bxc6`, `O-O`, `e8=Q`
- **Disambiguation**: `Rad1` (rook from a-file), `N5f3` (knight from rank 5)
- **Annotations**: `+` (check), `#` (checkmate), `!`, `?`
- **PGN format**: Headers `[Event "..."]`, movetext, comments `{...}`, variations `(...)`

### Audio Synthesis
- **Waveforms**: sine, triangle, square, sawtooth, composite
- **Frequency**: A4 = 440Hz, equal temperament tuning
- **WAV format**: RIFF header, 16-bit PCM, little-endian
- **ADSR envelope**: Attack, Decay, Sustain, Release

### Musical Mapping
| Column | Note |
|--------|------|
| a | C |
| b | D |
| c | E |
| d | F |
| e | G |
| f | A |
| g | B |
| h | C (octave up) |

Ranks 1-8 map to octaves (lower to higher).

### Piece Timbres
| Piece | Waveform |
|-------|----------|
| Pawn | Sine |
| Knight | Triangle |
| Rook | Square |
| Bishop | Sawtooth |
| Queen | Composite |
| King | Sine + harmonics |

## Implementation Language

**Rust** - Production-grade, zero dependencies:
- No external crates for core functionality
- Pure stdlib for audio synthesis and WAV output
- Binary I/O via `std::io::Write`

## Code Standards

- No external dependencies (pure Rust stdlib)
- Modules per domain: `wav`, `synth`, `freq`, `notation`, `board`
- Unit tests in each module with `#[cfg(test)]`
- Self-documenting function names
- `cargo clippy` must pass with no warnings

## Codebase Conventions

### Naming — Favor Meaning Over Brevity

**No single-letter or abbreviated variables.** Every name must convey intent.

```rust
// Bad
let c = piece.color();
let p = board.get(sq);
let rh = move_text.chars().nth(1);
let fh = move_text.chars().nth(0);

// Good
let color = piece.color();
let piece_at_target = board.get(square);
let rank_hint = move_text.chars().nth(1);
let file_hint = move_text.chars().nth(0);
```

**Extract magic expressions into named functions or variables:**

```rust
// Bad
if move_index % 2 == 0 { ... }
let move_number = move_index / 2 + 1;

// Good
fn is_white_turn(move_index: usize) -> bool { move_index % 2 == 0 }
let move_number = full_move_number(move_index);
```

**Use semantic methods instead of `None` literals for domain operations:**

```rust
// Bad
self.set(square, None);

// Good
self.clear_square(square);
```

**Bind intermediate variables for complex access patterns:**

```rust
// Bad — raw indexing with implicit rank-first, file-second order
self.squares[rank][file]

// Good — wrap in a method that makes the contract explicit
self.piece_at(rank, file)
```

### Function Complexity

- If a function can't be understood at a glance, it needs decomposition
- Functions like `find_origin` with mixed concerns (hints, disambiguation, piece movement) must be split into focused helpers
- Families of related functions (e.g., `_can_reach`) need descriptive names reflecting domain purpose, not generic prefixes

### Architecture — Separation of Concerns

- **REPL/CLI is a thin front layer.** It must not resolve castling, compute hints, or find piece origins. It delegates to the engine.
- **Engine logic lives in domain modules** (`board`, `notation`). The engine owns move resolution, disambiguation, and board state.
- **Hint/disambiguation logic** should be encapsulated in its own struct or module, not scattered across board and repl.
- **`main.rs` stays thin** — parse args, delegate to library code.

### Consistency

- Use `self.method()` for instance methods uniformly. Reserve `Self::` for associated functions (constructors like `Self::new()`) and pure functions that don't need instance state. Don't mix them inconsistently for methods that all take `&self`.
- Match arms on own enums must list all variants explicitly — no wildcard `_ =>` catch-all.

### Comments Policy

Comments explain *why*, not *what*. Not zero comments — contextual comments are valuable:
- Business logic that isn't obvious from naming
- Non-trivial algorithmic choices
- Edge cases and their reasoning
- Domain context (chess rules that inform implementation)

Do not over-comment obvious code. Do not strip all comments either.

## Rust Best Practices

### Idiomatic Patterns

- **Prefer borrowing over cloning.** `.clone()` to silence the borrow checker is an anti-pattern. Restructure code, use references, or reach for `Rc`/`Arc` when shared ownership is genuinely needed.
- **Default to immutability.** Only add `mut` when mutation is required. Confine `&mut` borrows to the smallest scope possible.
- **No `unwrap()` in production code.** Use `?`, `if let`, `match`, or combinators (`.map()`, `.and_then()`, `.unwrap_or_default()`). Use `expect("reason")` only when you can prove the value is always `Some`/`Ok`.
- **Replace boolean parameters with enums** for self-documenting call sites.
- **Use the newtype pattern** for domain types (`Square`, `File`, `Rank`) instead of raw primitives.

### Naming (Rust API Guidelines)

- **Getters:** `fn name()` not `fn get_name()` (C-GETTER)
- **Conversions:** `as_` (free view), `to_` (may allocate), `into_` (consumes self) (C-CONV)
- **Error types:** verb-object-error order: `ParseMoveError`, not `MoveParseError` (C-WORD-ORDER)
- **Casing:** types `UpperCamelCase`, functions/variables `snake_case`, constants `SCREAMING_SNAKE_CASE`

### Error Handling

- Define specific error enums per module (`BoardError`, `ParseError`, `ReplError`) with `Display` and `Error` impls
- Use `?` propagation, not `unwrap()` chains
- Implement `From` for error conversion at module boundaries
- Use `TryFrom` instead of `From` when conversions can fail

### Clippy

- `#![warn(clippy::all)]` at crate level
- Cherry-pick pedantic lints: `needless_pass_by_value`, `explicit_iter_loop`, `implicit_clone`, `manual_string_new`, `semicolon_if_nothing_returned`
- Consider `clippy::indexing_slicing` with `.get()` or a `piece_at()` method as escape hatch for board access

### Testing

Run tests: `cargo test`

- Descriptive test names: `parses_capture_with_disambiguation` not `test_parse_1`
- Use `assert_eq!` / `assert_ne!` over bare `assert!` — they print both values on failure
- Return `Result` from tests to use `?` instead of `unwrap()`
- Test edge cases explicitly: empty input, boundary values, error conditions
- Unit tests in each module with `#[cfg(test)]`
- Integration tests in `tests/` directory

## File Structure

```
├── Cargo.toml
├── src/
│   ├── lib.rs         # Library exports
│   ├── main.rs        # CLI entry point
│   ├── wav.rs         # WAV format output
│   ├── synth.rs       # Waveform generators
│   ├── freq.rs        # Frequency mapping
│   ├── notation.rs    # Algebraic notation parser
│   └── board.rs       # Board representation
├── tests/
│   └── integration.rs # End-to-end tests
├── CLAUDE.md
└── ROADMAP.md
```

## Git Workflow

- Conventional commits: `feat:`, `fix:`, `refactor:`, `test:`
- Feature branches: `feature/board`, `feature/synth`

## Skills

Rust specialist maintaining production-grade Rust systems. Expertise in:
- Zero-dependency implementations
- Binary I/O and audio processing
- Performance optimization
- Idiomatic Rust patterns

Available project skills:
- `/address-pr-comments` - Fetch PR review comments, group by file/category, propose actionable code changes
