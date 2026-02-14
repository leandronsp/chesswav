---
name: tdd
description: Implementer - takes SPEC tasks and implements them using TDD. Red-green-refactor cycle. Use when: implement, implement this, build this, code this, create function, add this feature, develop, make it work, let's build, start coding, TDD, test first, red green refactor, pick a task, next task, implement task.
---

# Implementer - TDD Engineer

**Takes SPEC tasks and implements them using strict TDD.**

## Primary Workflow

When invoked, the Implementer should:

1. **Read the SPEC** from `docs/spec/` (ask user which one if not specified)
2. **List available tasks** from the SPEC's Implementation Tasks section
3. **Ask user** which task to implement (or suggest next based on dependencies)
4. **Create a user story** from the task
5. **Implement using TDD** - red-green-refactor cycle

## SPEC → Story → Code

```
SPEC Task                    User Story                     Implementation
─────────────────────────────────────────────────────────────────────────────
Task 2: Implement      →     "As a developer, I need       →  1. Write failing test
board_init                    board_init() so the board        2. Implement minimum
Est: S                        starts with pieces in            3. Refactor
Test: check e1=K              standard position"               4. Commit
```

## Story Format

Before implementing each task, create a story:

```markdown
## Story: [Task Name]

**From**: docs/spec/<spec>.md → Task N

**As a** [developer/system/user]
**I need** [function/feature]
**So that** [value/purpose]

**Acceptance Criteria**:
- [ ] AC1: Specific testable condition
- [ ] AC2: Another condition

**Test Cases**:
```bash
test_case_1() { ... }
test_case_2() { ... }
```
```

## Iron Rules of TDD

1. **NO production code without a failing test**
2. **Baby steps** - smallest possible increments
3. **Run tests constantly** - after every change
4. **Refactor only when green**
5. **One task at a time** - finish before starting next

## The Cycle

### 🔴 RED - Write Failing Test First

```bash
# tests/board_test.sh

test_board_init_places_white_king() {
    board_init
    result=$(board_get "e1")
    assert_equals "K" "$result" "White king should be on e1"
}
```

Run it - **MUST FAIL**:
```bash
./tests/run_all.sh
# FAIL: White king should be on e1 (expected: K, got: )
```

### 🟢 GREEN - Minimum Code to Pass

```bash
# lib/board.sh

board_init() {
    declare -gA BOARD
    BOARD["e1"]="K"
}

board_get() {
    echo "${BOARD[$1]}"
}
```

Run it - **MUST PASS**:
```bash
./tests/run_all.sh
# PASS: test_board_init_places_white_king
```

### 🔵 REFACTOR - Clean Up (Stay Green)

```bash
board_init() {
    declare -gA BOARD

    # White pieces - back rank
    BOARD["a1"]="R"; BOARD["b1"]="N"; BOARD["c1"]="B"; BOARD["d1"]="Q"
    BOARD["e1"]="K"; BOARD["f1"]="B"; BOARD["g1"]="N"; BOARD["h1"]="R"
    # ... continue
}
```

Run it - **MUST STILL PASS**

## Task Selection

When picking tasks from SPEC:

1. **Check dependencies** - blocked tasks can't start
2. **Check current progress** - what's already done?
3. **Suggest next** - smallest unblocked task

```
Available Tasks:
  ✓ Task 1: Create board.sh skeleton (done)
  → Task 2: Implement board_init (ready - no blockers)
  ○ Task 3: Implement board_get (blocked by Task 2)
  ○ Task 4: Implement board_set (blocked by Task 2)

Suggest: Task 2
```

## Implementation Checklist

For each task:

### 1. Prepare
- [ ] Read task details from SPEC
- [ ] Write story with acceptance criteria

### 2. TDD Cycle
- [ ] 🔴 Write first failing test
- [ ] 🔴 Confirm test fails
- [ ] 🟢 Write minimum code to pass
- [ ] 🟢 Confirm test passes
- [ ] 🔵 Refactor if needed
- [ ] 🔵 Confirm tests still pass
- [ ] Repeat for next test case

### 3. Security Review
- [ ] All variables quoted
- [ ] Input validated (regex allowlist)
- [ ] No eval with external data
- [ ] Safe arithmetic
- [ ] Error handling in place

### 4. Finalize
- [ ] Run full test suite
- [ ] Run `bash -n` syntax check
- [ ] Commit with `/commit`
- [ ] Mark task complete
- [ ] Pick next task

## Test Framework

```bash
# tests/framework.sh

TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

assert_equals() {
    local expected="$1"
    local actual="$2"
    local msg="${3:-}"

    ((TESTS_RUN++))
    if [[ "$expected" == "$actual" ]]; then
        ((TESTS_PASSED++))
        echo "✓ $msg"
    else
        ((TESTS_FAILED++))
        echo "✗ $msg"
        echo "  expected: $expected"
        echo "  actual:   $actual"
    fi
}

assert_true() {
    local condition="$1"
    local msg="${2:-}"

    ((TESTS_RUN++))
    if eval "$condition"; then
        ((TESTS_PASSED++))
        echo "✓ $msg"
    else
        ((TESTS_FAILED++))
        echo "✗ $msg"
    fi
}

run_tests() {
    for test_fn in $(declare -F | awk '/test_/ {print $3}'); do
        $test_fn
    done

    echo ""
    echo "Tests: $TESTS_RUN | Passed: $TESTS_PASSED | Failed: $TESTS_FAILED"
    [[ $TESTS_FAILED -eq 0 ]]
}
```

## Baby Steps Example

**BAD - Too big:**
```bash
test_parse_full_game() {
    # Parse entire PGN, validate all moves, generate audio
    # 50 lines of test code
}
```

**GOOD - Baby steps:**
```bash
test_parse_pawn_move() {
    result=$(parse_move "e4")
    assert_equals "P" "${result[piece]}"
}

test_parse_knight_move() {
    result=$(parse_move "Nf3")
    assert_equals "N" "${result[piece]}"
}

test_parse_capture() {
    result=$(parse_move "Bxc6")
    assert_equals "1" "${result[capture]}"
}
```

## Integration with Other Skills

```
/po      → Creates PRD from ROADMAP
/analyst → Creates SPEC from PRD (with tasks)
/tdd     → Implements tasks from SPEC ← YOU ARE HERE
/security→ Security review before commit
/commit  → Commits completed work
/review  → Reviews implementation
```

### Security Checks (from /security)

Before committing, verify:

```bash
# Input validation
[[ "$input" =~ ^[a-h][1-8]$ ]] || return 1

# No eval with user data
# All variables quoted
# Safe arithmetic (no injection)
# Temp files with mktemp + trap cleanup
```

### Commit Workflow (from /commit)

After task passes all tests:

```bash
# Pre-commit checks
./tests/run_all.sh           # All tests pass
bash -n lib/*.sh             # Syntax check

# Commit
git add <specific-files>
git commit -m "feat: implement board_init

🤖 Vibe coded with Claude"
```

## Clean Code During Refactor

- Functions < 20 lines
- Single responsibility
- Descriptive names (no abbreviations)
- No magic numbers - use constants
- `local` for all function variables
- Quote all variables
- Follow patterns from SPEC

## Workflow Commands

```bash
# Before starting
cat docs/spec/<feature>.md      # Read the SPEC

# During implementation
./tests/run_all.sh              # Run all tests
bash -n lib/<module>.sh         # Syntax check

# After completing task
git status                      # Check changes
/commit                         # Commit work
```
