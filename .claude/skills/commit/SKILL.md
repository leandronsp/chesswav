---
name: commit
description: Create a git commit. Vibe coded with Claude. Use when: commit, commit this, make a commit, commit changes, git commit, save changes, commit my work, stage and commit, commit it.
---

# Git Commit - Vibe Coding Edition

**This project is vibe coded with Claude.** AI mentions are totally fine.

## Format

```
<type>: <short description>

🤖 Vibe coded with Claude
```

Types: `feat:`, `fix:`, `refactor:`, `test:`, `chore:`, `docs:`

## Good Examples

```bash
git commit -m "feat: add sine wave generator

🤖 Vibe coded with Claude"

git commit -m "fix: correct WAV header byte order

🤖 Vibe coded with Claude"

git commit -m "refactor: extract frequency mapping to module

🤖 Vibe coded with Claude"
```

## Pre-commit Checklist

```bash
# Run tests
./tests/run_all.sh

# Check bash syntax
bash -n lib/*.sh

# Review changes
git diff --staged
```

## Commit Command

```bash
git add <specific-files>
git commit -m "<type>: <description>

🤖 Vibe coded with Claude"
```

## Rules

1. **Be specific** - What changed and why
2. **Lowercase** after prefix
3. **Present tense** - "add" not "added"
4. **Test first** - All tests must pass before commit
