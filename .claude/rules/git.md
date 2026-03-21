---
description: Git operations — committing, branching, staging
globs: ["**/*"]
alwaysApply: false
---

# Git Conventions

## Commits

Format: `<type>: <short description>`

Types: `feat:`, `fix:`, `refactor:`, `test:`, `chore:`, `docs:`

Rules:
- Present tense ("add" not "added"), lowercase after prefix
- Never mention AI/Claude in commits
- Never add Co-Authored-By trailers
- No emojis in commit messages

## Staging

- `git add <specific files>` — never `git add .` or `git add -A`
- Review staged diff before committing: `git diff --staged`

## Branches

- `feat/<name>` — new features
- `fix/<name>` — bug fixes
- `refactor/<name>` — refactoring
- `chore/<name>` — maintenance

## Pre-commit

```bash
cargo test            # all tests pass
cargo clippy -- -D warnings  # no lint warnings
```

## Examples

```bash
git add src/engine/board.rs src/engine/chess.rs
git commit -m "feat: add en passant support"

git add src/audio/synth.rs
git commit -m "fix: correct WAV header byte order"

git add src/engine/board.rs src/engine/hint.rs
git commit -m "refactor: extract disambiguation into hint module"
```
