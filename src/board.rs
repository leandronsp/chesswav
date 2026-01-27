//! Chess board representation with piece placement.
//!
//! # Board Layout
//!
//! ```text
//!   8 │ r  n  b  q  k  b  n  r   ← Black back rank (index 56-63)
//!   7 │ p  p  p  p  p  p  p  p   ← Black pawns     (index 48-55)
//!   6 │ .  .  .  .  .  .  .  .
//!   5 │ .  .  .  .  .  .  .  .
//!   4 │ .  .  .  .  .  .  .  .
//!   3 │ .  .  .  .  .  .  .  .
//!   2 │ P  P  P  P  P  P  P  P   ← White pawns     (index 8-15)
//!   1 │ R  N  B  Q  K  B  N  R   ← White back rank (index 0-7)
//!     └─────────────────────────
//!       a  b  c  d  e  f  g  h
//! ```
//!
//! # Index Mapping
//!
//! Linear index = rank * 8 + file
//!
//! ```text
//! a1=0,  b1=1,  c1=2,  d1=3,  e1=4,  f1=5,  g1=6,  h1=7
//! a2=8,  b2=9,  ...
//! a8=56, b8=57, c8=58, d8=59, e8=60, f8=61, g8=62, h8=63
//! ```

use crate::chess::{Color, Piece, PieceKind, Square};

/// Starting indices for each rank (rank * 8)
const RANK_1: usize = 0;  // White back rank
const RANK_2: usize = 8;  // White pawns
const RANK_7: usize = 48; // Black pawns
const RANK_8: usize = 56; // Black back rank

/// A chess board with 64 squares.
///
/// Each square is `Option<Piece>`:
/// - `Some(Piece)` = occupied
/// - `None` = empty
pub struct Board {
    squares: [Option<Piece>; 64],
}

impl Board {
    /// Creates a board with standard starting position.
    ///
    /// ```text
    /// Back rank order: R N B Q K B N R
    ///                  0 1 2 3 4 5 6 7 (file index)
    /// ```
    pub fn new() -> Self {
        let mut squares = [None; 64];

        // Back rank pieces: R N B Q K B N R
        let back_rank = [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Queen,
            PieceKind::King,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ];

        // Place back rank pieces for both colors
        for (file, &kind) in back_rank.iter().enumerate() {
            squares[RANK_1 + file] = Some(Piece { kind, color: Color::White });
            squares[RANK_8 + file] = Some(Piece { kind, color: Color::Black });
        }

        // Place pawns on ranks 2 and 7
        for file in 0..8 {
            squares[RANK_2 + file] = Some(Piece { kind: PieceKind::Pawn, color: Color::White });
            squares[RANK_7 + file] = Some(Piece { kind: PieceKind::Pawn, color: Color::Black });
        }

        Board { squares }
    }

    /// Gets the piece at a square, if any.
    pub fn get(&self, square: &Square) -> Option<Piece> {
        self.squares[square.to_index()]
    }

    /// Sets a square to a piece or empty (`None`).
    pub fn set(&mut self, square: &Square, piece: Option<Piece>) {
        self.squares[square.to_index()] = piece;
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A1: Square = Square { file: 0, rank: 0 };
    const B1: Square = Square { file: 1, rank: 0 };
    const C1: Square = Square { file: 2, rank: 0 };
    const D1: Square = Square { file: 3, rank: 0 };
    const E1: Square = Square { file: 4, rank: 0 };
    const E2: Square = Square { file: 4, rank: 1 };
    const E4: Square = Square { file: 4, rank: 3 };
    const D5: Square = Square { file: 3, rank: 4 };
    const E7: Square = Square { file: 4, rank: 6 };
    const A8: Square = Square { file: 0, rank: 7 };
    const D8: Square = Square { file: 3, rank: 7 };
    const E8: Square = Square { file: 4, rank: 7 };

    #[test]
    fn init_white_king() {
        let b = Board::new();
        let p = b.get(&E1).unwrap();
        assert_eq!(p.kind, PieceKind::King);
        assert_eq!(p.color, Color::White);
    }

    #[test]
    fn init_white_queen() {
        let b = Board::new();
        let p = b.get(&D1).unwrap();
        assert_eq!(p.kind, PieceKind::Queen);
        assert_eq!(p.color, Color::White);
    }

    #[test]
    fn init_white_rook() {
        let b = Board::new();
        let p = b.get(&A1).unwrap();
        assert_eq!(p.kind, PieceKind::Rook);
        assert_eq!(p.color, Color::White);
    }

    #[test]
    fn init_white_knight() {
        let b = Board::new();
        let p = b.get(&B1).unwrap();
        assert_eq!(p.kind, PieceKind::Knight);
        assert_eq!(p.color, Color::White);
    }

    #[test]
    fn init_white_bishop() {
        let b = Board::new();
        let p = b.get(&C1).unwrap();
        assert_eq!(p.kind, PieceKind::Bishop);
        assert_eq!(p.color, Color::White);
    }

    #[test]
    fn init_white_pawn() {
        let b = Board::new();
        let p = b.get(&E2).unwrap();
        assert_eq!(p.kind, PieceKind::Pawn);
        assert_eq!(p.color, Color::White);
    }

    #[test]
    fn init_black_king() {
        let b = Board::new();
        let p = b.get(&E8).unwrap();
        assert_eq!(p.kind, PieceKind::King);
        assert_eq!(p.color, Color::Black);
    }

    #[test]
    fn init_black_queen() {
        let b = Board::new();
        let p = b.get(&D8).unwrap();
        assert_eq!(p.kind, PieceKind::Queen);
        assert_eq!(p.color, Color::Black);
    }

    #[test]
    fn init_black_rook() {
        let b = Board::new();
        let p = b.get(&A8).unwrap();
        assert_eq!(p.kind, PieceKind::Rook);
        assert_eq!(p.color, Color::Black);
    }

    #[test]
    fn init_black_pawn() {
        let b = Board::new();
        let p = b.get(&E7).unwrap();
        assert_eq!(p.kind, PieceKind::Pawn);
        assert_eq!(p.color, Color::Black);
    }

    #[test]
    fn empty_square() {
        let b = Board::new();
        assert!(b.get(&E4).is_none());
        assert!(b.get(&D5).is_none());
    }

    #[test]
    fn set_piece() {
        let mut b = Board::new();
        let pawn = Piece { kind: PieceKind::Pawn, color: Color::White };
        b.set(&E4, Some(pawn));
        let p = b.get(&E4).unwrap();
        assert_eq!(p.kind, PieceKind::Pawn);
    }

    #[test]
    fn clear_square() {
        let mut b = Board::new();
        b.set(&E2, None);
        assert!(b.get(&E2).is_none());
    }
}
