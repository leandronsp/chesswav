//! Frequency mapping - converts board squares to musical notes.
//!
//! # Mapping
//!
//! ```text
//! Column → Note (4th octave base frequencies in Hz)
//!
//!   a     b     c     d     e     f     g     h
//!   C     D     E     F     G     A     B     C
//!  262   294   330   349   392   440   494   523
//!
//! Rank → Octave (rank 4 = 4th octave, reference)
//!
//!   8 │ C6  D6  E6  F6  G6  A6  B6  C7   ← highest
//!   7 │ C5  D5  E5  F5  G5  A5  B5  C6
//!   6 │ ...
//!   5 │ ...
//!   4 │ C4  D4  E4  F4  G4  A4  B4  C5   ← reference (A4=440Hz)
//!   3 │ C3  D3  E3  F3  G3  A3  B3  C4
//!   2 │ C2  D2  E2  F2  G2  A2  B2  C3
//!   1 │ C1  D1  E1  F1  G1  A1  B1  C2   ← lowest
//!     └─────────────────────────────────
//!       a   b   c   d   e   f   g   h
//! ```
//!
//! # Octave shifting
//!
//! Each octave doubles the frequency:
//! - `freq << 1` = one octave up (×2)
//! - `freq >> 1` = one octave down (÷2)

use crate::chess::Square;

/// Base frequencies for 4th octave (rank 4).
/// A4 = 440 Hz is the international tuning standard.
const BASE_FREQ: [u32; 8] = [
    262, // a → C4
    294, // b → D4
    330, // c → E4
    349, // d → F4
    392, // e → G4
    440, // f → A4 (tuning reference)
    494, // g → B4
    523, // h → C5
];

/// Converts a board square to its frequency in Hz.
///
/// # Examples
///
/// ```text
/// e4 → file=4, rank=3 → BASE_FREQ[4]=392, octave_diff=0 → 392 Hz (G4)
/// a5 → file=0, rank=4 → BASE_FREQ[0]=262, octave_diff=1 → 262<<1 = 524 Hz (C5)
/// f2 → file=5, rank=1 → BASE_FREQ[5]=440, octave_diff=-2 → 440>>2 = 110 Hz (A2)
/// ```
pub fn from_square(square: &Square) -> u32 {
    let base = BASE_FREQ[square.file as usize];
    let octave_diff = (square.rank as i32) - 3; // rank 4 (index 3) is reference

    match octave_diff.cmp(&0) {
        std::cmp::Ordering::Greater => base << octave_diff,
        std::cmp::Ordering::Less => base >> (-octave_diff),
        std::cmp::Ordering::Equal => base,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A2: Square = Square { file: 0, rank: 1 };
    const A3: Square = Square { file: 0, rank: 2 };
    const A4: Square = Square { file: 0, rank: 3 };
    const A5: Square = Square { file: 0, rank: 4 };
    const A6: Square = Square { file: 0, rank: 5 };
    const B4: Square = Square { file: 1, rank: 3 };
    const E4: Square = Square { file: 4, rank: 3 };
    const F4: Square = Square { file: 5, rank: 3 };
    const G4: Square = Square { file: 6, rank: 3 };
    const H4: Square = Square { file: 7, rank: 3 };

    #[test]
    fn a4_c4() {
        assert_eq!(from_square(&A4), 262);
    }

    #[test]
    fn f4_a4() {
        assert_eq!(from_square(&F4), 440);
    }

    #[test]
    fn e4_g4() {
        assert_eq!(from_square(&E4), 392);
    }

    #[test]
    fn b4_d4() {
        assert_eq!(from_square(&B4), 294);
    }

    #[test]
    fn g4_b4() {
        assert_eq!(from_square(&G4), 494);
    }

    #[test]
    fn h4_c5() {
        assert_eq!(from_square(&H4), 523);
    }

    #[test]
    fn octave_up_a5() {
        assert_eq!(from_square(&A5), 524);
    }

    #[test]
    fn octave_down_a3() {
        assert_eq!(from_square(&A3), 131);
    }

    #[test]
    fn two_octaves_up() {
        assert_eq!(from_square(&A6), 1048);
    }

    #[test]
    fn two_octaves_down() {
        assert_eq!(from_square(&A2), 65);
    }
}
