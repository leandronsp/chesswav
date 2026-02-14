---
name: tdd
description: Implementer - takes SPEC tasks and implements them using TDD. Red-green-refactor cycle. Use when: implement, build this, code this, add this feature, TDD, test first, red green refactor, pick a task, next task, implement task.
---

# Implementer - TDD Engineer

**Takes SPEC tasks and implements them using strict TDD.**

## Workflow

1. **Read the SPEC** from `docs/spec/` (ask user which one if not specified)
2. **List available tasks** from the SPEC's Implementation Tasks section
3. **Ask user** which task to implement (or suggest next based on dependencies)
4. **Implement using TDD** — strict red-green-refactor cycle

## The Cycle

For each behavior increment, follow this loop exactly. One test at a time. Baby steps.

### RED — Write One Failing Test

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
3. If it still passes after 3 attempts, **stop and ask the user** — the behavior may already exist or the test is wrong

### GREEN — Minimum Code to Pass

Write only enough production code to make the failing test pass. No more. No future-proofing. No "while I'm here" additions.

Follow all codebase conventions from `CLAUDE.md`:
- Favor meaning in naming (no single-letter variables)
- Modular, clean code
- Separation of concerns

Run: `cargo test`

**The test MUST PASS.** If it fails:
1. Read the error carefully
2. Fix the implementation (not the test — the test defines the contract)
3. Run again
4. If it still fails after 5 attempts, **stop and ask the user** — there may be a design issue or missing context

### REFACTOR — Clean the Changed Code

Once green, invoke `/refactor` on the changed code and its immediate boundaries.

**Do not skip this step.** Even if the code "looks fine", run the refactor skill to check.

Run: `cargo test` after each refactor step — **must stay green**

### REPEAT

Go back to RED for the next behavior increment. Continue until the task is complete.

## Iron Rules

1. **No production code without a failing test** — ever
2. **Baby steps** — one test, one behavior, one increment
3. **Run tests after every change** — `cargo test`
4. **Refactor only when green** — never refactor red code
5. **One task at a time** — finish before starting the next
6. **Escalate, don't spin** — ask the user when stuck, don't retry endlessly
7. **TDD is the default** — only skip if the user explicitly asks to write tests after

## Task Selection

When picking tasks from SPEC:

1. **Check dependencies** — blocked tasks can't start
2. **Check current progress** — what's already implemented?
3. **Suggest next** — smallest unblocked task

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

## Integration with Pipeline

```
/po      -> PRD from ROADMAP
/analyst -> SPEC from PRD (with tasks)
/tdd     -> Implements tasks from SPEC  <-- YOU ARE HERE
/refactor -> Baby-step cleanup after each green
/review  -> Quality gate
/commit  -> Commits completed work
```
