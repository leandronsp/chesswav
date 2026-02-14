---
name: pr
description: Open or update a draft PR for the current branch. Use when: create PR, open PR, draft PR, pull request, prepare for review.
---

# Draft PR Creator

Creates or updates a draft pull request for the current branch.

## Usage

- `/pr` - Create draft PR for current branch (or update if one exists)
- `/pr <url>` - Update existing PR description

## Workflow

### 1. Run tech debt audit

Invoke `/techdebt` to scan for issues. If there are **Critical** items, fix them first.

### 2. Gather context

```bash
git branch --show-current
git log main..HEAD --oneline
git diff main...HEAD --stat
```

### 3. Check if PR already exists

```bash
gh pr view --json number,title,body 2>/dev/null
```

### 4. Write PR description

Use the template below. Keep it concise and natural.

### 5a. If NO existing PR - Create draft PR

```bash
gh pr create --draft --title "<title>" --body "$(cat <<'EOF'
<body>
EOF
)"
```

### 5b. If PR already exists - Update description

```bash
gh pr edit --title "<title>" --body "$(cat <<'EOF'
<updated body>
EOF
)"
```

Report the PR URL to the user when done.

## PR Title Format

```
<type>: <short description>
```

Types: `feat:`, `fix:`, `refactor:`, `test:`, `chore:`, `docs:`

Rules:
- Lowercase after prefix
- Present tense imperative ("add" not "added")
- Under 70 characters

## PR Body Template

```markdown
## Summary

1-2 paragraphs explaining what this does and why.

## Changes

- Highlight 1
- Highlight 2
- Highlight 3

## Testing

- [ ] `cargo test` passes
- [ ] `cargo clippy -- -D warnings` passes

## Tech Debt

- [ ] `/techdebt` audit passed (no Critical items)
```

## Style

- **Fluid prose** in Summary - natural writing, not robotic
- **2-3 bullet points** in Changes - highlights only, not a file list
- **No file lists** - GitHub shows that in "Files changed"

## Pipeline

```
/tdd <issue_url> -> /review -> /pr -> merge
```
