---
name: review
description: Deep code review - Rust idioms, audio correctness, chess logic, safety. Launches 3 parallel reviewers + plan-reviewer gate. Use when: review, review this, code review, check this code, review my changes, is this good, what do you think.
---

# Code Review — Parallel Agents + Plan-Reviewer Gate

**Reviews current changes for correctness, idioms, safety, audio math, and chess logic. Three parallel code-reviewer agents with focused scopes, aggregated findings, and a plan-reviewer critique before presenting fixes.**

## Workflow

### Phase 1: Diff

Get the full diff and diff stat against main:

```bash
git diff main...HEAD
git diff main...HEAD --stat
```

### Phase 2: Parallel Review

Launch **3 `code-reviewer` agents in parallel** (single message, 3 Agent tool calls). Each gets the full diff but a focused mandate.

**CRITICAL**: Launch all 3 agents in the **same message** so they run concurrently. Do NOT launch them sequentially.

#### Agent 1: Correctness & Safety

```
subagent_type: code-reviewer
prompt: |
  You are reviewing a Rust codebase that transforms chess games into audio. Focus ONLY on correctness and safety.

  ## Your scope

  ### Correctness
  - Logic errors, wrong math (frequencies, sample calculations, byte order)
  - Chess logic bugs (illegal moves, wrong disambiguation, broken check/checkmate)
  - WAV format errors (RIFF header, chunk sizes, PCM encoding)
  - Edge cases: empty input, boundary values, invalid notation

  ### Safety
  - `unwrap()` in production code — must use `?`, `if let`, `match`
  - Integer overflow — use `checked_*` or `saturating_*`
  - Panics on invalid input — return `Result` instead
  - `unsafe` without justification
  - Input validation at system boundaries (CLI args, PGN parsing)

  ## Output format

  Return findings as a markdown list, grouped by severity:

  ### Critical
  1) **Issue**: description
     **Location**: `file:line`
     **Fix**: solution

  ### Important
  A) **Issue**: description
     **Location**: `file:line`
     **Suggestion**: approach

  ### Minor
  * Nitpick or suggestion

  ### Positive
  - What's done well

  If no findings in a tier, omit that section. Be specific — cite file:line for every finding.

  ## Diff to review

  <diff>
  {PASTE FULL DIFF HERE}
  </diff>
```

#### Agent 2: Idioms & Architecture

```
subagent_type: code-reviewer
prompt: |
  You are reviewing a Rust codebase that transforms chess games into audio. Focus ONLY on idioms and architecture.

  ## Your scope

  ### Rust idioms
  - `.clone()` to work around borrow checker — restructure instead
  - `mut` when immutability would work — default to immutable
  - Wildcard `_ =>` on own enums — must list all variants
  - Boolean parameters — use enums for self-documenting call sites
  - Getter naming: `fn name()` not `fn get_name()`
  - Conversion naming: `as_` (free), `to_` (may allocate), `into_` (consumes)
  - Single-letter vars (except iterators) — descriptive names
  - Magic numbers — extract to named constants

  ### Architecture
  - God modules >300 lines — extract submodules
  - DRY violations (same pattern 3+ times) — extract helper
  - REPL/CLI doing engine work — should delegate to engine
  - `main.rs` doing too much — should be thin
  - Separation of concerns: engine owns chess, audio owns synthesis
  - Unnecessary abstractions (single-use wrappers)

  ## Output format

  Return findings as a markdown list, grouped by severity:

  ### Critical
  1) **Issue**: description
     **Location**: `file:line`
     **Fix**: solution

  ### Important
  A) **Issue**: description
     **Location**: `file:line`
     **Suggestion**: approach

  ### Minor
  * Nitpick or suggestion

  ### Positive
  - What's done well

  If no findings in a tier, omit that section. Be specific — cite file:line for every finding.

  ## Diff to review

  <diff>
  {PASTE FULL DIFF HERE}
  </diff>
```

#### Agent 3: Completeness & Contracts

