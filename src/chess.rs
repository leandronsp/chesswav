#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square {
    pub file: u8,
    pub rank: u8,
}

impl Square {
    pub fn file_char(&self) -> char {
        (b'a' + self.file) as char
    }

    pub fn rank_num(&self) -> u8 {
        self.rank + 1
    }

    pub fn to_index(&self) -> usize {
        (self.rank as usize) * 8 + (self.file as usize)
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file_char(), self.rank_num())
    }
}

#[derive(Debug, PartialEq)]
pub struct Move {
    pub piece: PieceKind,
    pub dest: Square,
    pub capture: bool,
}

impl Move {
    pub fn parse(input: &str) -> Option<Move> {
        let mut s: String = input
            .chars()
            .filter(|c| !matches!(c, '+' | '#' | '!' | '?'))
            .collect();

        let capture = s.contains('x');
        if capture {
            s = s.replace('x', "");
        }

        if s.len() < 2 {
            return None;
        }

        let first = s.chars().next()?;
        let piece = match first {
            'K' => PieceKind::King,
            'Q' => PieceKind::Queen,
            'R' => PieceKind::Rook,
            'B' => PieceKind::Bishop,
            'N' => PieceKind::Knight,
            _ => PieceKind::Pawn,
        };

        let dest_str = &s[s.len() - 2..];
        let mut chars = dest_str.chars();
        let file_char = chars.next()?;
        let rank_char = chars.next()?;

        if !('a'..='h').contains(&file_char) {
            return None;
        }
        let file = (file_char as u8) - b'a';

        let rank_num = rank_char.to_digit(10)?;
        if !(1..=8).contains(&rank_num) {
            return None;
        }
        let rank = (rank_num - 1) as u8;

        let dest = Square { file, rank };
        Some(Move { piece, dest, capture })
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
