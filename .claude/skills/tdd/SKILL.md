---
name: tdd
description: Implementer - fetches a GitHub issue, builds an implementation plan, then implements using TDD. Use when: implement, build this, code this, add this feature, TDD, test first, red green refactor, pick a task, next task.
---

# Implementer - TDD Engineer

**Fetches a GitHub issue, plans the implementation, then builds it with strict TDD.**

## Usage

- `/tdd <issue_url>` - Fetch issue, plan, and implement
- `/tdd <issue_number>` - Same, using issue number (e.g. `/tdd 5`)
- `/tdd` - Ask user which issue to implement

## Workflow

### Phase 1: Understand

1. **Fetch the issue** using `gh issue view <number> --json title,body`
2. **Scan the codebase** to understand what exists:
   - Read `src/lib.rs` for module structure
   - Explore relevant source files
   - Understand existing types, traits, and patterns
3. **Identify the gap** between current state and what the issue requires

### Phase 2: Plan

4. **Enter plan mode** to design the implementation:
   - Break the issue into ordered implementation tasks (baby steps)
   - Each task = one testable behavior increment
   - Identify files to create/modify
   - Identify new types, functions, error types
   - Note dependencies between tasks
5. **Present the plan** to the user for approval

### Phase 3: Implement (TDD)

6. **Create a feature branch**: `feature/<short-name>`
7. **For each task**, follow the RED-GREEN-REFACTOR cycle below

## The Cycle

For each behavior increment. One test at a time. Baby steps.

### RED - Write One Failing Test

Write the smallest possible test for the next behavior:

```rust
#[test]
fn initializes_white_king_on_e1() -> Result<(), BoardError> {
    let board = Board::new();
    let piece = board.piece_at(Square::E1);
    assert_eq!(piece, Some(Piece::new(PieceKind::King, Color::White)));
    Ok(())
}
```

Run: `cargo test`

**The test MUST FAIL.** If it passes:
1. Re-examine the test — is it actually testing new behavior?
2. Adjust the assertion to target untested behavior
3. If it still passes after 3 attempts, **stop and ask the user**

### GREEN - Minimum Code to Pass

Write only enough production code to make the failing test pass. No more. No future-proofing.

Follow all codebase conventions from `CLAUDE.md`:
- Favor meaning in naming (no single-letter variables)
- Separation of concerns
- Idiomatic Rust

Run: `cargo test`

**The test MUST PASS.** If it fails:
1. Read the error carefully
2. Fix the implementation (not the test)
3. If it still fails after 5 attempts, **stop and ask the user**

### REFACTOR - Clean the Changed Code

Once green, invoke `/refactor` on the changed code and its immediate boundaries.

Run: `cargo test` after each refactor step — **must stay green**

### REPEAT

Go back to RED for the next behavior increment. Continue until the task is complete.

## Iron Rules

1. **No production code without a failing test** — ever
2. **Baby steps** — one test, one behavior, one increment
3. **Run tests after every change** — `cargo test`
4. **Refactor only when green** — never refactor red code
5. **One task at a time** — finish before starting the next
6. **Escalate, don't spin** — ask the user when stuck
7. **TDD is the default** — only skip if the user explicitly says so

## Implementation Checklist

For each task:

- [ ] Write first failing test
- [ ] Confirm test fails (RED)
- [ ] Write minimum code to pass
- [ ] Confirm test passes (GREEN)
- [ ] Run `/refactor` on changed code
- [ ] Confirm tests still pass
- [ ] `cargo clippy -- -D warnings`
- [ ] Repeat for next behavior increment
- [ ] All task behaviors covered
- [ ] Commit with `/commit`

## Pipeline

```
/po <prompt> -> GitHub issue -> /tdd <issue_url> -> /review -> /pr
```
