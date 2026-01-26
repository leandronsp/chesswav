# PRD: ChessWAV Rust Migration

**Status: COMPLETED**

## Overview

Migrated ChessWAV from pure Bash to Rust while maintaining identical CLI interface and behavior. The migration achieved 1000x+ performance improvement while preserving the modular architecture and TDD practices.

## Goals

1. **Performance**: Sub-second WAV generation for full games (currently 5+ seconds)
2. **Compatibility**: Identical CLI interface (`echo "e4 e5" | chesswav > game.wav`)
3. **Maintainability**: Same modular structure, easier to extend
4. **Portability**: Single binary, no runtime dependencies

## Non-Goals

- Adding new features during migration (feature parity only)
- Changing the musical mapping or audio format
- GUI or web interface

## Success Criteria

- All existing integration tests pass (adapted to Rust)
- WAV output is byte-compatible with Bash version
- Build produces single static binary
- `cargo test` achieves same coverage as Bash tests

---

## Migration Strategy

**Approach**: Bottom-up, module-by-module migration with parallel Bash/Rust operation until complete.

Each phase delivers a working, tested module. The main executable switches to Rust only after all modules are migrated and tested.

---

## Phase 1: Project Setup & WAV Module

**Objective**: Establish Rust project structure and migrate the simplest I/O module.

### Tasks

1. Initialize Cargo project in `rust/` subdirectory
2. Set up CI with `cargo test` and `cargo clippy`
3. Implement `wav.rs` module:
   - `wav_header(num_samples: u32) -> Vec<u8>`
   - Write RIFF/WAVE header (44 bytes)
   - 16-bit PCM, mono, 44100 Hz
4. Port `test_wav.sh` tests to Rust unit tests
5. Verify byte-identical output with Bash version

### Acceptance Criteria

```rust
let header = wav::header(44100); // 1 second of audio
assert_eq!(header.len(), 44);
assert_eq!(&header[0..4], b"RIFF");
assert_eq!(&header[8..12], b"WAVE");
```

### Deliverables

- `rust/Cargo.toml`
- `rust/src/lib.rs`
- `rust/src/wav.rs`
- `rust/src/wav/tests.rs`

---

## Phase 2: Synth Module

**Objective**: Migrate audio synthesis (the performance bottleneck).

### Tasks

1. Implement `synth.rs` module:
   - `synth_sine(frequency: u32, duration_ms: u32) -> Vec<i16>`
   - Use `f64` for sine calculation (Rust stdlib `sin()`)
   - Sample rate: 44100 Hz
   - Amplitude: 28000 (matching Bash)
2. Port `test_synth.sh` tests
3. Benchmark: compare generation time vs Bash

### Acceptance Criteria

```rust
let samples = synth::sine(440, 100); // A4 for 100ms
assert_eq!(samples.len(), 4410); // 44100 * 0.1
```

### Expected Performance

| Metric | Bash | Rust |
|--------|------|------|
| 1 second audio | ~2s | <1ms |
| 40-move game | ~5s | <10ms |

---

## Phase 3: Frequency Module

**Objective**: Migrate square-to-frequency mapping.

### Tasks

1. Implement `freq.rs` module:
   - `freq_from_square(square: &str) -> u32`
   - Column→note mapping (a=C, b=D, ..., h=C)
   - Rank→octave transposition
2. Port `test_freq.sh` tests

### Acceptance Criteria

```rust
assert_eq!(freq::from_square("e4"), 392);  // G4
assert_eq!(freq::from_square("e5"), 784);  // G5 (octave up)
assert_eq!(freq::from_square("f4"), 440);  // A4
```

---

## Phase 4: Notation Module

**Objective**: Migrate algebraic notation parser.

### Tasks

1. Implement `notation.rs` module:
   - `parse(move_str: &str) -> Option<Move>`
   - `Move` struct: `piece`, `dest`, `capture`
   - Handle: pawn moves, piece moves, captures, annotations
2. Port `test_notation.sh` tests

### Data Structures

```rust
#[derive(Debug, PartialEq)]
pub enum Piece { Pawn, Knight, Bishop, Rook, Queen, King }

#[derive(Debug)]
pub struct Move {
    pub piece: Piece,
    pub dest: String,    // e.g., "e4"
    pub capture: bool,
}
```

