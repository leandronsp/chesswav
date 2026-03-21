---
description: Rust testing and TDD conventions
globs: ["src/**/*.rs", "tests/**/*.rs"]
alwaysApply: false
---

# Testing Conventions

## Running Tests

```bash
cargo test                              # all tests
cargo test board::tests                 # single module
cargo test board::tests::parses_e4      # single test
cargo clippy -- -D warnings            # lint check
```

## TDD Cycle

1. **RED** — Write the test asserting correct behavior, run it, confirm it fails
2. **GREEN** — Write minimum code to make it pass
3. **REFACTOR** — Clean up while staying green
4. Repeat

### Tests drive code, never the reverse

- The test defines what correct behavior is — **never change a test to match a wrong implementation**
- If the implementation returns the wrong value but the test expects the right one, fix the implementation
- Every match arm, every error variant must have a test that fails without it

## Conventions

- Write tests BEFORE or alongside implementation, never after
- Test public API only — never test private functions
- One assertion focus per test (multiple asserts OK if one logical thing)
- Descriptive test names: `parses_capture_with_disambiguation` not `test_parse_1`
- `assert_eq!` / `assert_ne!` over bare `assert!` — they print both values on failure
- Return `Result` from tests to use `?` instead of `unwrap()`
- Unit tests in each module with `#[cfg(test)]`
- Integration tests in `tests/` directory

## Edge Cases to Test

- Empty input / invalid notation
- Boundary values (edge of board, rank 1, rank 8, file a, file h)
- All piece types and special moves (castling, en passant, promotion)
- Check and checkmate positions
- Audio: sample overflow, zero-length notes, frequency boundaries
