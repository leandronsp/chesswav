---
name: refactor
description: Baby-step refactoring of changed code and its boundaries. Called after GREEN phase in TDD. Use when: refactor, clean up, improve naming, extract function, simplify, reduce complexity, after green.
---

# Refactor — Baby Steps

**Refactor changed code and its immediate boundaries. Tests must stay green after every step.**

## When Called

This skill is invoked after the GREEN phase of `/tdd`. It operates only on:
- Code that was just changed or added
- Immediate boundaries (callers, callees, sibling functions in the same module)

It does NOT touch unrelated code.

## Process

### 1. Identify Scope

Look at the code changed since the last green state. List the functions, structs, and modules touched.

### 2. Check Against Conventions

Review changed code against `CLAUDE.md` conventions:

| Check | Convention |
|-------|-----------|
| **Naming** | No single-letter variables, favor meaning over brevity |
| **Magic expressions** | Extracted to named functions or variables |
| **Semantic methods** | `clear_square()` not `set(sq, None)` |
| **Function complexity** | Understandable at a glance, decomposed if not |
| **Consistency** | `self.` vs `Self::` used correctly |
| **Rust idioms** | No `unwrap()`, prefer borrowing, exhaustive match |
| **Comments** | *Why* not *what*, present where business logic isn't obvious |

### 3. Apply Changes — One at a Time

For each refactoring opportunity:

1. Make one small change
2. Run `cargo test` — **must pass**
3. If tests break, revert and try a different approach
4. Move to the next change

**Baby steps.** Never batch multiple refactors into one step.

### 4. Common Refactoring Moves

In order of priority:

1. **Rename** — variables, functions, parameters to favor meaning
2. **Extract function** — if a block does one identifiable thing
3. **Inline** — remove unnecessary indirection
4. **Replace magic values** — constants or named functions
5. **Simplify conditionals** — `if let`, `match`, combinators
6. **Remove duplication** — only if 3+ occurrences (don't abstract prematurely)

### 5. Boundary Check

After refactoring the changed code, glance at immediate callers and callees:
- Do signatures still make sense?
- Did naming improvements create inconsistency with adjacent code?
- Are there obvious improvements in the boundary that are directly related to this change?

Fix only what's directly connected. Don't cascade into unrelated refactoring.

## Iron Rules

1. **Tests must stay green** after every single step
2. **Baby steps only** — one refactor move per step
3. **Changed code and boundaries only** — don't touch unrelated code
4. **Revert if tests break** — don't fix forward, try a different approach
5. **No new behavior** — refactoring changes structure, not behavior
6. **Run `cargo clippy` at the end** — must pass with no warnings

## Output

After completing, briefly report what was changed:

```
Refactored:
- Renamed `c` to `color` in apply_move
- Extracted `clear_square()` from set(sq, None) pattern
- No changes needed in boundaries

All tests passing. Clippy clean.
```
