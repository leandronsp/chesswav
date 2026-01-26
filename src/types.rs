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
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square {
    pub file: u8,
    pub rank: u8,
}

impl Square {
    pub fn parse(s: &str) -> Option<Square> {
        let mut chars = s.chars();
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

        Some(Square { file, rank })
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_e4() {
        let sq = Square::parse("e4").unwrap();
        assert_eq!(sq.file, 4);
        assert_eq!(sq.rank, 3);
    }

    #[test]
    fn parse_a1() {
        let sq = Square::parse("a1").unwrap();
        assert_eq!(sq.file, 0);
        assert_eq!(sq.rank, 0);
    }

    #[test]
    fn parse_h8() {
        let sq = Square::parse("h8").unwrap();
        assert_eq!(sq.file, 7);
        assert_eq!(sq.rank, 7);
    }

    #[test]
    fn parse_invalid_file() {
        assert!(Square::parse("i4").is_none());
    }

    #[test]
    fn parse_invalid_rank() {
        assert!(Square::parse("e9").is_none());
        assert!(Square::parse("e0").is_none());
    }

    #[test]
    fn file_char_a() {
        let sq = Square { file: 0, rank: 0 };
        assert_eq!(sq.file_char(), 'a');
    }

    #[test]
    fn file_char_h() {
        let sq = Square { file: 7, rank: 0 };
        assert_eq!(sq.file_char(), 'h');
    }

    #[test]
    fn rank_num_1() {
        let sq = Square { file: 0, rank: 0 };
        assert_eq!(sq.rank_num(), 1);
    }

    #[test]
    fn rank_num_8() {
        let sq = Square { file: 0, rank: 7 };
        assert_eq!(sq.rank_num(), 8);
    }

    #[test]
    fn to_index_a1() {
        let sq = Square::parse("a1").unwrap();
        assert_eq!(sq.to_index(), 0);
    }

    #[test]
    fn to_index_h8() {
        let sq = Square::parse("h8").unwrap();
        assert_eq!(sq.to_index(), 63);
    }

    #[test]
    fn to_index_e4() {
        let sq = Square::parse("e4").unwrap();
        assert_eq!(sq.to_index(), 28);
    }

    #[test]
    fn display() {
        let sq = Square::parse("e4").unwrap();
        assert_eq!(format!("{}", sq), "e4");
    }
}
