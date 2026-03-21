---
description: Rust patterns, idioms, and anti-patterns
globs: ["src/**/*.rs", "tests/**/*.rs"]
alwaysApply: false
---

# Rust Patterns

## Ownership & Borrowing

- **Prefer borrowing over cloning** — `.clone()` to silence the borrow checker is an anti-pattern
- **Default to immutability** — only `mut` when mutation is required
- **Confine `&mut` borrows** to the smallest scope possible
- **Use `Rc`/`Arc`** only when shared ownership is genuinely needed

## Error Handling

- **No `unwrap()` in production code** — use `?`, `if let`, `match`, or combinators
- **`expect("reason")`** only when you can prove the value is always `Some`/`Ok`
- **Specific error enums per module** — `BoardError`, `ParseError`, `AudioError`
- **`Display` and `Error` impls** on all error types
- **`From` for error conversion** at module boundaries
- **`TryFrom` instead of `From`** when conversions can fail

## Pattern Matching

- **Exhaustive match on own enums** — no wildcard `_ =>` catch-all
- **`if let`** for single-variant matching
- **`match`** for multi-variant dispatch

## Naming (Rust API Guidelines)

- **Getters:** `fn name()` not `fn get_name()`
- **Conversions:** `as_` (free view), `to_` (may allocate), `into_` (consumes self)
- **Error types:** verb-object-error order: `ParseMoveError`, not `MoveParseError`
- **Casing:** types `UpperCamelCase`, functions/variables `snake_case`, constants `SCREAMING_SNAKE_CASE`
- **Descriptive variable names** — no single-letter vars except iterators
- **Domain-driven naming** — types over primitives (`Square`, `File`, `Rank`)

## Semantic Methods

```rust
// BAD — raw None literal for domain operations
self.set(square, None);

// GOOD — semantic method
self.clear_square(square);

// BAD — raw indexing with implicit order
self.squares[rank][file]

// GOOD — method that makes the contract explicit
self.piece_at(rank, file)
```

## Comments

- Comment non-obvious logic, gotchas, and domain context (chess rules)
- Don't comment self-documenting functions
- Explain WHY, not WHAT
- Extract magic expressions into named functions or variables

## Architecture

- Thin CLI (`main.rs`), domain logic in `engine/` and `audio/` modules
- Separation of concerns: REPL delegates to engine, never resolves moves itself
- Functions decomposed if not understandable at a glance
- Modules <300 lines — extract submodules when growing

## Anti-Patterns

- `.clone()` to work around borrow checker
- `unwrap()` in production code
- Wildcard `_ =>` on own enums
- Boolean parameters — use enums for self-documenting call sites
- God modules >300 lines
- Magic numbers without named constants
- Defensive guards on internal code
- Commenting obvious code
