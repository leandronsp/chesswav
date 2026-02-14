use std::fmt;

use crate::chess::{Piece, Square};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone)]
pub struct Board {
    squares: [[Option<(Piece, Color)>; 8]; 8],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        let mut squares = [[None; 8]; 8];

        let back_rank = [
            Piece::Rook,
            Piece::Knight,
            Piece::Bishop,
            Piece::Queen,
            Piece::King,
            Piece::Bishop,
            Piece::Knight,
            Piece::Rook,
        ];

        for (file, &piece) in back_rank.iter().enumerate() {
            squares[0][file] = Some((piece, Color::White));
            squares[1][file] = Some((Piece::Pawn, Color::White));
            squares[6][file] = Some((Piece::Pawn, Color::Black));
            squares[7][file] = Some((piece, Color::Black));
        }

        Board { squares }
    }

    pub fn get(&self, file: u8, rank: u8) -> Option<(Piece, Color)> {
        self.squares[rank as usize][file as usize]
    }

    fn set(&mut self, file: u8, rank: u8, piece: Option<(Piece, Color)>) {
        self.squares[rank as usize][file as usize] = piece;
    }

    pub fn apply_move(&mut self, m: &ParsedMove) {
        let piece_on_origin = self.get(m.origin.file, m.origin.rank);
        self.set(m.origin.file, m.origin.rank, None);

        if let Some(promo) = m.promotion {
            let color = piece_on_origin.map(|(_, c)| c).unwrap_or(Color::White);
            self.set(m.dest.file, m.dest.rank, Some((promo, color)));
        } else {
            self.set(m.dest.file, m.dest.rank, piece_on_origin);
        }

        if let Some((rook_from, rook_to)) = m.castling_rook {
            let rook = self.get(rook_from.file, rook_from.rank);
            self.set(rook_from.file, rook_from.rank, None);
            self.set(rook_to.file, rook_to.rank, rook);
        }
    }

    pub fn find_origin(
        &self,
        piece: Piece,
        dest: &Square,
        color: Color,
        file_hint: Option<u8>,
        rank_hint: Option<u8>,
    ) -> Option<Square> {
        for rank in 0..8u8 {
            for file in 0..8u8 {
                if let Some((p, c)) = self.get(file, rank) {
                    if p != piece || c != color {
                        continue;
                    }
                    if let Some(fh) = file_hint
                        && file != fh
                    {
                        continue;
                    }
                    if let Some(rh) = rank_hint
                        && rank != rh
                    {
                        continue;
                    }
                    if self.can_reach(piece, color, file, rank, dest) {
                        return Some(Square { file, rank });
                    }
                }
            }
        }
        None
    }

    fn can_reach(&self, piece: Piece, color: Color, file: u8, rank: u8, dest: &Square) -> bool {
        match piece {
            Piece::Pawn => self.pawn_can_reach(color, file, rank, dest),
            Piece::Knight => Self::knight_can_reach(file, rank, dest),
            Piece::Bishop => self.bishop_can_reach(file, rank, dest),
            Piece::Rook => self.rook_can_reach(file, rank, dest),
            Piece::Queen => {
                self.bishop_can_reach(file, rank, dest) || self.rook_can_reach(file, rank, dest)
            }
            Piece::King => Self::king_can_reach(file, rank, dest),
        }
    }

    fn pawn_can_reach(&self, color: Color, file: u8, rank: u8, dest: &Square) -> bool {
        let (direction, start_rank): (i8, u8) = match color {
            Color::White => (1, 1),
            Color::Black => (-1, 6),
        };
        let df = (dest.file as i8) - (file as i8);
        let dr = (dest.rank as i8) - (rank as i8);

        if df == 0 && dr == direction && self.get(dest.file, dest.rank).is_none() {
            return true;
        }
        if df == 0 && dr == 2 * direction && rank == start_rank {
            let mid_rank = (rank as i8 + direction) as u8;
            if self.get(file, mid_rank).is_none() && self.get(dest.file, dest.rank).is_none() {
                return true;
            }
        }
        if df.abs() == 1 && dr == direction {
            return true;
        }
        false
    }

    fn knight_can_reach(file: u8, rank: u8, dest: &Square) -> bool {
        let df = ((dest.file as i8) - (file as i8)).abs();
        let dr = ((dest.rank as i8) - (rank as i8)).abs();
        (df == 2 && dr == 1) || (df == 1 && dr == 2)
    }

    fn bishop_can_reach(&self, file: u8, rank: u8, dest: &Square) -> bool {
        let df = (dest.file as i8) - (file as i8);
        let dr = (dest.rank as i8) - (rank as i8);
        if df.abs() != dr.abs() || df == 0 {
            return false;
        }
        self.path_clear(file, rank, dest, df.signum(), dr.signum())
    }

    fn rook_can_reach(&self, file: u8, rank: u8, dest: &Square) -> bool {
        let df = (dest.file as i8) - (file as i8);
        let dr = (dest.rank as i8) - (rank as i8);
        if (df != 0 && dr != 0) || (df == 0 && dr == 0) {
            return false;
        }
        self.path_clear(file, rank, dest, df.signum(), dr.signum())
    }

    fn king_can_reach(file: u8, rank: u8, dest: &Square) -> bool {
        let df = ((dest.file as i8) - (file as i8)).abs();
        let dr = ((dest.rank as i8) - (rank as i8)).abs();
        df <= 1 && dr <= 1 && (df + dr) > 0
    }

    fn path_clear(&self, file: u8, rank: u8, dest: &Square, df: i8, dr: i8) -> bool {
        let mut f = file as i8 + df;
        let mut r = rank as i8 + dr;
        while f != dest.file as i8 || r != dest.rank as i8 {
            if self.get(f as u8, r as u8).is_some() {
                return false;
            }
            f += df;
            r += dr;
        }
        true
    }
}

