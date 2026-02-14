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
4. **Implement using TDD** - red-green-refactor cycle

## The Cycle

### RED - Write Failing Test First

```rust
#[test]
fn initializes_white_king_on_e1() -> Result<(), BoardError> {
    let board = Board::new();
    let piece = board.piece_at(Square::E1);
    assert_eq!(piece, Some(Piece::new(PieceKind::King, Color::White)));
    Ok(())
}
```

Run: `cargo test` - **MUST FAIL**

### GREEN - Minimum Code to Pass

Write only enough production code to make the failing test pass. No more.

Run: `cargo test` - **MUST PASS**

### REFACTOR - Clean Up (Stay Green)

Improve naming, extract functions, remove duplication. Tests must stay green.

Run: `cargo test` - **MUST STILL PASS**

## Iron Rules

1. **No production code without a failing test**
2. **Baby steps** - smallest possible increments
3. **Run tests after every change** - `cargo test`
4. **Refactor only when green**
5. **One task at a time**

## Task Selection

When picking tasks from SPEC:

1. **Check dependencies** - blocked tasks can't start
2. **Check current progress** - what's already implemented?
3. **Suggest next** - smallest unblocked task

## Implementation Checklist

For each task:

- [ ] Write first failing test
- [ ] Confirm test fails
- [ ] Write minimum code to pass
- [ ] Confirm test passes
- [ ] Refactor if needed
- [ ] Confirm tests still pass
- [ ] `cargo clippy -- -D warnings`
- [ ] Commit with `/commit`

## Integration with Pipeline

```
/po      -> PRD from ROADMAP
/analyst -> SPEC from PRD (with tasks)
/tdd     -> Implements tasks from SPEC  <-- YOU ARE HERE
/review  -> Quality gate
/commit  -> Commits completed work
```
