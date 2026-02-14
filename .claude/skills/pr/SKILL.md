---
name: pr
description: Prepare and open a draft PR for the current branch. Use when asked to create a PR, open a PR, or prepare changes for pull request.
---

# Draft PR Creator

Creates or updates a draft pull request for the current branch.

## Usage

- `/pr` - Create draft PR for current branch (or update if one already exists)
- `/pr <url>` - Update existing PR description

## PR Title Format

```
<type>: <short description>
```

Types: `feat:`, `fix:`, `refactor:`, `test:`, `chore:`, `docs:`

Examples:
- `feat: add ADSR envelope to synth module`
- `fix: correct WAV header byte order`
- `refactor: extract frequency mapping to module`

Rules:
- Lowercase after prefix
- Present tense imperative ("add" not "added")
- Under 70 characters

## Workflow

### 1. Run tech debt audit

Before creating the PR, invoke `/techdebt` to scan for issues. If there are **Critical** items, fix them first before proceeding.

### 2. Gather context

```bash
git branch --show-current
git log main..HEAD --oneline
git diff main...HEAD --stat
```

### 3. Write PR description

Use the template below. Keep it concise.

### 4. Check if PR already exists

```bash
gh pr view --json number,title,body 2>/dev/null
```

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
- [ ] Manual testing done (if applicable)

## Tech Debt

- [ ] `/techdebt` audit passed (no Critical items)
```

## Style

- **Fluid prose** in Summary - natural writing, not robotic
- **2-3 bullet points** in Changes - highlights only, not a file list
- **No file lists** - GitHub handles that in "Files changed"

## Examples

### Small Fix
```markdown
## Summary

Fix frequency calculation that was producing wrong notes for squares on the h-file. The octave offset was off by one.

## Changes

- Fix off-by-one in `freq::square_to_frequency` for h-file squares
- Add regression test for boundary squares
```

### Feature
```markdown
## Summary

Add ADSR envelope to the synth module, giving notes natural attack and decay dynamics instead of abrupt on/off. Each phase (attack, decay, sustain, release) is configurable per piece timbre.

## Changes

- Add `Envelope` struct with ADSR parameters
- Integrate envelope into `synth::generate_samples`
- Wire piece timbres to envelope presets
```
