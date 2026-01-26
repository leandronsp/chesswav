use crate::types::{Color, PieceKind, Square};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

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
            squares[file] = Some(Piece { kind, color: Color::White });
            squares[56 + file] = Some(Piece { kind, color: Color::Black });
        }

        for file in 0..8 {
            squares[8 + file] = Some(Piece { kind: PieceKind::Pawn, color: Color::White });
            squares[48 + file] = Some(Piece { kind: PieceKind::Pawn, color: Color::Black });
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

    fn sq(s: &str) -> Square {
        Square::parse(s).unwrap()
    }

    #[test]
    fn init_white_king() {
        let b = Board::new();
        let p = b.get(&sq("e1")).unwrap();
        assert_eq!(p.kind, PieceKind::King);
        assert_eq!(p.color, Color::White);
    }

    #[test]
    fn init_white_queen() {
        let b = Board::new();
        let p = b.get(&sq("d1")).unwrap();
        assert_eq!(p.kind, PieceKind::Queen);
        assert_eq!(p.color, Color::White);
    }

    #[test]
    fn init_white_rook() {
        let b = Board::new();
        let p = b.get(&sq("a1")).unwrap();
        assert_eq!(p.kind, PieceKind::Rook);
        assert_eq!(p.color, Color::White);
    }

    #[test]
    fn init_white_knight() {
        let b = Board::new();
        let p = b.get(&sq("b1")).unwrap();
        assert_eq!(p.kind, PieceKind::Knight);
        assert_eq!(p.color, Color::White);
    }

    #[test]
    fn init_white_bishop() {
        let b = Board::new();
        let p = b.get(&sq("c1")).unwrap();
        assert_eq!(p.kind, PieceKind::Bishop);
        assert_eq!(p.color, Color::White);
    }

    #[test]
    fn init_white_pawn() {
        let b = Board::new();
        let p = b.get(&sq("e2")).unwrap();
        assert_eq!(p.kind, PieceKind::Pawn);
        assert_eq!(p.color, Color::White);
    }

    #[test]
    fn init_black_king() {
        let b = Board::new();
        let p = b.get(&sq("e8")).unwrap();
        assert_eq!(p.kind, PieceKind::King);
        assert_eq!(p.color, Color::Black);
    }

    #[test]
    fn init_black_queen() {
        let b = Board::new();
        let p = b.get(&sq("d8")).unwrap();
        assert_eq!(p.kind, PieceKind::Queen);
        assert_eq!(p.color, Color::Black);
    }

    #[test]
    fn init_black_rook() {
        let b = Board::new();
        let p = b.get(&sq("a8")).unwrap();
        assert_eq!(p.kind, PieceKind::Rook);
        assert_eq!(p.color, Color::Black);
    }

    #[test]
    fn init_black_pawn() {
        let b = Board::new();
        let p = b.get(&sq("e7")).unwrap();
        assert_eq!(p.kind, PieceKind::Pawn);
        assert_eq!(p.color, Color::Black);
    }

    #[test]
    fn empty_square() {
        let b = Board::new();
        assert!(b.get(&sq("e4")).is_none());
        assert!(b.get(&sq("d5")).is_none());
    }

    #[test]
    fn set_piece() {
        let mut b = Board::new();
        let pawn = Piece { kind: PieceKind::Pawn, color: Color::White };
        b.set(&sq("e4"), Some(pawn));
        let p = b.get(&sq("e4")).unwrap();
        assert_eq!(p.kind, PieceKind::Pawn);
    }

    #[test]
    fn clear_square() {
        let mut b = Board::new();
        b.set(&sq("e2"), None);
        assert!(b.get(&sq("e2")).is_none());
    }
}
