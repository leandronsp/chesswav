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

impl Square {
    fn parse(file_char: char, rank_char: char) -> Option<Square> {
        let file = Self::parse_file(file_char)?;
        let rank = Self::parse_rank(rank_char)?;
        Some(Square { file, rank })
    }

    fn parse_file(c: char) -> Option<u8> {
        Self::validate_file(c)?;
        Some((c as u8) - b'a')
    }

    fn parse_rank(c: char) -> Option<u8> {
        let rank_num = c.to_digit(10)?;
        Self::validate_rank(rank_num)?;
        Some((rank_num - 1) as u8)
    }

    fn validate_file(c: char) -> Option<()> {
        if ('a'..='h').contains(&c) { Some(()) } else { None }
    }

    fn validate_rank(rank_num: u32) -> Option<()> {
        if (1..=8).contains(&rank_num) { Some(()) } else { None }
    }
}

/// A chess move parsed from algebraic notation.
#[derive(Debug, PartialEq)]
pub struct Move {
    pub dest: Square,
}

impl Move {
    /// Parses algebraic notation into a Move.
    /// E.g "Ne4" is parsed into:
    ///     Move { dest: Square { file: 4, rank: 3 } }
    /// Returns `None` for invalid notation.
    pub fn parse(input: &str) -> Option<Move> {
        let clean = Self::strip_annotations(input);
        let (file_char, rank_char) = Self::extract_destination(&clean)?;
        let dest = Square::parse(file_char, rank_char)?;
        Some(Move { dest })
    }

    fn strip_annotations(input: &str) -> String {
        input
            .chars()
            .filter(|c| !matches!(c, '+' | '#' | '!' | '?' | 'x'))
            .collect()
    }

    fn extract_destination(s: &str) -> Option<(char, char)> {
        Self::validate_length(s)?;
        let dest_str = &s[s.len() - 2..];
        let mut chars = dest_str.chars();
        Some((chars.next()?, chars.next()?))
    }

    fn validate_length(s: &str) -> Option<()> {
        if s.len() >= 2 { Some(()) } else { None }
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
