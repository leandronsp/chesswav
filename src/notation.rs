use crate::types::{PieceKind, Square};

#[derive(Debug, PartialEq)]
pub struct Move {
    pub piece: PieceKind,
    pub dest: Square,
    pub capture: bool,
}

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
    let dest = Square::parse(dest_str)?;

    Some(Move { piece, dest, capture })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pawn_move_e4() {
        let m = parse("e4").unwrap();
        assert_eq!(m.piece, PieceKind::Pawn);
        assert_eq!(m.dest, Square::parse("e4").unwrap());
        assert!(!m.capture);
    }

    #[test]
    fn pawn_move_d5() {
        let m = parse("d5").unwrap();
        assert_eq!(m.piece, PieceKind::Pawn);
        assert_eq!(m.dest, Square::parse("d5").unwrap());
        assert!(!m.capture);
    }

    #[test]
    fn knight_move() {
        let m = parse("Nf3").unwrap();
        assert_eq!(m.piece, PieceKind::Knight);
        assert_eq!(m.dest, Square::parse("f3").unwrap());
        assert!(!m.capture);
    }

    #[test]
    fn bishop_move() {
        let m = parse("Bb5").unwrap();
        assert_eq!(m.piece, PieceKind::Bishop);
        assert_eq!(m.dest, Square::parse("b5").unwrap());
    }

    #[test]
    fn queen_move() {
        let m = parse("Qh4").unwrap();
        assert_eq!(m.piece, PieceKind::Queen);
        assert_eq!(m.dest, Square::parse("h4").unwrap());
    }

    #[test]
    fn rook_move() {
        let m = parse("Ra1").unwrap();
        assert_eq!(m.piece, PieceKind::Rook);
        assert_eq!(m.dest, Square::parse("a1").unwrap());
    }

    #[test]
    fn king_move() {
        let m = parse("Ke2").unwrap();
        assert_eq!(m.piece, PieceKind::King);
        assert_eq!(m.dest, Square::parse("e2").unwrap());
    }

    #[test]
    fn piece_capture() {
        let m = parse("Bxc6").unwrap();
        assert_eq!(m.piece, PieceKind::Bishop);
        assert_eq!(m.dest, Square::parse("c6").unwrap());
        assert!(m.capture);
    }

    #[test]
    fn pawn_capture() {
        let m = parse("exd5").unwrap();
        assert_eq!(m.piece, PieceKind::Pawn);
        assert_eq!(m.dest, Square::parse("d5").unwrap());
        assert!(m.capture);
    }

    #[test]
    fn queen_capture() {
        let m = parse("Qxf7").unwrap();
        assert_eq!(m.piece, PieceKind::Queen);
        assert_eq!(m.dest, Square::parse("f7").unwrap());
        assert!(m.capture);
    }

    #[test]
    fn check_annotation() {
        let m = parse("Qh5+").unwrap();
        assert_eq!(m.piece, PieceKind::Queen);
        assert_eq!(m.dest, Square::parse("h5").unwrap());
    }

    #[test]
    fn checkmate_annotation() {
        let m = parse("Qxf7#").unwrap();
        assert_eq!(m.piece, PieceKind::Queen);
        assert_eq!(m.dest, Square::parse("f7").unwrap());
        assert!(m.capture);
    }
}