```
subagent_type: code-reviewer
prompt: |
  You are reviewing a Rust codebase that transforms chess games into audio. Focus ONLY on completeness and contracts.

  ## Your scope

  ### Documentation sync
  - Does `README.md` "Project Structure" match the actual `src/` layout?
  - Does `CLAUDE.md` "File Structure" match the actual `src/` layout?
  - Are `//!` doc comments in `main.rs` and `lib.rs` up to date?
  - Are `///` doc comments on changed public types/functions accurate?
  - New/removed/renamed modules reflected in docs?

  ### Error handling contracts
  - Specific error enums per module (`BoardError`, `ParseError`, `AudioError`)
  - `Display` and `Error` impls on error types
  - `From` for error conversion at module boundaries
  - `?` propagation, not `unwrap()` chains

  ### Test coverage
  - New public behavior has corresponding tests
  - Descriptive test names: `parses_capture_with_disambiguation` not `test_1`
  - `assert_eq!` / `assert_ne!` over bare `assert!`
  - Return `Result` from tests to use `?`
  - Edge cases tested (empty input, boundaries, special moves)
  - `#[cfg(test)]` module in each file

  ### Comments
  - Non-obvious logic has WHY comments (not WHAT)
  - No commenting obvious/self-documenting code
  - Domain context comments (chess rules that inform implementation)

  ## Output format

  Return findings as a markdown list, grouped by severity:

  ### Critical
  1) **Issue**: description
     **Location**: `file:line`
     **Fix**: solution

  ### Important
  A) **Issue**: description
     **Location**: `file:line`
     **Suggestion**: approach

  ### Minor
  * Nitpick or suggestion

  ### Positive
  - What's done well

  If no findings in a tier, omit that section. Be specific — cite file:line for every finding.

  ## Diff to review

  <diff>
  {PASTE FULL DIFF HERE}
  </diff>
```

### Phase 3: Aggregate

After all 3 agents return, merge their findings:

1. **Merge** all findings into unified tiers (Critical / Important / Minor / Positive)
2. **Deduplicate** same file:line across agents
3. **Tag** each finding with source: `[safety]`, `[idioms]`, `[completeness]`
4. **Cap**: max 5 Critical, 7 Important, 3 Minor (drop lowest-impact excess)
5. **Verdict**: any Critical or Important -> "Needs fixes"; only Minor/Positive -> "Clean with suggestions"

### Phase 4: Plan + Critique

**If Critical or Important findings exist:**

1. Build a numbered fix plan:
   ```
   1. [severity] file:line — proposed fix
   2. [severity] file:line — proposed fix
   ...
   ```

2. Launch a **`plan-reviewer` agent** with the fix plan and diff stat:
   ```
   subagent_type: plan-reviewer
   prompt: |
     Review this fix plan for a Rust codebase.

     ## Diff stat
     {PASTE DIFF STAT}

     ## Fix plan
     {PASTE NUMBERED FIX PLAN}

     Critique the plan:
     - Are any fixes over-engineered for the actual problem?
     - Are there gaps — issues in the diff that the plan misses?
     - Are any fixes redundant or conflicting?
     - Would any fix break existing behavior?
     - Is the priority ordering correct?

     Return:
     1. Fixes to DROP (over-engineered or unnecessary) with reasoning
     2. Fixes to ADD (gaps the plan missed) with file:line and description
     3. Fixes to MODIFY (scope adjustment) with reasoning
     4. Overall assessment: "Plan is solid" or "Plan needs adjustment"
   ```

3. **Incorporate critique**: drop over-engineered fixes, add missed gaps, adjust scope
4. Present the **critique-adjusted plan** to the user

**If only Minor findings**: skip plan-reviewer, present review directly.

### Phase 5: Present

Show the aggregated review to the user:

```markdown
## Code Review — {branch name}

**Diff**: {files changed}, {insertions}+, {deletions}-

### Critical
1) [safety] **Issue**: description
   **Location**: `file:line`
   **Fix**: solution

### Important
A) [idioms] **Issue**: description
   **Location**: `file:line`
   **Suggestion**: approach

### Minor
* [completeness] Nitpick or suggestion

### Positive
- What's done well

### Verdict
[ ] Clean - ready for `/pr`
[x] Needs fixes - see plan below

---

## Fix Plan (critique-adjusted)

1. [Critical] `file:line` — fix description
2. [Important] `file:line` — fix description
...

*Plan reviewed by plan-reviewer. Dropped N over-engineered fixes, added M gaps.*
```

### Phase 6: Implement

After user approves the plan:

1. **Enter plan mode** with the fix plan
2. Implement fixes in plan order
3. Run verification:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   ```
4. Report results

## Pipeline

```
/po -> GitHub issue -> /dev -> /review -> /commit -> /pr
```
