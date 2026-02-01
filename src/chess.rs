//! Chess domain types - squares and moves.
//!
//! # Algebraic Notation
//!
//! ```text
//! Squares: file (a-h) + rank (1-8), e.g., "e4", "Nf3"
//! Piece letters: K, Q, R, B, N (pawn has no letter)
//! Capture: "x", Annotations: "+", "#", "!", "?" (all ignored)
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
}

impl Piece {
    fn from_char(c: char) -> Option<Piece> {
        match c {
            'N' => Some(Piece::Knight),
            'R' => Some(Piece::Rook),
            'B' => Some(Piece::Bishop),
            'Q' => Some(Piece::Queen),
            'K' => Some(Piece::King),
            _ => None,
        }
    }
}

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
    pub piece: Piece,
    pub dest: Square,
}

impl Move {
    /// Parses algebraic notation into a Move.
    /// move_index determines turn: even = white (rank 0), odd = black (rank 7).
    pub fn parse(input: &str, move_index: usize) -> Option<Move> {
        let clean = Self::strip_annotations(input);
        let rank = if move_index % 2 == 0 { 0 } else { 7 };

        if let Some(m) = Self::parse_castling(&clean, rank) {
            return Some(m);
        }

        let first_char = clean.chars().next()?;
        let piece = Piece::from_char(first_char).unwrap_or(Piece::Pawn);
        let (file_char, rank_char) = Self::extract_destination(&clean)?;
        let dest = Square::parse(file_char, rank_char)?;

        Some(Move { piece, dest })
    }

    fn parse_castling(clean: &str, rank: u8) -> Option<Move> {
        match clean {
            "OO" => Some(Move { piece: Piece::King, dest: Square { file: 6, rank } }),
            "OOO" => Some(Move { piece: Piece::King, dest: Square { file: 2, rank } }),
            _ => None,
        }
    }

    fn strip_annotations(input: &str) -> String {
        input
            .chars()
            .filter(|c| !matches!(c, '+' | '#' | '!' | '?' | 'x' | '-'))
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
        let m = Move::parse("e4", 0).unwrap();
        assert_eq!(m.piece, Piece::Pawn);
        assert_eq!(m.dest, Square { file: 4, rank: 3 });
    }

    #[test]
    fn move_knight() {
        let m = Move::parse("Nf3", 0).unwrap();
        assert_eq!(m.piece, Piece::Knight);
        assert_eq!(m.dest, Square { file: 5, rank: 2 });
    }

    #[test]
    fn move_capture() {
        let m = Move::parse("Bxc6", 0).unwrap();
        assert_eq!(m.piece, Piece::Bishop);
        assert_eq!(m.dest, Square { file: 2, rank: 5 });
    }

    #[test]
    fn move_bishop() {
        let m = Move::parse("Bb5", 0).unwrap();
        assert_eq!(m.piece, Piece::Bishop);
        assert_eq!(m.dest, Square { file: 1, rank: 4 });
    }

    #[test]
    fn move_queen() {
        let m = Move::parse("Qh5+", 0).unwrap();
        assert_eq!(m.piece, Piece::Queen);
        assert_eq!(m.dest, Square { file: 7, rank: 4 });
    }

    #[test]
    fn move_king() {
        let m = Move::parse("Kf1", 0).unwrap();
        assert_eq!(m.piece, Piece::King);
        assert_eq!(m.dest, Square { file: 5, rank: 0 });
    }

    #[test]
    fn castling_kingside_white() {
        let m = Move::parse("O-O", 0).unwrap();
        assert_eq!(m.piece, Piece::King);
        assert_eq!(m.dest, Square { file: 6, rank: 0 });
    }

    #[test]
    fn castling_kingside_black() {
        let m = Move::parse("O-O", 1).unwrap();
        assert_eq!(m.piece, Piece::King);
        assert_eq!(m.dest, Square { file: 6, rank: 7 });
    }

    #[test]
    fn castling_queenside_white() {
        let m = Move::parse("O-O-O", 0).unwrap();
        assert_eq!(m.piece, Piece::King);
        assert_eq!(m.dest, Square { file: 2, rank: 0 });
    }

    #[test]
    fn castling_queenside_black() {
        let m = Move::parse("O-O-O", 1).unwrap();
        assert_eq!(m.piece, Piece::King);
        assert_eq!(m.dest, Square { file: 2, rank: 7 });
    }

    #[test]
    fn move_rook() {
        let m = Move::parse("Rad1", 0).unwrap();
        assert_eq!(m.piece, Piece::Rook);
        assert_eq!(m.dest, Square { file: 3, rank: 0 });
    }

    #[test]
    fn move_invalid_file() {
        assert!(Move::parse("Ni4", 0).is_none());
    }

    #[test]
    fn move_invalid_rank() {
        assert!(Move::parse("Ne9", 0).is_none());
        assert!(Move::parse("Ne0", 0).is_none());
    }
}
