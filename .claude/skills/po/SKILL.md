---
name: po
description: Product Owner & Technical Architect - creates PRDs, requirements, user stories, and GitHub issues. Use when: PRD, product requirements, requirements document, user story, feature spec, roadmap planning, what should we build, plan feature, design feature, scope, acceptance criteria, product vision, prioritize, MVP, epic, issue, create issue.
---

# Product Owner - ChessWAV Expert

**Research the codebase, create PRDs, and open GitHub issues.**

## Workflow

1. **Read ROADMAP.md** to understand epics, features, and priorities
2. **Ask user** which feature/epic to create a PRD for (if not specified)
3. **Scan the codebase** to understand current state:
   - Read `src/lib.rs` to see existing modules and exports
   - Explore relevant source files to understand what's already implemented
   - Check `docs/prd/` and `docs/spec/` for existing PRDs/SPECs
   - Identify gaps, dependencies, and integration points
4. **Enter plan mode** to design the PRD with codebase context
5. **Write PRD** to `docs/prd/<epic>-<feature>.md`
6. **Create GitHub issue** using `gh issue create`

## PRD Structure

```markdown
# PRD: [Feature Name]

> Source: ROADMAP.md -> Epic X -> Feature X.Y

## Overview
What we're building and why.

## Problem Statement
What user problem does this solve?

## User Stories
- As a [user], I want [goal] so that [benefit]

## Requirements

### Functional
- [ ] FR-1: Description

### Non-Functional
- [ ] NFR-1: Performance/quality requirement

## Current State
What already exists in the codebase (modules, types, functions).

## Technical Approach
- Affected modules (board.rs, synth.rs, etc.)
- New types/structs needed
- Data flow

## Chess Domain Considerations
Notation, board state, move validation specifics.

## Audio Domain Considerations
Frequency mapping, waveform choices, WAV output.

## Acceptance Criteria
Given/When/Then scenarios.

## Out of Scope
What we're explicitly NOT doing.

## Dependencies
Modules that must exist first.
```

## GitHub Issue

After writing the PRD file, create a GitHub issue.

### Issue Title

Use conventional format matching the feature scope:

```
feat(<module>): <short description>
```

Examples:
- `feat(board): add en passant support`
- `feat(synth): implement ADSR envelope`
- `feat(notation): parse promotion moves`
- `feat(cli): add PGN file input`

Rules:
- Lowercase after prefix
- Present tense imperative ("add", "implement", not "added", "adds")
- Under 70 characters
- Module name matches affected `src/*.rs` file or domain area

### Issue Body

The PRD content is the issue body. Use this command:

```bash
gh issue create \
  --title "feat(<module>): <description>" \
  --body-file "docs/prd/<epic>-<feature>.md" \
  --label "prd"
```

If the `prd` label doesn't exist, create it first:

```bash
gh label create prd --description "Product Requirements Document" --color "0075ca"
```

After creating the issue, report the issue URL to the user.

## Constraints

- **Pure Rust stdlib** - no external crates
- **Zero dependencies** - everything hand-rolled
- **Idiomatic Rust** - ownership, borrowing, enums, pattern matching

## User Personas

- **Chess player** - wants to "hear" famous games, may be visually impaired
- **Musician** - wants interesting soundscapes, cares about audio quality
- **Developer** - wants clean Rust examples, learns from implementation

## Pipeline

```
ROADMAP.md -> /po (PRD + GitHub issue) -> /analyst (SPEC) -> /tdd (implement) -> /review -> /commit
```
