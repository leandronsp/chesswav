# Address PR Comments

Review and address pull request comments systematically.

## Usage

`/address-pr-comments [PR_NUMBER]`

If no PR number is provided, detect from the current branch via `gh pr view --json number -q .number`.

## Instructions

### 1. Resolve Repository and PR

```bash
# Detect owner/repo
gh repo view --json owner,name -q '(.owner.login + "/" + .name)'

# Detect PR number from current branch (if not provided)
gh pr view --json number -q .number
```

### 2. Fetch Comments

```bash
# Inline review comments (attached to code lines)
gh api repos/{owner}/{repo}/pulls/{pr_number}/comments

# General PR conversation comments
gh api repos/{owner}/{repo}/issues/{pr_number}/comments

# Review summaries
gh pr view {pr_number} --json reviews
```

Extract only comments authored by the repository owner or explicitly requested reviewers. Skip comments that are already resolved or outdated (check `position: null` on inline comments, which indicates the code has changed since the comment was made).

### 3. Categorize Comments

Group each comment into one of these categories:

| Category | Action |
|----------|--------|
| **suggestion** | Propose a specific code change |
| **question** | Include the answer in the action plan; propose code change if applicable |
| **refactor** | Propose a refactoring with before/after |
| **naming** | Rename variables/functions to favor meaning |
| **architecture** | Propose structural changes (module extraction, separation of concerns) |

For **question** comments: include a brief answer in the action plan so the user can review it alongside the proposed changes. Do not reply to the PR until the user approves in step 5.

### 4. Group by File

Present comments organized by file path, then by category within each file. For each comment:

- Quote the original comment
- State the category
- Propose the concrete change (code diff or explanation)

### 5. Build Action Plan

Produce a summary table:

| File | Comment | Category | Proposed Action |
|------|---------|----------|-----------------|
| `src/board.rs` | "favor meaning..." | naming | Rename `c` to `color`, `rh` to `rank_hint` |

### 6. Present for Approval

**Do NOT apply changes automatically.** Present the full action plan and wait for user approval before making any code modifications.

After approval, apply changes file by file, running `cargo check` after each file to ensure compilation. Run `cargo test` after all changes are applied.

### 7. Reply to Comments

After changes are applied and tests pass, ask the user if they want to reply to the PR comments indicating they've been addressed.

For **inline review comments**:
```bash
gh api repos/{owner}/{repo}/pulls/{pr_number}/comments/{comment_id}/replies -f body="Addressed: {brief description}"
```

For **general PR comments**:
```bash
gh api repos/{owner}/{repo}/issues/{pr_number}/comments -f body="Addressed: {brief description}"
```

## Conventions

Follow all conventions defined in the project's `CLAUDE.md`, especially:
- **Naming**: Favor meaning over brevity
- **Architecture**: Separation of concerns between REPL/CLI and engine
- **Comments**: Explain *why*, not *what*
- **Rust idioms**: No `unwrap()` in production, prefer borrowing, match all enum variants
