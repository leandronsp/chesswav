---
name: bug-finder
description: Hunt bugs through edge cases, corrupted input, stress testing. Use when: find bugs, break this, try to break, edge cases, stress test, QA this, what could go wrong, chaos test, test edge cases, find issues, hunt bugs, look for bugs, any bugs, something wrong, debug this, why is this failing, it's broken, not working.
---

# Bug Finder - Chaos Engineering for Bash

## Philosophy

**Reproduce first, then fix.** Never assume a bug exists until you see it fail.

## Bug Hunting Strategy

### 1. Input Fuzzing

```bash
# Empty input
echo "" | ./chesswav

# Whitespace only
echo "   " | ./chesswav

# Invalid moves
echo "Zz9 invalid !@#$" | ./chesswav

# Extremely long input
printf 'e4 %.0s' {1..10000} | ./chesswav

# Unicode chaos
echo "e4♞e5🏰Nf3" | ./chesswav

# Null bytes
printf 'e4\x00e5' | ./chesswav

# Special characters
echo "e4; rm -rf /" | ./chesswav  # injection attempt
echo 'e4 $(whoami) e5' | ./chesswav
```

### 2. Boundary Testing

```bash
# Frequency boundaries
square_to_freq "a1"  # lowest
square_to_freq "h8"  # highest

# Sample count boundaries
synth_sine 20 0      # zero samples
synth_sine 20 1      # single sample
synth_sine 20 999999 # huge sample count

# Amplitude boundaries
# Test clipping at -32768 and 32767
```

### 3. State Corruption

```bash
# Uninitialized board
board_get "e4"  # before board_init

# Invalid squares
board_get "z9"
board_get ""
board_get "e44"

# Move on empty square
# Capture non-existent piece
# Castle when already moved
```

### 4. Arithmetic Edge Cases

```bash
# Division by zero
# Integer overflow in sample calculation
# Negative frequencies
# NaN propagation (if using bc)
```

## Common Bash Bugs

### Unquoted Variables
```bash
# BUG: fails with spaces or globs
file=$1
cat $file

# FIX
cat "$file"
```

### Word Splitting in Arrays
```bash
# BUG: splits on spaces
arr="one two three"
for item in $arr; do

# FIX
arr=("one" "two" "three")
for item in "${arr[@]}"; do
```

### Arithmetic Overflow
```bash
# BUG: bash integers are signed 64-bit
echo $((2**63))  # wraps to negative

# FIX: check bounds before operations
```

### Missing Error Handling
```bash
# BUG: continues on failure
cd "$dir"
rm -rf *

# FIX
cd "$dir" || exit 1
```

## Bug Report Format

```markdown
## 🐛 Bug Report

### Summary
One-line description

### Reproduction
\`\`\`bash
# Exact commands to reproduce
echo "bad input" | ./chesswav
\`\`\`

### Expected
What should happen

### Actual
What actually happens (include error output)

### Root Cause
Analysis of why it fails

### Suggested Fix
Code change to resolve
```

## Edge Case Checklist

- [ ] Empty input
- [ ] Whitespace-only input
- [ ] Invalid chess notation
- [ ] Out-of-range squares
- [ ] Illegal moves
- [ ] Very long games (100+ moves)
- [ ] Special characters in PGN comments
- [ ] Binary data in input
- [ ] Concurrent execution
- [ ] Disk full during WAV write
- [ ] Permission denied scenarios
- [ ] Interrupted execution (Ctrl+C)

## Workflow

1. **Identify target** - What function/module to break?
2. **List assumptions** - What does the code assume about input?
3. **Violate assumptions** - Create inputs that break those assumptions
4. **Document failures** - Record exact reproduction steps
5. **Write failing test** - Capture the bug as a test case
6. **Fix with TDD** - Use `/tdd` to implement fix