fn piece_char(piece: Piece, color: Color) -> char {
    let c = match piece {
        Piece::Pawn => 'P',
        Piece::Knight => 'N',
        Piece::Bishop => 'B',
        Piece::Rook => 'R',
        Piece::Queen => 'Q',
        Piece::King => 'K',
    };
    match color {
        Color::White => c,
        Color::Black => c.to_ascii_lowercase(),
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            write!(f, "  {} |", rank + 1)?;
            for file in 0..8 {
                let ch = match self.squares[rank][file] {
                    Some((piece, color)) => piece_char(piece, color),
                    None => '.',
                };
                write!(f, " {ch}")?;
            }
            writeln!(f)?;
        }
        writeln!(f, "    +----------------")?;
        writeln!(f, "      a b c d e f g h")?;
        Ok(())
    }
}

pub struct ParsedMove {
    pub origin: Square,
    pub dest: Square,
    pub promotion: Option<Piece>,
    pub castling_rook: Option<(Square, Square)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_position_white_pawns() {
        let board = Board::new();
        for file in 0..8 {
            assert_eq!(board.get(file, 1), Some((Piece::Pawn, Color::White)));
        }
    }

    #[test]
    fn initial_position_black_pawns() {
        let board = Board::new();
        for file in 0..8 {
            assert_eq!(board.get(file, 6), Some((Piece::Pawn, Color::Black)));
        }
    }

    #[test]
    fn initial_position_white_back_rank() {
        let board = Board::new();
        assert_eq!(board.get(0, 0), Some((Piece::Rook, Color::White)));
        assert_eq!(board.get(1, 0), Some((Piece::Knight, Color::White)));
        assert_eq!(board.get(2, 0), Some((Piece::Bishop, Color::White)));
        assert_eq!(board.get(3, 0), Some((Piece::Queen, Color::White)));
        assert_eq!(board.get(4, 0), Some((Piece::King, Color::White)));
    }

