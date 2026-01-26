# ChessWAV MVP - Product Requirements Document

## Problem Statement

Chess enthusiasts and players want new ways to experience their games. While visual replays are common, there's no simple tool to transform chess games into audio experiences. ChessWAV fills this gap by converting chess moves into musical notes, creating unique soundscapes from games.

## Target Users

### Primary Persona: Chess Hobbyist
- Plays chess casually
- Interested in creative interpretations of games
- Comfortable with command-line tools
- Wants a simple, no-dependency solution

### Secondary Persona: Developer/Tinkerer
- Wants to understand audio synthesis
- Values pure implementations without external dependencies
- Interested in bash scripting capabilities

## User Stories

### US-1: Convert Moves to Audio
**As a** chess player
**I want to** pipe chess moves to a command and get a WAV file
**So that** I can hear my game as music

### US-2: Simple Input Format
**As a** user
**I want to** input moves in standard algebraic notation
**So that** I don't need to learn a new format

### US-3: Standard Output
**As a** user
**I want to** receive a standard WAV file
**So that** I can play it with any audio player

## Functional Requirements

### FR-1: Board Representation
- System maintains internal board state
- Initial position is standard chess setup
- Board updates after each move
- Support getting/setting piece at any square

### FR-2: Move Parsing
- Parse pawn moves: `e4`, `d5`
- Parse piece moves: `Nf3`, `Bb5`, `Qh4`
- Parse captures: `Bxc6`, `exd5`
- Extract piece type and destination square
- **Out of scope for MVP**: castling, en passant, promotion, disambiguation

### FR-3: Frequency Mapping
- Map columns a-h to musical notes C, D, E, F, G, A, B, C
- Map ranks 1-8 to octaves (lower ranks = lower octaves)
- Use A4 = 440Hz as reference
- Return integer frequencies (scaled for bash arithmetic)

### FR-4: Waveform Generation
- Generate sine wave samples
- Support configurable duration (default 300ms)
- Output 16-bit signed PCM samples
- Sample rate: 44100Hz

### FR-5: WAV Output
- Generate valid 44-byte RIFF/WAV header
- Support 16-bit mono PCM format
- Little-endian byte order
- Concatenate header with sample data

### FR-6: Command Line Interface
- Read moves from STDIN (space-separated)
- Output WAV data to STDOUT
- Usage: `echo "e4 e5 Nf3 Nc6" | chesswav > game.wav`

## Non-Functional Requirements

### NFR-1: Pure Bash Implementation
- No external dependencies
- Works on bash 4.0+
- No compiled binaries
- Portable across Unix-like systems

### NFR-2: Performance
- Process 100 moves in under 10 seconds
- Generated audio should be under 1MB for typical games

### NFR-3: Reliability
- Invalid moves should not crash the program
- Output valid WAV even with parsing errors

## Technical Constraints

1. **Language**: Pure bash (no awk, sed, perl, python)
2. **Audio**: 44100Hz sample rate, 16-bit PCM, mono
3. **Math**: Integer arithmetic only (bash limitation)
4. **Binary Output**: Using `printf '\xNN'` for bytes

## Acceptance Criteria

### AC-1: Basic Pipeline
```
Given a valid sequence of moves
When piped to chesswav
Then a valid WAV file is produced
```

### AC-2: Move Parsing
```
Given the move "e4"
When parsed
Then piece=pawn, destination=e4, capture=false
```

### AC-3: Frequency Mapping
```
Given square "a4"
When mapped to frequency
Then frequency ~ 262Hz (C4)
```

### AC-4: Audio Output
```
Given a 4-move sequence
When converted to audio
Then WAV contains 4 distinct tones
```

### AC-5: Playback
```
Given the generated WAV file
When opened in standard audio player
Then audio plays without errors
```

## Out of Scope (MVP)

- Castling (O-O, O-O-O)
- En passant
- Pawn promotion
- Move disambiguation (Rad1 vs Rfd1)
- Multiple waveform types (only sine)
- ADSR envelope
- PGN file parsing
- Interactive mode
- Check/checkmate annotations
- Piece-specific timbres
- Capture sounds

## Success Metrics

1. `echo "e4 e5 Nf3 Nc6" | ./chesswav > game.wav` produces playable audio
2. All unit tests pass
3. Four distinct notes are audible (G, G, A, E approximately)
