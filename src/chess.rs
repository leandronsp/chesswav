//! Chess domain types - squares and moves.
//!
//! # Algebraic Notation
//!
//! ```text
//! Squares: file (a-h) + rank (1-8), e.g., "e4", "Nf3"
//! Piece letters: K, Q, R, B, N (pawn has no letter)
//! Capture: "x", Annotations: "+", "#", "!", "?" (stripped during parse)
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Threat {
    None,
    Check,
    Checkmate,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Capture {
    None,
    Taken,
}

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
        if ('a'..='h').contains(&c) {
            Some(())
        } else {
            None
        }
    }

    fn validate_rank(rank_num: u32) -> Option<()> {
        if (1..=8).contains(&rank_num) {
            Some(())
        } else {
            None
        }
    }
}

/// A chess move parsed from algebraic notation.
#[derive(Debug, PartialEq)]
pub struct Move {
    pub piece: Piece,
    pub dest: Square,
    pub threat: Threat,
    pub capture: Capture,
    pub promotion: Option<Piece>,
}

impl Move {
    /// Parses algebraic notation into a Move.
    /// move_index determines turn: even = white (rank 0), odd = black (rank 7).
    pub fn parse(input: &str, move_index: usize) -> Option<Move> {
        let threat = match (input.contains('#'), input.contains('+')) {
            (true, _) => Threat::Checkmate,
            (_, true) => Threat::Check,
            _ => Threat::None,
        };
        let capture = if input.contains('x') { Capture::Taken } else { Capture::None };
        let promotion = Self::parse_promotion(input);
        let clean = Self::strip_annotations(input);
        let rank = if move_index % 2 == 0 { 0 } else { 7 };

        if let Some(m) = Self::parse_castling(&clean, rank, threat, capture) {
            return Some(m);
        }

        let first_char = clean.chars().next()?;
        let piece = Piece::from_char(first_char).unwrap_or(Piece::Pawn);
        let (file_char, rank_char) = Self::extract_destination(&clean)?;
        let dest = Square::parse(file_char, rank_char)?;

        Some(Move { piece, dest, threat, capture, promotion })
    }

    fn parse_castling(clean: &str, rank: u8, threat: Threat, capture: Capture) -> Option<Move> {
        match clean {
            "OO" => Some(Move {
                piece: Piece::King,
                dest: Square { file: 6, rank },
                threat,
                capture,
                promotion: None,
            }),
            "OOO" => Some(Move {
                piece: Piece::King,
                dest: Square { file: 2, rank },
                threat,
                capture,
                promotion: None,
            }),
            _ => None,
        }
    }

    fn parse_promotion(input: &str) -> Option<Piece> {
        let after_eq = input.split('=').nth(1)?;
        Piece::from_char(after_eq.chars().next()?)
    }

    fn strip_annotations(input: &str) -> String {
        input
            .split('=')
            .next()
            .unwrap_or(input)
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
        assert_eq!(m.threat, Threat::None);
        assert_eq!(m.capture, Capture::None);
        assert_eq!(m.promotion, None);
    }

    #[test]
    fn move_knight() {
        let m = Move::parse("Nf3", 0).unwrap();
        assert_eq!(m.piece, Piece::Knight);
        assert_eq!(m.dest, Square { file: 5, rank: 2 });
        assert_eq!(m.threat, Threat::None);
        assert_eq!(m.capture, Capture::None);
        assert_eq!(m.promotion, None);
    }

    #[test]
    fn move_capture() {
        let m = Move::parse("Bxc6", 0).unwrap();
        assert_eq!(m.piece, Piece::Bishop);
        assert_eq!(m.dest, Square { file: 2, rank: 5 });
        assert_eq!(m.threat, Threat::None);
        assert_eq!(m.capture, Capture::Taken);
        assert_eq!(m.promotion, None);
    }

    #[test]
    fn move_bishop() {
        let m = Move::parse("Bb5", 0).unwrap();
        assert_eq!(m.piece, Piece::Bishop);
        assert_eq!(m.dest, Square { file: 1, rank: 4 });
        assert_eq!(m.threat, Threat::None);
        assert_eq!(m.capture, Capture::None);
        assert_eq!(m.promotion, None);
    }

    #[test]
    fn move_queen() {
        let m = Move::parse("Qh5+", 0).unwrap();
        assert_eq!(m.piece, Piece::Queen);
        assert_eq!(m.dest, Square { file: 7, rank: 4 });
        assert_eq!(m.threat, Threat::Check);
        assert_eq!(m.capture, Capture::None);
        assert_eq!(m.promotion, None);
    }

    #[test]
    fn move_king() {
        let m = Move::parse("Kf1", 0).unwrap();
        assert_eq!(m.piece, Piece::King);
        assert_eq!(m.dest, Square { file: 5, rank: 0 });
        assert_eq!(m.threat, Threat::None);
        assert_eq!(m.capture, Capture::None);
        assert_eq!(m.promotion, None);
    }

