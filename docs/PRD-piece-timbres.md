# PRD: Piece-Based Timbres

## Problem Statement

Currently, all chess moves produce identical sine wave tones. `Kf3` and `Nf3` sound exactly the same because piece information is discarded during parsing. This makes it impossible to distinguish pieces by ear, reducing the musical expressiveness of the output.

## Goal

Each piece type produces a distinct timbre (waveform), allowing listeners to identify pieces by their sound character alone.

## Success Criteria

- `Nf3` (Knight) produces a triangle wave
- `Kf3` (King) produces a sine wave with harmonics
- Running `echo "e4 Nf3" | cargo run` produces two audibly different tones
- All existing tests pass
- No external dependencies added

## Piece-to-Waveform Mapping

| Piece | Notation | Waveform | Character |
|-------|----------|----------|-----------|
| Pawn | `e4`, `d5` | Sine | Pure, clean |
| Knight | `Nf3`, `Nc6` | Triangle | Mellow, soft |
| Bishop | `Bb5`, `Bxc6` | Sawtooth | Bright, buzzy |
| Rook | `Rad1`, `Re1` | Square | Hollow, woody |
| Queen | `Qh4`, `Qxf7` | Composite | Rich, full |
| King | `Kf1`, `O-O` | Sine + harmonics | Warm, noble |

## Current State

```
"Nf3" → Move::parse() → Move { dest: f3 } → sine(349Hz) → WAV
                              ↑
                        piece info LOST
```

## Target State

```
"Nf3" → Move::parse() → Move { piece: Knight, dest: f3 } → triangle(349Hz) → WAV
                              ↑
                        piece info PRESERVED
```

## Technical Requirements

### 1. Domain Model (`chess.rs`)

Add `Piece` enum and include it in `Move`:

```rust
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Move {
    pub piece: Piece,
    pub dest: Square,
}
```

### 2. Parser Updates (`chess.rs`)

Update `Move::parse()` to extract piece type:

| Input | Piece Detection |
|-------|-----------------|
| `e4`, `exd5` | Pawn (no uppercase prefix) |
| `Nf3`, `Nxe5` | Knight (N prefix) |
| `Bb5`, `Bxc6` | Bishop (B prefix) |
| `Rad1`, `Rxe1` | Rook (R prefix) |
| `Qh4`, `Qxf7` | Queen (Q prefix) |
| `Kf1`, `Kxd2` | King (K prefix) |
| `O-O`, `O-O-O` | King (castling) |

### 3. Waveform Generators (`synth.rs`)

Implement missing waveform functions:

```rust
pub fn sine(freq: f32, duration_ms: u32) -> Vec<i16>      // EXISTS
pub fn triangle(freq: f32, duration_ms: u32) -> Vec<i16>  // NEW
pub fn square(freq: f32, duration_ms: u32) -> Vec<i16>    // NEW
pub fn sawtooth(freq: f32, duration_ms: u32) -> Vec<i16>  // NEW
pub fn composite(freq: f32, duration_ms: u32) -> Vec<i16> // NEW
pub fn harmonics(freq: f32, duration_ms: u32) -> Vec<i16> // NEW
```

**Waveform Formulas:**

- **Triangle**: `4 * |((t * freq) % 1) - 0.5| - 1`
- **Square**: `sign(sin(2π * freq * t))`
- **Sawtooth**: `2 * ((t * freq) % 1) - 1`
- **Composite**: Sum of sine at f, f*2, f*3 with decreasing amplitudes
- **Harmonics**: Sine + 0.5*sin(2f) + 0.25*sin(3f)

### 4. Audio Pipeline (`audio.rs`)

Update `move_to_samples()` to route pieces to waveforms:

```rust
fn move_to_samples(m: &Move) -> Vec<i16> {
    let freq = freq::from_square(&m.dest);
    match m.piece {
        Piece::Pawn   => synth::sine(freq, NOTE_MS),
        Piece::Knight => synth::triangle(freq, NOTE_MS),
        Piece::Bishop => synth::sawtooth(freq, NOTE_MS),
        Piece::Rook   => synth::square(freq, NOTE_MS),
        Piece::Queen  => synth::composite(freq, NOTE_MS),
        Piece::King   => synth::harmonics(freq, NOTE_MS),
    }
}
```

## Implementation Order

1. **Add `Piece` enum** to `chess.rs`
2. **Update `Move` struct** to include piece field
3. **Update parser** to extract piece from notation
4. **Add unit tests** for piece parsing
5. **Implement waveform generators** in `synth.rs` (triangle, square, sawtooth)
6. **Add unit tests** for each waveform
7. **Implement composite waveforms** (composite, harmonics)
8. **Update `move_to_samples()`** to route pieces to waveforms
9. **Integration test** end-to-end with mixed pieces

## Test Cases

### Parser Tests

```rust
#[test]
fn parses_pawn_move() {
    let m = Move::parse("e4").unwrap();
    assert_eq!(m.piece, Piece::Pawn);
}

#[test]
fn parses_knight_move() {
    let m = Move::parse("Nf3").unwrap();
    assert_eq!(m.piece, Piece::Knight);
}

#[test]
fn parses_castling_as_king() {
    let m = Move::parse("O-O").unwrap();
    assert_eq!(m.piece, Piece::King);
}
```

### Waveform Tests

```rust
#[test]
fn triangle_wave_range() {
    let samples = synth::triangle(440.0, 100);
    assert!(samples.iter().all(|&s| s >= -32767 && s <= 32767));
}

#[test]
fn square_wave_binary() {
    let samples = synth::square(440.0, 100);
    assert!(samples.iter().all(|&s| s == -32767 || s == 32767));
}
```

## Out of Scope

- ADSR envelope shaping (future enhancement)
- Volume normalization across waveforms
- Stereo panning based on board position
- Capture/check sound effects

## Risks

| Risk | Mitigation |
|------|------------|
| Composite waveforms clip at high frequencies | Normalize amplitude after summing |
| Square waves sound harsh | Consider slight smoothing or reduced amplitude |
| Parsing edge cases (promotions, en passant) | Default to pawn for ambiguous notation |

## Dependencies

None. Pure Rust stdlib only.
