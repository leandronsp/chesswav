---
name: review
description: Deep code review with bash best practices, security, audio correctness. Use when: review, review this, code review, check this code, review my changes, look at this code, analyze this, critique this, is this good, what do you think, review the code, check my implementation.
---

# Code Review - Bash & Audio Expert

## Expertise Applied

- **Bash mastery**: Modern expansions, POSIX compliance, shellcheck rules
- **Audio synthesis**: Correct waveform math, frequency calculations, WAV format
- **Chess logic**: Notation parsing, board state, move validation
- **Security**: Input validation, injection prevention, safe arithmetic

## Review Priorities

### 1. Correctness
- Does the math work? (frequencies, waveforms, byte calculations)
- Is the chess logic right? (legal moves, board state)
- Are edge cases handled?

### 2. Bash Best Practices
```bash
# BAD: word splitting, glob expansion
for file in $files; do

# GOOD: proper quoting
for file in "${files[@]}"; do

# BAD: external command
length=$(echo "$str" | wc -c)

# GOOD: parameter expansion
length=${#str}

# BAD: command substitution in arithmetic
result=$(($(cat file)))

# GOOD: read into variable first
value=$(<file)
result=$((value))
```

### 3. Security
- Input sanitization (PGN/move input)
- No eval with user input
- Safe arithmetic (avoid overflow)
- No command injection vectors

### 4. Performance
- Avoid subshells in loops
- Use bash builtins over external commands
- Efficient array operations

### 5. Modularity
- Single responsibility per function
- Clear interfaces between modules
- No global state pollution

## Review Structure

```markdown
### Critical 🔴
1) **Issue**: description
   **Fix**: solution

### Important 🟡
A) **Issue**: description
   **Suggestion**: approach

### Minor 🟢
* Nitpick or suggestion
```

## Bash-Specific Checks

- [ ] All variables quoted: `"${var}"`
- [ ] Arrays properly declared: `declare -a` / `declare -A`
- [ ] Arithmetic in `$(( ))` not `$[ ]`
- [ ] `[[ ]]` not `[ ]` for conditionals
- [ ] `local` for function variables
- [ ] Proper error handling with `set -euo pipefail`
- [ ] No useless use of cat/echo
- [ ] Parameter expansion over sed/awk when possible

## Audio-Specific Checks

- [ ] Frequencies calculated with equal temperament
- [ ] Sample rate consistent (44100 Hz)
- [ ] 16-bit signed samples (-32768 to 32767)
- [ ] Little-endian byte order for WAV
- [ ] No clipping (amplitude within bounds)
- [ ] Correct RIFF chunk sizes

## Chess-Specific Checks

- [ ] Algebraic notation correctly parsed
- [ ] Board state updated after each move
- [ ] Disambiguation handled (Rad1 vs Rfd1)
- [ ] Special moves: castling, en passant, promotion

## Skill Workflow

```
ROADMAP.md
    │
    ▼
/po      → Creates PRD from ROADMAP
    │
    ▼
/analyst → Creates SPEC from PRD (with tasks)
    │
    ▼
/tdd     → Implements tasks from SPEC
    │
    ▼
/review  → Quality gate before merge ← YOU ARE HERE
    │
    ▼
/commit  → Commits completed work
```

## When to Review

- After `/tdd` completes a task
- Before merging feature branches
- When refactoring existing code
- After bug fixes
