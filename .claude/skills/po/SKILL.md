---
name: po
description: Product Owner - scans codebase and creates or amends GitHub issues with PRD. Use when: PRD, feature, issue, create issue, amend issue, what should we build, plan feature, design feature, scope, requirements, user story.
---

# Product Owner - ChessWAV Expert

**Scan the codebase, write a PRD, and open a GitHub issue.**

## Usage

- `/po <prompt>` - Create a GitHub issue from the given feature description
- `/po <prompt> --priority <P0|P1|P2|P3>` - Create issue with explicit priority
- `/po` - Ask user what to build, then create the issue
- `/po amend <issue_number> <summary>` - Append a revision to an existing issue (called by `/tdd` when the implementer's research refines the PRD)

If no `--priority` is given, ask the user which priority to assign.

## Workflow

1. **Scan the codebase** to understand current state:
   - Read `src/lib.rs` to see existing modules and exports
   - Explore relevant source files for what's already implemented
   - Identify gaps, dependencies, and integration points
2. **Read ROADMAP.md** if the prompt maps to a roadmap item
3. **Write the PRD** as the issue body (no local file)
4. **Create GitHub issue** with conventional title and PRD body

## Issue Title

Conventional format:

```
feat(<module>): <short description>
```

Examples:
- `feat(board): add en passant support`
- `feat(synth): implement ADSR envelope`
- `feat(notation): parse promotion moves`
- `fix(repl): handle invalid move input gracefully`

Rules:
- Lowercase after prefix
- Present tense imperative ("add", not "added")
- Under 70 characters
- Module name matches `src/*.rs` or domain area

## Issue Body (PRD)

```markdown
## Overview
What we're building and why.

## Problem Statement
What user problem does this solve?

## User Stories
- As a [user], I want [goal] so that [benefit]

## Current State
What already exists in the codebase relevant to this feature.

## Requirements

### Functional
- [ ] FR-1: Description

### Non-Functional
- [ ] NFR-1: Performance/quality requirement

## Technical Approach
- Affected modules
- New types/structs needed
- Data flow

## Acceptance Criteria
Given/When/Then scenarios.

## Out of Scope
What we're explicitly NOT doing.
```

## Creating the Issue

```bash
gh issue create \
  --title "feat(<module>): <description>" \
  --body "$(cat <<'EOF'
<PRD content>
EOF
)" \
  --label "prd" \
  --label "P2: medium"
```

Replace `"P2: medium"` with the chosen priority label (`P0: critical`, `P1: high`, `P2: medium`, `P3: low`).

If the `prd` label doesn't exist, create it first:

```bash
gh label create prd --description "Product Requirements Document" --color "0075ca"
```

Report the issue URL to the user when done.

## Amending an Issue

When called with `/po amend <issue_number> <summary>`, the PO appends a revision to the existing issue. **The original PRD is immutable** — never edit or overwrite it.

### Amend Workflow

1. **Fetch the current issue** using `gh issue view <number> --json title,body`
2. **Read the summary** provided by the TDD engineer
3. **Research the codebase** if needed to validate the proposed changes
4. **Append a revision block** to the issue body:

```bash
gh issue edit <number> --body "$(cat <<'EOF'
<original body unchanged>

---

## Revision <N> — <date>

**Source:** TDD implementer research

### Changes
- <what changed and why>

### Updated Requirements
- [ ] FR-X: <new or modified requirement>

### Removed/Deferred
- ~~FR-Y: <requirement removed or moved to out of scope>~~
EOF
)"
```

### Amend Rules

- **Never modify the original PRD text** — append only
- **Increment revision number** (Revision 1, Revision 2, ...)
- **Each revision is self-contained** — reader can understand what changed without diffing
- **Link back to the source** — who requested the change and why
- The PO may push back on the amendment if it contradicts product goals — surface disagreement to the user

## Constraints

- **Pure Rust stdlib** - no external crates
- **Zero dependencies** - everything hand-rolled
- **Idiomatic Rust** - ownership, borrowing, enums, pattern matching

## Pipeline

```
/po <prompt> -> GitHub issue -> /tdd <issue_url> -> /review -> /pr
                     ^                |
                     |                | (PRD feedback)
                     +--- /po amend --+
```