### Acceptance Criteria

```rust
let m = notation::parse("Nxf3").unwrap();
assert_eq!(m.piece, Piece::Knight);
assert_eq!(m.dest, "f3");
assert!(m.capture);
```

---

## Phase 5: Board Module

**Objective**: Migrate board representation (for future move validation).

### Tasks

1. Implement `board.rs` module:
   - `Board::new() -> Board` (standard starting position)
   - `board.get(square: &str) -> Option<Piece>`
   - `board.set(square: &str, piece: Option<Piece>)`
2. Port `test_board.sh` tests

### Data Structures

```rust
pub struct Board {
    squares: [Option<(Piece, Color)>; 64],
}

pub enum Color { White, Black }
```

---

## Phase 6: CLI & Integration

**Objective**: Create main executable and retire Bash version.

### Tasks

1. Implement `main.rs`:
   - Read moves from stdin
   - Parse each move
   - Generate audio samples
   - Write WAV to stdout
2. Add `--play` flag (pipe to `aplay` on Linux, `afplay` on macOS)
3. Port `test_integration.sh` tests
4. Move Rust binary to project root as `chesswav`
5. Archive Bash version to `legacy/`

### CLI Interface (unchanged)

```bash
# Basic usage
echo "e4 e5 Nf3 Nc6" | ./chesswav > game.wav

# With playback
echo "e4 e5 Nf3" | ./chesswav --play

# From file
./chesswav < moves.txt > output.wav
```

### Acceptance Criteria

- `./chesswav` produces identical WAV output as Bash version
- All integration tests pass
- Binary size < 2MB (static linking)
- Works on Linux and macOS

---

## Phase 7: Cleanup & Documentation

**Objective**: Finalize migration.

### Tasks

1. Update `CLAUDE.md` for Rust conventions
2. Update `README.md` with build instructions
3. Remove `lib/` Bash modules
4. Update `ROADMAP.md` for future Rust features
5. Add `Makefile` or `justfile` for common tasks

---

## File Structure (Final)

```
chesswav/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs           # CLI entry point
│   ├── lib.rs            # Library exports
│   ├── wav.rs            # WAV format
│   ├── synth.rs          # Audio synthesis
│   ├── freq.rs           # Frequency mapping
│   ├── notation.rs       # Move parsing
│   └── board.rs          # Board state
├── tests/
│   └── integration.rs    # End-to-end tests
├── legacy/               # Archived Bash version
│   ├── chesswav.sh
│   └── lib/
├── CLAUDE.md
├── README.md
└── docs/
    └── prd/
        └── rust-migration.md
```

---

## Technical Decisions

### Dependencies

Minimal, stdlib-focused:

```toml
[dependencies]
# None required for MVP

[dev-dependencies]
# None required for MVP
```

If needed later:
- `hound` - WAV file I/O (optional, manual implementation preferred)
- `clap` - CLI argument parsing (optional, manual for MVP)

### Error Handling

```rust
// Use Result for fallible operations
pub fn parse(input: &str) -> Result<Move, ParseError>

// Main handles errors gracefully (skip invalid moves, like Bash)
```

### Testing Strategy

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_wav_header_size() {
        let header = super::header(1000);
        assert_eq!(header.len(), 44);
    }
}
```

Run: `cargo test`

---

## Timeline Estimate

| Phase | Scope |
|-------|-------|
| 1 | Project setup + WAV module |
| 2 | Synth module |
| 3 | Freq module |
| 4 | Notation module |
| 5 | Board module |
| 6 | CLI + Integration |
| 7 | Cleanup |

---

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| WAV byte mismatch | Compare hex dumps, test with audio players |
| Platform differences | Test on Linux + macOS CI |
| Scope creep | Strict feature parity, no new features |

---

## Open Questions

1. Should we use `hound` crate or implement WAV manually? (Recommendation: manual, matches Bash approach)
2. Keep Bash tests as reference or convert all to Rust? (Recommendation: convert all)
3. Cross-compile for Windows? (Recommendation: defer to post-migration)