    #[test]
    fn castling_kingside_white() {
        let m = Move::parse("O-O", 0).unwrap();
        assert_eq!(m.piece, Piece::King);
        assert_eq!(m.dest, Square { file: 6, rank: 0 });
        assert_eq!(m.threat, Threat::None);
        assert_eq!(m.capture, Capture::None);
        assert_eq!(m.promotion, None);
    }

    #[test]
    fn castling_kingside_black() {
        let m = Move::parse("O-O", 1).unwrap();
        assert_eq!(m.piece, Piece::King);
        assert_eq!(m.dest, Square { file: 6, rank: 7 });
        assert_eq!(m.threat, Threat::None);
        assert_eq!(m.capture, Capture::None);
        assert_eq!(m.promotion, None);
    }

    #[test]
    fn castling_queenside_white() {
        let m = Move::parse("O-O-O", 0).unwrap();
        assert_eq!(m.piece, Piece::King);
        assert_eq!(m.dest, Square { file: 2, rank: 0 });
        assert_eq!(m.threat, Threat::None);
        assert_eq!(m.capture, Capture::None);
        assert_eq!(m.promotion, None);
    }

    #[test]
    fn castling_queenside_black() {
        let m = Move::parse("O-O-O", 1).unwrap();
        assert_eq!(m.piece, Piece::King);
        assert_eq!(m.dest, Square { file: 2, rank: 7 });
        assert_eq!(m.threat, Threat::None);
        assert_eq!(m.capture, Capture::None);
        assert_eq!(m.promotion, None);
    }

    #[test]
    fn move_rook() {
        let m = Move::parse("Rad1", 0).unwrap();
        assert_eq!(m.piece, Piece::Rook);
        assert_eq!(m.dest, Square { file: 3, rank: 0 });
        assert_eq!(m.threat, Threat::None);
        assert_eq!(m.capture, Capture::None);
        assert_eq!(m.promotion, None);
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

    #[test]
    fn check_detected() {
        let m = Move::parse("Nf3+", 0).unwrap();
        assert_eq!(m.threat, Threat::Check);
    }

    #[test]
    fn no_check_by_default() {
        let m = Move::parse("Nf3", 0).unwrap();
        assert_eq!(m.threat, Threat::None);
    }

    #[test]
    fn check_on_pawn_move() {
        let m = Move::parse("e4+", 0).unwrap();
        assert_eq!(m.piece, Piece::Pawn);
        assert_eq!(m.threat, Threat::Check);
    }

    #[test]
    fn capture_detected() {
        let m = Move::parse("Nxe5", 0).unwrap();
        assert_eq!(m.capture, Capture::Taken);
    }

    #[test]
    fn no_capture_by_default() {
        let m = Move::parse("Ne5", 0).unwrap();
        assert_eq!(m.capture, Capture::None);
    }

    #[test]
    fn capture_with_check() {
        let m = Move::parse("Bxf7+", 0).unwrap();
        assert_eq!(m.piece, Piece::Bishop);
        assert_eq!(m.threat, Threat::Check);
        assert_eq!(m.capture, Capture::Taken);
    }

    #[test]
    fn checkmate_detected() {
        let m = Move::parse("Qf7#", 0).unwrap();
        assert_eq!(m.piece, Piece::Queen);
        assert_eq!(m.dest, Square { file: 5, rank: 6 });
        assert_eq!(m.threat, Threat::Checkmate);
    }

    #[test]
    fn promotion_detected() {
        let m = Move::parse("e8=Q", 0).unwrap();
        assert_eq!(m.piece, Piece::Pawn);
        assert_eq!(m.dest, Square { file: 4, rank: 7 });
        assert_eq!(m.promotion, Some(Piece::Queen));
    }

    #[test]
    fn promotion_with_capture() {
        let m = Move::parse("exd8=Q", 0).unwrap();
        assert_eq!(m.piece, Piece::Pawn);
        assert_eq!(m.dest, Square { file: 3, rank: 7 });
        assert_eq!(m.capture, Capture::Taken);
        assert_eq!(m.promotion, Some(Piece::Queen));
    }

    #[test]
    fn no_promotion_by_default() {
        let m = Move::parse("e4", 0).unwrap();
        assert_eq!(m.promotion, None);
    }

    #[test]
    fn promotion_to_knight() {
        let m = Move::parse("e8=N", 0).unwrap();
        assert_eq!(m.promotion, Some(Piece::Knight));
    }

    #[test]
    fn promotion_to_rook() {
        let m = Move::parse("a1=R", 1).unwrap();
        assert_eq!(m.promotion, Some(Piece::Rook));
    }
}
