//! Chess domain types - squares and moves.
//!
//! # Algebraic Notation
//!
//! ```text
//! Squares: file (a-h) + rank (1-8), e.g., "e4", "Nf3"
//! Piece letters: K, Q, R, B, N (pawn has no letter)
//! Capture: "x", Annotations: "+", "#", "!", "?" (all ignored)
//! ```

/// A board square with file (column a-h) and rank (row 1-8).
///
/// Internally stored as 0-indexed: file 0-7, rank 0-7.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square {
    pub file: u8, // 0=a, 1=b, ..., 7=h
    pub rank: u8, // 0=rank1, 1=rank2, ..., 7=rank8
}

/// A chess move parsed from algebraic notation.
#[derive(Debug, PartialEq)]
pub struct Move {
    pub dest: Square,
}

impl Move {
    /// Parses algebraic notation into a Move.
    ///
    /// Returns `None` for invalid notation.
    pub fn parse(input: &str) -> Option<Move> {
        // Remove annotations and capture marker
        let s: String = input
            .chars()
            .filter(|c| !matches!(c, '+' | '#' | '!' | '?' | 'x'))
            .collect();

        if s.len() < 2 {
            return None;
        }

        // Last two chars are destination square
        let dest_str = &s[s.len() - 2..];
        let mut chars = dest_str.chars();
        let file_char = chars.next()?;
        let rank_char = chars.next()?;

        // Validate and convert file (a-h → 0-7)
        if !('a'..='h').contains(&file_char) {
            return None;
        }
        let file = (file_char as u8) - b'a';

        // Validate and convert rank (1-8 → 0-7)
        let rank_num = rank_char.to_digit(10)?;
        if !(1..=8).contains(&rank_num) {
            return None;
        }
        let rank = (rank_num - 1) as u8;

        Some(Move {
            dest: Square { file, rank },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_pawn_e4() {
        let m = Move::parse("e4").unwrap();
        assert_eq!(m.dest, Square { file: 4, rank: 3 });
    }

    #[test]
    fn move_knight() {
        let m = Move::parse("Nf3").unwrap();
        assert_eq!(m.dest, Square { file: 5, rank: 2 });
    }

    #[test]
    fn move_capture() {
        let m = Move::parse("Bxc6").unwrap();
        assert_eq!(m.dest, Square { file: 2, rank: 5 });
    }

    #[test]
    fn move_with_annotation() {
        let m = Move::parse("Qh5+").unwrap();
        assert_eq!(m.dest, Square { file: 7, rank: 4 });
    }

    #[test]
    fn move_invalid_file() {
        assert!(Move::parse("Ni4").is_none());
    }

    #[test]
    fn move_invalid_rank() {
        assert!(Move::parse("Ne9").is_none());
        assert!(Move::parse("Ne0").is_none());
    }
}