    #[test]
    fn initial_position_empty_middle() {
        let board = Board::new();
        for rank in 2..6 {
            for file in 0..8 {
                assert_eq!(board.get(file, rank), None);
            }
        }
    }

    #[test]
    fn apply_simple_move() {
        let mut board = Board::new();
        let m = ParsedMove {
            origin: Square { file: 4, rank: 1 },
            dest: Square { file: 4, rank: 3 },
            promotion: None,
            castling_rook: None,
        };
        board.apply_move(&m);
        assert_eq!(board.get(4, 1), None);
        assert_eq!(board.get(4, 3), Some((Piece::Pawn, Color::White)));
    }

    #[test]
    fn apply_castling_kingside_white() {
        let mut board = Board::new();
        board.set(5, 0, None);
        board.set(6, 0, None);
        let m = ParsedMove {
            origin: Square { file: 4, rank: 0 },
            dest: Square { file: 6, rank: 0 },
            promotion: None,
            castling_rook: Some((Square { file: 7, rank: 0 }, Square { file: 5, rank: 0 })),
        };
        board.apply_move(&m);
        assert_eq!(board.get(6, 0), Some((Piece::King, Color::White)));
        assert_eq!(board.get(5, 0), Some((Piece::Rook, Color::White)));
        assert_eq!(board.get(4, 0), None);
        assert_eq!(board.get(7, 0), None);
    }

    #[test]
    fn apply_promotion() {
        let mut board = Board::new();
        board.set(4, 6, Some((Piece::Pawn, Color::White)));
        board.set(4, 7, None);
        let m = ParsedMove {
            origin: Square { file: 4, rank: 6 },
            dest: Square { file: 4, rank: 7 },
            promotion: Some(Piece::Queen),
            castling_rook: None,
        };
        board.apply_move(&m);
        assert_eq!(board.get(4, 7), Some((Piece::Queen, Color::White)));
        assert_eq!(board.get(4, 6), None);
    }

    #[test]
    fn find_origin_pawn_e4() {
        let board = Board::new();
        let dest = Square { file: 4, rank: 3 };
        let origin = board.find_origin(Piece::Pawn, &dest, Color::White, None, None);
        assert_eq!(origin, Some(Square { file: 4, rank: 1 }));
    }

    #[test]
    fn find_origin_knight_f3() {
        let board = Board::new();
        let dest = Square { file: 5, rank: 2 };
        let origin = board.find_origin(Piece::Knight, &dest, Color::White, None, None);
        assert_eq!(origin, Some(Square { file: 6, rank: 0 }));
    }

    #[test]
    fn find_origin_with_file_hint() {
        let mut board = Board::new();
        board.set(0, 3, Some((Piece::Rook, Color::White)));
        board.set(7, 3, Some((Piece::Rook, Color::White)));
        let dest = Square { file: 3, rank: 3 };
        let origin = board.find_origin(Piece::Rook, &dest, Color::White, Some(0), None);
        assert_eq!(origin, Some(Square { file: 0, rank: 3 }));
    }

    #[test]
    fn display_initial_position() {
        let board = Board::new();
        let display = format!("{board}");
        assert!(display.contains("r n b q k b n r"));
        assert!(display.contains("P P P P P P P P"));
        assert!(display.contains("a b c d e f g h"));
    }

    #[test]
    fn pawn_double_push_blocked() {
        let mut board = Board::new();
        board.set(4, 2, Some((Piece::Pawn, Color::Black)));
        let dest = Square { file: 4, rank: 3 };
        let origin = board.find_origin(Piece::Pawn, &dest, Color::White, None, None);
        assert_eq!(origin, None);
    }

    #[test]
    fn bishop_blocked_by_piece() {
        let board = Board::new();
        let dest = Square { file: 0, rank: 2 };
        let origin = board.find_origin(Piece::Bishop, &dest, Color::White, None, None);
        assert_eq!(origin, None);
    }
}
