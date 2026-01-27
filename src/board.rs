use crate::chess::{Color, Piece, PieceKind, Square};

const RANK_1: usize = 0;
const RANK_2: usize = 8;
const RANK_7: usize = 48;
const RANK_8: usize = 56;

pub struct Board {
    squares: [Option<Piece>; 64],
}

impl Board {
    pub fn new() -> Self {
        let mut squares = [None; 64];

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

        for (file, &kind) in back_rank.iter().enumerate() {
            squares[RANK_1 + file] = Some(Piece { kind, color: Color::White });
            squares[RANK_8 + file] = Some(Piece { kind, color: Color::Black });
        }

        for file in 0..8 {
            squares[RANK_2 + file] = Some(Piece { kind: PieceKind::Pawn, color: Color::White });
            squares[RANK_7 + file] = Some(Piece { kind: PieceKind::Pawn, color: Color::Black });
        }

        Board { squares }
    }

    pub fn get(&self, square: &Square) -> Option<Piece> {
        self.squares[square.to_index()]
    }

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
