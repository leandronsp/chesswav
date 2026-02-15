# ChessWAV Project Guidelines

## First Principle: Less Code

No code is better than code. Every line must justify its existence. Before writing, ask: is this really needed? Is it clean, modular, minimal? When hitting too many lines, delete. Prefer deleting code over adding it. A smaller codebase is always better than a larger one.

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
- Modules per bounded context: `engine/`, `audio/`, `tui/`
- Unit tests in each module with `#[cfg(test)]`
- Self-documenting function names
- `cargo clippy` must pass with no warnings

## TDD — Mandatory

**All code changes (features, bug fixes, refactoring) MUST follow TDD unless the user explicitly asks to write tests after.**

This is not optional. Agents must use the TDD approach by default:

1. **RED** — Write one failing test. Run `cargo test`. It **must fail**. If it passes after multiple attempts to make it fail, ask the user for intervention.
2. **GREEN** — Write the minimum code to make the test pass. Run `cargo test`. It **must pass**. If it fails after multiple fix attempts, ask the user for intervention.
3. **REFACTOR** — Invoke `/refactor` on the changed code and its boundaries. Baby steps only. Tests must stay green after each refactor step.

**Baby steps**: one test at a time, one small change at a time. Never write multiple tests before making them pass. Never write large chunks of production code without a failing test driving it.

**Loop**: repeat RED → GREEN → REFACTOR for each behavior increment until the task is complete.

**Before writing any code, always read the `/tdd` skill** (`.claude/skills/tdd/SKILL.md`) for the full process, retry thresholds, and escalation rules.

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

### Documentation Hygiene

When making changes, **update all affected documentation**:
- `README.md` — project structure, features, usage, CLI flags
- `CLAUDE.md` — file structure, module list, conventions
- `//!` doc comments in `main.rs` and `lib.rs` — usage examples, module overview
- Inline `///` doc comments on public types and functions that changed behavior

Documentation must stay in sync with the code. Stale docs are worse than no docs.

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
│   ├── lib.rs               # Library exports
│   ├── main.rs              # CLI entry point
│   ├── engine/
│   │   ├── mod.rs           # Engine module exports
│   │   ├── chess.rs         # Domain types (Piece, Square, Move, parser)
│   │   ├── board.rs         # Board representation & move execution
│   │   └── hint.rs          # Move disambiguation hints
│   ├── audio/
│   │   ├── mod.rs           # Audio module exports
│   │   ├── freq.rs          # Square to frequency mapping
│   │   ├── synth.rs         # Note synthesis & orchestration
│   │   ├── wav.rs           # WAV file encoder
│   │   ├── waveform.rs      # Waveform generators (sine, triangle, square, saw)
│   │   └── blend.rs         # Waveform blending for composite timbres
│   └── tui/
│       ├── mod.rs           # TUI module exports
│       ├── repl.rs          # Interactive REPL
│       └── display/
│           ├── mod.rs       # Display mode abstraction
│           ├── sprite.rs    # Half-block pixel art renderer
│           ├── unicode.rs   # Unicode chess symbol renderer
│           ├── ascii.rs     # Plain text renderer
│           └── colors.rs    # ANSI color support (truecolor/256)
├── tests/
│   └── integration.rs       # End-to-end tests
├── CLAUDE.md
└── ROADMAP.md
```

## Git Workflow

- Conventional commits: `feat:`, `fix:`, `refactor:`, `test:`
- Feature branches: `feature/board`, `feature/synth`

## Skills

### Development Pipeline

Skills form a pipeline from idea to shipped code, with a feedback loop between implementer and PO:

```
/po <prompt> -> GitHub issue -> /tdd <issue_url> -> /review -> /pr
                     ^                |
                     |                | (PRD feedback)
                     +--- /po amend --+
```

| Skill | Purpose | When to Use |
|-------|---------|-------------|
| `/po` | Scan codebase, create/amend GitHub issue with PRD | Starting a new feature, defining requirements, amending PRD after implementer feedback |
| `/tdd` | Fetch issue, research, plan, implement with red-green-refactor | Building features from GitHub issues |
| `/refactor` | Baby-step cleanup after green | Called by `/tdd` after each GREEN phase |
| `/review` | Code review + techdebt audit, plan fixes | After implementation, before PR |
| `/techdebt` | Audit diff for pattern deviations and code smells | Called by `/review` and `/pr` |
| `/pr` | Open/update draft PR on GitHub | After review passes, before merge |
| `/commit` | Git commit following conventions | After tests pass |

### Standalone Skills

| Skill | Purpose | When to Use |
|-------|---------|-------------|
| `/analyst` | Create SPEC from PRD (detailed task breakdown) | When you need a formal spec document |
| `/bug-finder` | Edge case hunting, stress testing | Something feels fragile, need to break it |
| `/synth` | Audio synthesis reference (WAV, waveforms, ADSR) | Working on audio modules |
| `/address-pr-comments` | Fetch and address PR review comments | After receiving PR feedback |
