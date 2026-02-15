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
2. **Research the codebase independently** — don't assume the PRD covers everything:
   - Read `src/lib.rs` for module structure
   - Explore all source files relevant to the issue (read them, don't skim)
   - Understand existing types, traits, patterns, and conventions in use
   - Trace call paths and data flow through the affected areas
   - Look for edge cases, constraints, or existing behavior the PRD may not mention
   - Check tests for implicit contracts and expected behavior
3. **Identify the gap** between current state and what the issue requires
4. **Challenge the PRD** if research reveals:
   - Missing requirements or overlooked edge cases
   - A better technical approach than what was suggested
   - Existing code that already partially solves the problem
   - Constraints or dependencies the PRD didn't account for
   - Requirements that are infeasible, overly complex, or conflict with existing code
   - Scope that should be split into separate issues

### Phase 1.5: PRD Feedback (when needed)

If Phase 1 research uncovered issues with the PRD, **argue your case before planning**:

5. **Present findings to the user** — explain what you found and what you'd change
6. **If user agrees**, invoke `/po amend <issue_number>` with a summary of the changes:
   - The PO will append a revision to the issue (original PRD stays intact)
   - Wait for the amended issue before proceeding to Phase 2
7. **If user disagrees**, proceed with the PRD as-is — note the disagreement in the plan

This is not a formality. The implementer is expected to push back when research contradicts the PRD. Better to fix the spec than to build the wrong thing.

### Phase 2: Plan

8. **Enter plan mode** to design the implementation:
   - Summarize research findings — what exists, what's missing, any PRD amendments
   - Break the issue into ordered implementation tasks (baby steps)
   - Each task = one testable behavior increment
   - Identify files to create/modify
   - Identify new types, functions, error types
   - Note dependencies between tasks
9. **Present the plan** to the user for approval

### Phase 3: Implement (TDD)

10. **Create a feature branch**: `feature/<short-name>`
11. **For each task**, follow the RED-GREEN-REFACTOR cycle below

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
- [ ] Update all affected documentation (`README.md`, `CLAUDE.md`, doc comments) to reflect changes
- [ ] Commit with `/commit`

## Pipeline

```
/po <prompt> -> GitHub issue -> /tdd <issue_url> -> /review -> /pr
                     ^                |
                     |                | (PRD feedback)
                     +--- /po amend --+
```
