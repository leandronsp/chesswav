# 🎵 ChessWAV - Technical Roadmap

## Product Vision

**ChessWAV** transforms chess games into audio. Each move becomes a note, each piece has its timbre, each capture has its drama.

**Input**: PGN file or moves via STDIN
**Output**: Playable WAV file

---

## Epics and Features

### 📦 EPIC 1: Chess Engine Core

The heart of the system — represent and validate board state.

#### Feature 1.1: Board Representation

**User Story**: As a system, I need to represent the board state to know where each piece is.

**Acceptance Criteria**:
- [ ] Board represented as data structure (64 positions)
- [ ] Initial position configurable (standard chess setup)
- [ ] Function to print board (debug)
- [ ] Function to get piece at a square
- [ ] Function to set piece at a square

**Deliverables**:
- Board module with unit tests

---

#### Feature 1.2: Algebraic Notation Parser

**User Story**: As a system, I need to convert algebraic notation (e.g., "Nf3") into concrete movement (origin → destination).

**Acceptance Criteria**:
- [ ] Parse pawn moves: `e4`, `d5`, `exd5`
- [ ] Parse piece moves: `Nf3`, `Bb5`, `Qh4`, `Rad1`
- [ ] Parse captures: `Bxc6`, `exd5`, `Qxf7`
- [ ] Parse promotions: `e8=Q`, `bxa1=N`
- [ ] Parse castling: `O-O` (kingside), `O-O-O` (queenside)
- [ ] Parse annotations: `+` (check), `#` (checkmate)
- [ ] Return struct with: piece, origin, destination, capture?, promotion?, check?, mate?

**Known Complexities**:
- Disambiguation: `Rad1` vs `Rfd1` (which rook?)
- Determining origin requires knowing board state
- En passant needs history (last move)

**Deliverables**:
- Notation parser module with unit tests

---

#### Feature 1.3: Move Resolver

**User Story**: As a system, I need to determine the origin square of a move based on current board state.

**Acceptance Criteria**:
- [ ] Given `Nf3` and initial board → origin is `g1`
- [ ] Given `Rad1` → find rook on column 'a'
- [ ] Validate that move is legal (piece exists, path clear)
- [ ] Support en passant (requires tracking last move)
- [ ] Support castling (validate king/rook haven't moved, path clear, doesn't pass through check)

**Deliverables**:
- Move resolver module with unit tests

---

#### Feature 1.4: Move Executor

**User Story**: As a system, I need to apply a move to the board and update state.

**Acceptance Criteria**:
- [ ] Move piece from origin to destination
- [ ] Remove captured piece
- [ ] Execute castling (move king AND rook)
- [ ] Execute en passant (capture adjacent pawn)
- [ ] Promote pawn
- [ ] Update turn (white/black)
- [ ] Maintain move history (for en passant and repetition)

**Deliverables**:
- Move executor module with unit tests

---

#### Feature 1.5: PGN Parser

**User Story**: As a user, I want to load a game from a PGN file.

**Acceptance Criteria**:
- [ ] Extract movetext ignoring headers `[Event "..."]`
- [ ] Ignore comments `{...}` and variations `(...)`
- [ ] Ignore move numbers `1.` `2.`
- [ ] Ignore result `1-0`, `0-1`, `1/2-1/2`, `*`
- [ ] Return clean list of moves
- [ ] Support multi-line PGN

**Deliverables**:
- PGN parser module with unit tests

---

### 📦 EPIC 2: Audio Synthesis Engine

Transform moves into sound.

#### Feature 2.1: Square → Frequency Mapping

**User Story**: As a system, I need to convert a board square into an audible frequency.

**Acceptance Criteria**:
- [ ] Map columns a-h to notes C, D, E, F, G, A, B, C
- [ ] Map ranks 1-8 to octaves (1=low, 8=high)
- [ ] Base frequency: A4 = 440Hz
- [ ] Function: `square_to_freq("e4")` → `329.63`

**Deliverables**:
- Frequency mapping module with unit tests

---

#### Feature 2.2: Waveform Generators

**User Story**: As a system, I need to generate different waveforms to give personality to pieces.

**Acceptance Criteria**:
- [ ] Sine wave (pawn)
- [ ] Triangle wave (knight)
- [ ] Square wave (rook)
- [ ] Sawtooth wave (bishop)
- [ ] Composite wave (queen) — sum of harmonics
- [ ] Function receives: frequency, duration (samples), type

**Deliverables**:
- Waveform generator module with unit tests

---

#### Feature 2.3: ADSR Envelope (Stretch Goal)

**User Story**: As a system, I want notes to have natural dynamics (attack, sustain, decay).

**Acceptance Criteria**:
- [ ] Attack: time to reach max volume
- [ ] Decay: time to fall to sustain
- [ ] Sustain: level maintained while note plays
- [ ] Release: fade out after note ends

**Note**: Can be simplified for v1 with linear fade-in/fade-out only.

**Deliverables**:
- Envelope module with unit tests

