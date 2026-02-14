//! Move disambiguation and hint extraction.
//!
//! Algebraic notation can be ambiguous when multiple pieces of the same type
//! can reach the same square (e.g., `Rad1` vs `Rfd1`). This module extracts
//! file/rank hints from notation and resolves castling moves into fully
//! specified origin-destination pairs.
//!
//! Since we don't yet track full game state (move history, en passant rights,
//! castling availability), disambiguation relies solely on notation hints and
//! the current board position.
//!
//! ## Exported functions
//!
//! - `is_castling` — detects castling notation (`O-O`, `O-O-O`)
//! - `resolve_castling` — converts castling into a `ResolvedMove` with rook movement
//! - `strip_annotations` — removes check/capture/annotation symbols from notation
//! - `extract_hints` — extracts file/rank disambiguation hints from cleaned notation

use crate::board::Color;
use crate::chess::{NotationMove, Piece, ResolvedMove, Square};

pub fn is_castling(notation: &str) -> bool {
    let clean: String = notation
        .chars()
        .filter(|character| !matches!(character, '+' | '#'))
        .collect();
    clean == "O-O" || clean == "O-O-O"
}

pub fn resolve_castling(chess_move: &NotationMove, color: Color) -> Option<ResolvedMove> {
    let rank = match color {
        Color::White => 0,
        Color::Black => 7,
    };

    let kingside = chess_move.dest.file == 6;
    let (rook_from, rook_to) = if kingside {
        (Square { file: 7, rank }, Square { file: 5, rank })
    } else {
        (Square { file: 0, rank }, Square { file: 3, rank })
    };

    Some(ResolvedMove {
        origin: Square { file: 4, rank },
        dest: chess_move.dest,
        promotion: None,
        castling_rook: Some((rook_from, rook_to)),
    })
}

pub fn strip_annotations(notation: &str) -> String {
    notation
        .split('=')
        .next()
        .unwrap_or(notation)
        .chars()
        .filter(|character| !matches!(character, '+' | '#' | '!' | '?' | 'x' | '-'))
        .collect()
}

pub fn extract_hints(clean: &str, piece: Piece) -> (Option<u8>, Option<u8>) {
    if piece == Piece::Pawn {
        return extract_pawn_hints(clean);
    }

    if clean.len() <= 3 {
        return (None, None);
    }

    let disambiguation = &clean[1..clean.len() - 2];
    let mut file_hint = None;
    let mut rank_hint = None;

    for hint_char in disambiguation.chars() {
        if ('a'..='h').contains(&hint_char) {
            file_hint = Some(hint_char as u8 - b'a');
        } else if ('1'..='8').contains(&hint_char) {
            rank_hint = Some(hint_char as u8 - b'1');
        }
    }

    (file_hint, rank_hint)
}

fn extract_pawn_hints(clean: &str) -> (Option<u8>, Option<u8>) {
    if clean.len() > 2 {
        let source_file = clean.chars().next().unwrap();
        if ('a'..='h').contains(&source_file) {
            return (Some(source_file as u8 - b'a'), None);
        }
    }
    (None, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::{Capture, Threat};

    #[test]
    fn kingside_castling_detected() {
        assert!(is_castling("O-O"));
    }

    #[test]
    fn queenside_castling_detected() {
        assert!(is_castling("O-O-O"));
    }

    #[test]
    fn castling_with_check_detected() {
        assert!(is_castling("O-O+"));
    }

    #[test]
    fn normal_move_not_castling() {
        assert!(!is_castling("Nf3"));
    }

    #[test]
    fn resolve_kingside_castling_white() {
        let chess_move = NotationMove {
            piece: Piece::King,
            dest: Square { file: 6, rank: 0 },
            threat: Threat::None,
            capture: Capture::None,
            promotion: None,
        };
        let parsed = resolve_castling(&chess_move, Color::White).unwrap();
        assert_eq!(parsed.origin, Square { file: 4, rank: 0 });
        assert_eq!(parsed.dest, Square { file: 6, rank: 0 });
        assert_eq!(
            parsed.castling_rook,
            Some((Square { file: 7, rank: 0 }, Square { file: 5, rank: 0 }))
        );
    }

    #[test]
    fn resolve_queenside_castling_black() {
        let chess_move = NotationMove {
            piece: Piece::King,
            dest: Square { file: 2, rank: 7 },
            threat: Threat::None,
            capture: Capture::None,
            promotion: None,
        };
        let parsed = resolve_castling(&chess_move, Color::Black).unwrap();
        assert_eq!(parsed.origin, Square { file: 4, rank: 7 });
        assert_eq!(
            parsed.castling_rook,
            Some((Square { file: 0, rank: 7 }, Square { file: 3, rank: 7 }))
        );
    }

    #[test]
    fn strip_annotations_removes_symbols() {
        assert_eq!(strip_annotations("Nxf3+"), "Nf3");
        assert_eq!(strip_annotations("Qh5#"), "Qh5");
        assert_eq!(strip_annotations("e4!"), "e4");
    }

    #[test]
    fn strip_annotations_handles_promotion() {
        assert_eq!(strip_annotations("e8=Q"), "e8");
    }

    #[test]
    fn extract_hints_no_disambiguation() {
        assert_eq!(extract_hints("Nf3", Piece::Knight), (None, None));
    }

    #[test]
    fn extract_hints_file_disambiguation() {
        assert_eq!(extract_hints("Rad1", Piece::Rook), (Some(0), None));
    }

    #[test]
    fn extract_hints_rank_disambiguation() {
        assert_eq!(extract_hints("N5f3", Piece::Knight), (None, Some(4)));
    }

    #[test]
    fn extract_hints_pawn_capture() {
        assert_eq!(extract_hints("ed5", Piece::Pawn), (Some(4), None));
    }

    #[test]
    fn extract_hints_pawn_simple_move() {
        assert_eq!(extract_hints("e4", Piece::Pawn), (None, None));
    }
}
