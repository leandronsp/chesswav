//! Frequency mapping - converts board squares to musical notes.
//!
//! # Mapping
//!
//! ```text
//! Column → Note (using equal temperament tuning)
//!
//!   a     b     c     d     e     f     g     h
//!   C     D     E     F     G     A     B     C
//!
//! Rank → Octave (rank 4 = 4th octave, reference)
//!
//!   8 │ C8  D8  E8  F8  G8  A8  B8  C9   ← highest
//!   7 │ C7  D7  E7  F7  G7  A7  B7  C8
//!   6 │ C6  D6  E6  F6  G6  A6  B6  C7
//!   5 │ C5  D5  E5  F5  G5  A5  B5  C6
//!   4 │ C4  D4  E4  F4  G4  A4  B4  C5   ← reference (A4=440Hz)
//!   3 │ C3  D3  E3  F3  G3  A3  B3  C4
//!   2 │ C2  D2  E2  F2  G2  A2  B2  C3
//!   1 │ C1  D1  E1  F1  G1  A1  B1  C2   ← lowest
//!     └─────────────────────────────────
//!       a   b   c   d   e   f   g   h
//! ```
//!
//! # Equal Temperament
//!
//! Frequency formula: f = 440 × 2^(semitones_from_A4 / 12)

use crate::engine::chess::Square;

/// A4 = 440 Hz is the international tuning standard.
const A4_FREQ: f64 = 440.0;

/// Semitones in an octave.
const SEMITONES_PER_OCTAVE: i32 = 12;

/// Reference rank for 4th octave (0-indexed).
const REFERENCE_RANK: i32 = 3;

/// Semitones from A for file 'f' (which maps to note A).
const A_SEMITONES_FROM_C: i32 = 9;

/// Semitones from C for each file (a-h → C, D, E, F, G, A, B, C).
const FILE_SEMITONES: [i32; 8] = [
    0,  // a → C
    2,  // b → D
    4,  // c → E
    5,  // d → F
    7,  // e → G
    9,  // f → A
    11, // g → B
    12, // h → C (octave up)
];

/// Converts a board square to its frequency in Hz using equal temperament.
pub fn from_square(square: &Square) -> u32 {
    let semitones = semitones_from_a4(square);
    frequency_from_semitones(semitones)
}

/// Calculates the number of semitones from A4 for a given square.
/// E.g for f4 (file 5, rank 3):
///  - file 5 (f) → 9 semitones from C
///  - rank 3 (4th octave) → 0 semitones
///  - total: 9 + 0 - 9 = 0 semitones
fn semitones_from_a4(square: &Square) -> i32 {
    let file_semitones = FILE_SEMITONES[square.file as usize];
    let octave_diff = (square.rank as i32) - REFERENCE_RANK;
    let rank_semitones = octave_diff * SEMITONES_PER_OCTAVE;

    file_semitones + rank_semitones - A_SEMITONES_FROM_C
}

fn frequency_from_semitones(semitones: i32) -> u32 {
    let freq = A4_FREQ * 2.0_f64.powf(semitones as f64 / SEMITONES_PER_OCTAVE as f64);
    freq.round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a4_is_440() {
        let f4 = Square { file: 5, rank: 3 }; // f4 → A4
        assert_eq!(from_square(&f4), 440);
    }

    #[test]
    fn c4_is_262() {
        let a4 = Square { file: 0, rank: 3 }; // a4 → C4
        assert_eq!(from_square(&a4), 262);
    }

    #[test]
    fn c5_is_523() {
        let h4 = Square { file: 7, rank: 3 }; // h4 → C5
        assert_eq!(from_square(&h4), 523);
    }

    #[test]
    fn c5_from_octave_up() {
        let a5 = Square { file: 0, rank: 4 }; // a5 → C5
        assert_eq!(from_square(&a5), 523);
    }

    #[test]
    fn octave_up_doubles() {
        let a4 = Square { file: 5, rank: 3 }; // f4 → A4
        let a5 = Square { file: 5, rank: 4 }; // f5 → A5
        assert_eq!(from_square(&a4), 440);
        assert_eq!(from_square(&a5), 880);
    }

    #[test]
    fn octave_down_halves() {
        let a4 = Square { file: 5, rank: 3 }; // f4 → A4
        let a3 = Square { file: 5, rank: 2 }; // f3 → A3
        assert_eq!(from_square(&a4), 440);
        assert_eq!(from_square(&a3), 220);
    }

    #[test]
    fn e4_is_g4() {
        let e4 = Square { file: 4, rank: 3 }; // e4 → G4
        assert_eq!(from_square(&e4), 392);
    }

    #[test]
    fn lowest_note() {
        let a1 = Square { file: 0, rank: 0 }; // a1 → C1
        assert_eq!(from_square(&a1), 33);
    }

    #[test]
    fn highest_note() {
        let h8 = Square { file: 7, rank: 7 }; // h8 → C9
        assert_eq!(from_square(&h8), 8372);
    }
}
