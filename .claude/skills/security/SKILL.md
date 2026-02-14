---
name: security
description: Security-first development for bash scripts. Input validation, injection prevention. Use when: security, secure this, input validation, sanitize, injection, safe, harden, validate input, user input, untrusted input, is this safe, security review, secure code, prevent injection, escape input.
---

# Security - First Class Citizen

## Bash Security Principles

1. **Never trust input** - All external data is hostile
2. **Quote everything** - Prevent word splitting and glob expansion
3. **Validate before use** - Check format, range, type
4. **Fail safely** - Use `set -euo pipefail`
5. **Least privilege** - Don't run as root, limit file permissions

## Safe Script Header

```bash
#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

# Fail on unset variables
# Fail on pipe errors
# Restrict IFS to newline and tab only
```

## Input Validation

### Chess Move Validation

```bash
validate_move() {
    local move="$1"

    # Allow only valid chess notation characters
    if [[ ! "$move" =~ ^[KQRBNP]?[a-h]?[1-8]?x?[a-h][1-8](=[QRBN])?[+#]?$ ]] &&
       [[ ! "$move" =~ ^O-O(-O)?[+#]?$ ]]; then
        echo "Invalid move: $move" >&2
        return 1
    fi
}

validate_square() {
    local square="$1"

    # Exactly 2 chars: a-h followed by 1-8
    if [[ ! "$square" =~ ^[a-h][1-8]$ ]]; then
        echo "Invalid square: $square" >&2
        return 1
    fi
}
```

### Filename Validation

```bash
validate_filename() {
    local file="$1"

    # No path traversal
    if [[ "$file" == *".."* ]]; then
        echo "Path traversal blocked" >&2
        return 1
    fi

    # Only allowed extensions
    if [[ ! "$file" =~ \.(pgn|wav)$ ]]; then
        echo "Invalid file type" >&2
        return 1
    fi

    # No special characters
    if [[ "$file" =~ [^a-zA-Z0-9._/-] ]]; then
        echo "Invalid characters in filename" >&2
        return 1
    fi
}
```

## Injection Prevention

### Command Injection

```bash
# VULNERABLE: User input in command
file="$1"
cat $file              # Word splitting + glob expansion
eval "cat $file"       # Direct injection
bash -c "cat $file"    # Subshell injection

# SAFE: Proper quoting
cat -- "$file"         # -- prevents option injection
```

### Arithmetic Injection

```bash
# VULNERABLE: Unvalidated arithmetic
num="$1"
result=$(($num + 1))   # If num="1; rm -rf /", bash expands it

# SAFE: Validate first
if [[ ! "$num" =~ ^-?[0-9]+$ ]]; then
    echo "Invalid number" >&2
    exit 1
fi
result=$((num + 1))
```

### PGN Parsing Safety

```bash
# PGN may contain:
# - Comments with arbitrary text: {any text here}
# - Annotations: $1, $2, etc.
# - Variations: (1.e4 e5 2.Nf3)
# - Results: 1-0, 0-1, 1/2-1/2

# SAFE: Strip dangerous content first
sanitize_pgn() {
    local pgn="$1"

    # Remove comments
    pgn="${pgn//\{*\}/}"

    # Remove variations
    pgn="${pgn//\(*\)/}"

    # Remove annotations
    pgn="${pgn//\$[0-9]*/}"

    # Only keep allowed characters
    pgn=$(echo "$pgn" | tr -cd 'a-hKQRBNPO0-9x=+#. \n-')

    echo "$pgn"
}
```

## Safe File Operations

```bash
# Create temp files safely
tmpfile=$(mktemp) || exit 1
trap 'rm -f "$tmpfile"' EXIT

# Safe file writing
write_wav() {
    local outfile="$1"
    local tmpout

    tmpout=$(mktemp) || return 1

    # Write to temp first
    generate_wav > "$tmpout" || { rm -f "$tmpout"; return 1; }

    # Atomic move
    mv "$tmpout" "$outfile"
}

# Check before overwrite
if [[ -e "$outfile" ]]; then
    echo "File exists: $outfile" >&2
    exit 1
fi
```

## Integer Overflow

```bash
# Bash uses signed 64-bit integers
# Max: 9223372036854775807
# Min: -9223372036854775808

safe_multiply() {
    local a=$1 b=$2
    local max=9223372036854775807

    # Check for overflow before operation
    if ((a > 0 && b > 0 && a > max / b)); then
        echo "Overflow" >&2
        return 1
    fi

    echo $((a * b))
}
```

## Security Checklist

- [ ] `set -euo pipefail` at script start
- [ ] All variables quoted
- [ ] Input validated against allowlist regex
- [ ] No `eval` with user data
- [ ] No unquoted command substitution
- [ ] Temp files created with `mktemp`
- [ ] Temp files cleaned up with `trap`
- [ ] No hardcoded secrets
- [ ] File permissions restricted (600/700)
- [ ] Path traversal blocked
- [ ] Integer bounds checked

## Error Messages

```bash
# DON'T leak information
echo "Database connection failed: $DB_PASSWORD" >&2  # BAD

# DO be vague externally
echo "An error occurred" >&2                          # GOOD
logger "DB failed: $DB_ERROR"                         # Log internally
```
