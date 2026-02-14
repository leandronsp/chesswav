---
name: commit
description: Create a git commit following project conventions. Use when: commit, commit this, make a commit, commit changes, git commit, save changes, commit my work, stage and commit.
---

# Git Commit

## Format

```
<type>: <short description>
```

Types: `feat:`, `fix:`, `refactor:`, `test:`, `chore:`, `docs:`

## Rules

1. **Concise** - short message, present tense ("add" not "added")
2. **Lowercase** after prefix
3. **No AI mentions** - never reference Claude, AI, or assistants
4. **No emojis** in commit messages
5. **Specific files** - `git add <files>`, never `git add .`

## Pre-commit Checklist

```bash
cargo clippy -- -D warnings
cargo test
git diff --staged
```

## Examples

```bash
git commit -m "feat: add sine wave generator"
git commit -m "fix: correct WAV header byte order"
git commit -m "refactor: extract frequency mapping to module"
```
