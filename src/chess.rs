//! Chess domain types - pieces, squares, and moves.
//!
//! # Algebraic Notation
//!
//! ```text
//! Piece letters: K=King, Q=Queen, R=Rook, B=Bishop, N=Knight, (none)=Pawn
//! Squares: file (a-h) + rank (1-8), e.g., "e4", "Nf3"
//! Capture: "x" between piece and destination, e.g., "Bxc6"
//! Annotations: "+" (check), "#" (checkmate), "!", "?" (ignored)
//! ```
//!
//! # Examples
//!
//! ```text
//! "e4"   → Pawn to e4
//! "Nf3"  → Knight to f3
//! "Bxc6" → Bishop captures on c6
//! "Qh5+" → Queen to h5 with check
//! ```

/// Piece color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

/// Piece type, identified by letter in algebraic notation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceKind {
    King,   // K
    Queen,  // Q
    Rook,   // R
    Bishop, // B
    Knight, // N
    Pawn,   // (no letter)
}

/// A chess piece with type and color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

/// A board square with file (column a-h) and rank (row 1-8).
///
/// Internally stored as 0-indexed: file 0-7, rank 0-7.
///
/// ```text
///   8 │ rank=7
///   7 │ rank=6
///   . │ ...
///   2 │ rank=1
///   1 │ rank=0
///     └────────
///       a b c d e f g h
///       0 1 2 3 4 5 6 7  (file)
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square {
    pub file: u8, // 0=a, 1=b, ..., 7=h
    pub rank: u8, // 0=rank1, 1=rank2, ..., 7=rank8
}

impl Square {
    /// Returns file as character: 0→'a', 7→'h'.
    pub fn file_char(&self) -> char {
        (b'a' + self.file) as char
    }

    /// Returns rank as number: 0→1, 7→8.
    pub fn rank_num(&self) -> u8 {
        self.rank + 1
    }

    /// Converts to linear index 0-63 for board array.
    ///
    /// Formula: `rank * 8 + file`
    ///
    /// ```text
    /// a1=0,  b1=1,  ..., h1=7
    /// a2=8,  b2=9,  ..., h2=15
    /// ...
    /// a8=56, b8=57, ..., h8=63
    /// ```
    pub fn to_index(&self) -> usize {
        (self.rank as usize) * 8 + (self.file as usize)
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file_char(), self.rank_num())
    }
}

/// A chess move parsed from algebraic notation.
///
/// ```text
/// "Bxc6" → Move { piece: Bishop, dest: c6, capture: true }
/// "e4"   → Move { piece: Pawn, dest: e4, capture: false }
/// ```
#[derive(Debug, PartialEq)]
pub struct Move {
    pub piece: PieceKind,
    pub dest: Square,
    pub capture: bool,
}

