# PRD: Check (+) Audio Differentiation

> Source: ROADMAP.md → Epic 2 → Feature 2.4 (Piece → Timbre Mapping — check event)

## Problem Statement

Currently, `Nf3+` and `Nf3` produce identical audio. The `+` annotation is stripped during parsing and has no effect on the generated sound. A check is a significant chess event — the king is under attack — and should be audible.

## Goal

Moves with check (`+`) produce a subtly different timbre than the same move without check, using the existing `Blend` system. The difference should be noticeable but not jarring — a tonal shift, not an alarm.

## Success Criteria

- `Nf3+` sounds different from `Nf3`
- `e4` (no check) is unaffected
- The check effect works consistently across all piece types
- All existing tests pass
- No external dependencies added

## Current State

```
"Nf3+" → strip_annotations() → "Nf3" → Move { piece: Knight, dest: f3 }
                ↑
          check info LOST
```

`Move::parse()` calls `strip_annotations()` which removes `+` entirely. The `Move` struct has no field to carry this information forward.

## Target State

```
"Nf3+" → Move { piece: Knight, dest: f3, check: true }
              │
              ▼
         triangle(349Hz, blend with sine_mix for check)
```

The `+` annotation is preserved in the `Move` struct and used in the audio pipeline to modify the `Blend` parameters, adding a sine mix that brightens the timbre when a move delivers check.

## Audio Design

### Blend-Based Check Effect

The existing `Blend` struct already supports `sine_mix` (0.0–1.0) and `harmonics` band-limiting. Check modifies the blend by increasing sine mix, which adds a "shimmer" or "brightness" to the note:

| Piece | Normal Blend | Check Blend |
|-------|-------------|-------------|
| Pawn | `sine` (no blend) | `sine` + `harmonics: Some(3)` |
| Knight | `Blend::none()` | `Blend::with_sine(0.3)` |
| Rook | `sine_mix: 0.4, harmonics: 7` | `sine_mix: 0.6, harmonics: 7` |
| Bishop | `sine_mix: 0.3, harmonics: 8` | `sine_mix: 0.5, harmonics: 8` |
| Queen | `Blend::none()` | `Blend::with_sine(0.3)` |
| King | `Blend::none()` | `Blend::with_sine(0.3)` |

The idea: check pushes each piece's timbre slightly toward sine (purer/brighter), creating an audible "lift" that signals tension.

For Pawn specifically (already pure sine), check adds harmonics overtones to differentiate — making the pawn sound richer rather than brighter.

## Technical Requirements

### 1. Domain Model (`chess.rs`)

Add `check` field to `Move`:

```rust
pub struct Move {
    pub piece: Piece,
    pub dest: Square,
    pub check: bool,
}
```

Update `Move::parse()` to detect `+` before stripping annotations:

```rust
pub fn parse(input: &str, move_index: usize) -> Option<Move> {
    let check = input.contains('+');
    let clean = Self::strip_annotations(input);
    // ... rest of parsing ...
    Some(Move { piece, dest, check })
}
```

### 2. Audio Pipeline (`audio.rs`)

Update `move_to_samples()` to adjust blend based on `check`:

```rust
fn move_to_samples(m: &Move, silence: &[i16]) -> Vec<i16> {
    let freq = freq::from_square(&m.dest);
    let note = match (m.piece, m.check) {
        (Piece::Pawn, false)   => synth::sine(freq, NOTE_MS),
        (Piece::Pawn, true)    => synth::harmonics(freq, NOTE_MS, Blend::none()),
        (Piece::Knight, false) => synth::triangle(freq, NOTE_MS, Blend::none()),
        (Piece::Knight, true)  => synth::triangle(freq, NOTE_MS, Blend::with_sine(0.3)),
        // ... etc
    };
    // ...
}
```

### 3. No Changes Needed

- `blend.rs` — already supports all needed operations
- `synth.rs` — already accepts `Blend` for all waveform types
- `waveform.rs` — no changes
- `freq.rs` — no changes
- `wav.rs` — no changes

## Test Cases

### Parser Tests

```rust
#[test]
fn check_detected() {
    let m = Move::parse("Nf3+", 0).unwrap();
    assert!(m.check);
}

#[test]
fn no_check_by_default() {
    let m = Move::parse("Nf3", 0).unwrap();
    assert!(!m.check);
}

#[test]
fn check_on_pawn() {
    let m = Move::parse("e4+", 0).unwrap();
    assert!(m.check);
    assert_eq!(m.piece, Piece::Pawn);
}
```

### Audio Tests

```rust
#[test]
fn check_produces_different_samples() {
    let normal = generate("Nf3");
    let check = generate("Nf3+");
    assert_ne!(normal, check);
}

#[test]
fn check_same_length_as_normal() {
    assert_eq!(generate("Nf3").len(), generate("Nf3+").len());
}
```

## Implementation Order

1. Add `check: bool` to `Move` struct
2. Update `Move::parse()` to detect `+` before stripping
3. Update all existing test expectations for new field
4. Add parser tests for check detection
5. Update `move_to_samples()` with check-aware blend routing
6. Add audio tests comparing check vs non-check output
7. Run `cargo clippy` and `cargo test`

## Out of Scope

- Checkmate (`#`) audio — separate future PRD
- Vibrato/tremolo effects — would require `synth.rs` changes
- Duration changes for check moves
- Volume/amplitude changes for check

## Dependencies

None. All required infrastructure (`Blend`, piece-aware parsing, waveform generators) already exists.

## Open Questions

1. **Pawn check strategy**: Should pawn check switch to `harmonics` waveform, or should we add a `Blend` parameter to `synth::sine()`?
2. **Check blend values**: The sine_mix values (0.3, 0.5, 0.6) are initial proposals — may need tuning by ear after implementation.
