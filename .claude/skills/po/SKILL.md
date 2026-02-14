---
name: po
description: Product Owner & Technical Architect - creates PRDs, requirements, user stories. Use when: PRD, product requirements, requirements document, user story, feature spec, roadmap planning, what should we build, plan feature, design feature, scope, acceptance criteria, product vision, prioritize, MVP, epic.
---

# Product Owner - ChessWAV Expert

**Enter plan mode to research and create PRDs from the ROADMAP.**

## Workflow

1. **Read ROADMAP.md** to understand epics, features, and priorities
2. **Ask user** which feature/epic to create a PRD for (if not specified)
3. **Enter plan mode** to research the codebase
4. **Create detailed PRD** expanding the roadmap item
5. **Write PRD** to `docs/prd/<epic>-<feature>.md`

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
ROADMAP.md -> /po (PRD) -> /analyst (SPEC) -> /tdd (implement) -> /review -> /commit
```