---

#### Feature 2.4: Piece → Timbre Mapping

**User Story**: As a system, I need to associate each piece type with a waveform.

**Acceptance Criteria**:
- [ ] Configuration of piece → waveform
- [ ] Configuration of special events:
  - Capture: shorter note + harmonic
  - Check: vibrato or tremolo
  - Checkmate: long note with dramatic fade
  - Castling: two simultaneous notes (chord)

**Deliverables**:
- Piece-to-sound mapping module with unit tests

---

#### Feature 2.5: Move Synthesizer

**User Story**: As a system, I need to generate audio samples for a complete move.

**Acceptance Criteria**:
- [ ] Receives: parsed move (piece, destination, flags)
- [ ] Returns: array of samples (bytes)
- [ ] Base duration: 300ms per move
- [ ] Capture: adds captured piece note (shorter, decaying)
- [ ] Silence between moves: 50ms

**Deliverables**:
- Move synthesizer module with unit tests

---

### 📦 EPIC 3: WAV Output

Generate valid audio file.

#### Feature 3.1: WAV Header Generator

**User Story**: As a system, I need to generate a valid WAV header.

**Acceptance Criteria**:
- [ ] Correct RIFF chunk
- [ ] fmt subchunk with audio parameters
- [ ] data subchunk with correct size
- [ ] Little-endian where required
- [ ] Function receives: total number of samples

**Deliverables**:
- WAV header module with unit tests

---

#### Feature 3.2: WAV File Writer

**User Story**: As a system, I need to combine header + samples and write WAV file.

**Acceptance Criteria**:
- [ ] Concatenate header + all samples
- [ ] Write valid binary file
- [ ] File plays correctly in standard audio players

**Deliverables**:
- WAV writer module with integration tests

---

### 📦 EPIC 4: CLI & Integration

User interface.

#### Feature 4.1: Main CLI

**User Story**: As a user, I want a simple interface to convert games to audio.

**Acceptance Criteria**:
- [ ] `chesswav game.pgn -o output.wav` — PGN file input
- [ ] `echo "e4 e5 Nf3" | chesswav -o output.wav` — STDIN input
- [ ] `chesswav --interactive` — interactive mode (move by move)
- [ ] Flag `--play` — play after generating
- [ ] Flag `--verbose` — show each move being processed
- [ ] Flag `--help` — usage

**Deliverables**:
- CLI binary with integration tests

---

#### Feature 4.2: Streaming/Interactive Mode

**User Story**: As a user, I want to type moves and hear them immediately.

**Acceptance Criteria**:
- [ ] REPL loop: prompt → move → sound
- [ ] Play via system audio
- [ ] Show ASCII board after each move
- [ ] Command `quit` to exit
- [ ] Command `reset` for new game

**Deliverables**:
- Interactive mode module

---

## 🎯 MVP (Minimum Viable Product)

The simplest possible working version:

1. ✅ Basic board representation
2. ✅ Simple move parser (pawns, pieces — no castling, no en passant)
3. ✅ Square → frequency mapping
4. ✅ Sine wave only
5. ✅ WAV output
6. ✅ Basic CLI

**MVP deliverable**:
```bash
echo "e4 e5 Nf3 Nc6" | chesswav > game.wav
```

Play a simple opening and hear it. That's it.

---

## Open Questions

1. **Testing strategy**: Unit tests per module + integration tests for CLI?

2. **Performance**: Acceptable limit for long games? (40 moves = ~12 seconds of audio)

3. **Polyphony**: V1 monophonic (one note at a time) or chords from the start?

4. **En passant / Castling**: Full implementation or simplify for v1?

5. **Project name**: ChessWAV? PGN2WAV? ChessSynth? Something more creative?

---

## Suggested Sprint Plan

### Sprint 1: Foundation (MVP)
- Feature 1.1: Board Representation
- Feature 1.2: Notation Parser (basic)
- Feature 2.1: Square → Frequency
- Feature 2.2: Sine wave generator
- Feature 3.1: WAV Header
- Feature 3.2: WAV Writer
- Feature 4.1: Basic CLI

### Sprint 2: Chess Logic
- Feature 1.2: Complete parser (captures, promotion, castling)
- Feature 1.3: Move Resolver
- Feature 1.4: Move Executor

### Sprint 3: Audio Richness
- Feature 2.2: All waveforms
- Feature 2.4: Piece → Timbre mapping
- Feature 2.5: Move Synthesizer

### Sprint 4: Full Features
- Feature 1.5: PGN Parser
- Feature 2.3: ADSR Envelope
- Feature 4.2: Interactive Mode

### Sprint 5: Polish
- Documentation
- Error handling
- Edge cases

---

## Future Refinements

PRDs will be created per feature as we progress through sprints.

### Multiplayer via Sockets

Real-time multiplayer mode using TCP sockets. Two players connect, play a game, and each move generates audio for both sides. Audio can be opted out by either player. Perfect for live stream X1 challenges.