impl Move {
    /// Parses algebraic notation into a Move.
    ///
    /// Returns `None` for invalid notation.
    ///
    /// # Examples
    ///
    /// ```text
    /// "e4"    → Some(Move { piece: Pawn, dest: e4, capture: false })
    /// "Nf3"   → Some(Move { piece: Knight, dest: f3, capture: false })
    /// "Bxc6"  → Some(Move { piece: Bishop, dest: c6, capture: true })
    /// "Qh5+"  → Some(Move { piece: Queen, dest: h5, capture: false })
    /// "invalid" → None
    /// ```
    pub fn parse(input: &str) -> Option<Move> {
        // Remove annotations (+, #, !, ?)
        let mut s: String = input
            .chars()
            .filter(|c| !matches!(c, '+' | '#' | '!' | '?'))
            .collect();

        // Check for capture and remove 'x'
        let capture = s.contains('x');
        if capture {
            s = s.replace('x', "");
        }

        if s.len() < 2 {
            return None;
        }

        // First char determines piece (uppercase = piece, lowercase = pawn)
        let first = s.chars().next()?;
        let piece = match first {
            'K' => PieceKind::King,
            'Q' => PieceKind::Queen,
            'R' => PieceKind::Rook,
            'B' => PieceKind::Bishop,
            'N' => PieceKind::Knight,
            _ => PieceKind::Pawn,
        };

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
            piece,
            dest: Square { file, rank },
            capture,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_file_char_a() {
        let sq = Square { file: 0, rank: 0 };
        assert_eq!(sq.file_char(), 'a');
    }

    #[test]
    fn square_file_char_h() {
        let sq = Square { file: 7, rank: 0 };
        assert_eq!(sq.file_char(), 'h');
    }

    #[test]
    fn square_rank_num_1() {
        let sq = Square { file: 0, rank: 0 };
        assert_eq!(sq.rank_num(), 1);
    }

    #[test]
    fn square_rank_num_8() {
        let sq = Square { file: 0, rank: 7 };
        assert_eq!(sq.rank_num(), 8);
    }

    #[test]
    fn square_to_index_a1() {
        let sq = Square { file: 0, rank: 0 };
        assert_eq!(sq.to_index(), 0);
    }

    #[test]
    fn square_to_index_h8() {
        let sq = Square { file: 7, rank: 7 };
        assert_eq!(sq.to_index(), 63);
    }

    #[test]
    fn square_to_index_e4() {
        let sq = Square { file: 4, rank: 3 };
        assert_eq!(sq.to_index(), 28);
    }

    #[test]
    fn square_display() {
        let sq = Square { file: 4, rank: 3 };
        assert_eq!(format!("{}", sq), "e4");
    }

    #[test]
    fn move_pawn_e4() {
        let m = Move::parse("e4").unwrap();
        assert_eq!(m.piece, PieceKind::Pawn);
        assert_eq!(m.dest, Square { file: 4, rank: 3 });
        assert!(!m.capture);
    }

    #[test]
    fn move_pawn_d5() {
        let m = Move::parse("d5").unwrap();
        assert_eq!(m.piece, PieceKind::Pawn);
        assert_eq!(m.dest, Square { file: 3, rank: 4 });
        assert!(!m.capture);
    }

    #[test]
    fn move_knight() {
        let m = Move::parse("Nf3").unwrap();
        assert_eq!(m.piece, PieceKind::Knight);
        assert_eq!(m.dest, Square { file: 5, rank: 2 });
        assert!(!m.capture);
    }

    #[test]
    fn move_bishop() {
        let m = Move::parse("Bb5").unwrap();
        assert_eq!(m.piece, PieceKind::Bishop);
        assert_eq!(m.dest, Square { file: 1, rank: 4 });
    }

    #[test]
    fn move_queen() {
        let m = Move::parse("Qh4").unwrap();
        assert_eq!(m.piece, PieceKind::Queen);
        assert_eq!(m.dest, Square { file: 7, rank: 3 });
    }

    #[test]
    fn move_rook() {
        let m = Move::parse("Ra1").unwrap();
        assert_eq!(m.piece, PieceKind::Rook);
        assert_eq!(m.dest, Square { file: 0, rank: 0 });
    }

    #[test]
    fn move_king() {
        let m = Move::parse("Ke2").unwrap();
        assert_eq!(m.piece, PieceKind::King);
        assert_eq!(m.dest, Square { file: 4, rank: 1 });
    }

    #[test]
    fn move_piece_capture() {
        let m = Move::parse("Bxc6").unwrap();
        assert_eq!(m.piece, PieceKind::Bishop);
        assert_eq!(m.dest, Square { file: 2, rank: 5 });
        assert!(m.capture);
    }

    #[test]
    fn move_pawn_capture() {
        let m = Move::parse("exd5").unwrap();
        assert_eq!(m.piece, PieceKind::Pawn);
        assert_eq!(m.dest, Square { file: 3, rank: 4 });
        assert!(m.capture);
    }

    #[test]
    fn move_queen_capture() {
        let m = Move::parse("Qxf7").unwrap();
        assert_eq!(m.piece, PieceKind::Queen);
        assert_eq!(m.dest, Square { file: 5, rank: 6 });
        assert!(m.capture);
    }

    #[test]
    fn move_check_annotation() {
        let m = Move::parse("Qh5+").unwrap();
        assert_eq!(m.piece, PieceKind::Queen);
        assert_eq!(m.dest, Square { file: 7, rank: 4 });
    }

    #[test]
    fn move_checkmate_annotation() {
        let m = Move::parse("Qxf7#").unwrap();
        assert_eq!(m.piece, PieceKind::Queen);
        assert_eq!(m.dest, Square { file: 5, rank: 6 });
        assert!(m.capture);
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
