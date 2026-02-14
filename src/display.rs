//! Board display formatting.
//!
//! Provides text rendering of the board state for the REPL and debug output.
//!
//! ## Exported functions
//!
//! - `piece_symbol` — maps a piece and color to its display character (uppercase = white, lowercase = black)
//! - `Board::fmt` — renders the board as an 8x8 grid with rank/file labels

use std::fmt;

use crate::board::{Board, Color};
use crate::chess::Piece;

pub fn piece_symbol(piece: Piece, color: Color) -> char {
    let symbol = match piece {
        Piece::Pawn => 'P',
        Piece::Knight => 'N',
        Piece::Bishop => 'B',
        Piece::Rook => 'R',
        Piece::Queen => 'Q',
        Piece::King => 'K',
    };
    match color {
        Color::White => symbol,
        Color::Black => symbol.to_ascii_lowercase(),
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            write!(f, "  {} |", rank + 1)?;
            for file in 0..8u8 {
                let symbol = match self.get(file, rank as u8) {
                    Some((piece, color)) => piece_symbol(piece, color),
                    None => '.',
                };
                write!(f, " {symbol}")?;
            }
            writeln!(f)?;
        }
        writeln!(f, "    +----------------")?;
        writeln!(f, "      a b c d e f g h")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_pawn_symbol() {
        assert_eq!(piece_symbol(Piece::Pawn, Color::White), 'P');
    }

    #[test]
    fn black_pawn_symbol() {
        assert_eq!(piece_symbol(Piece::Pawn, Color::Black), 'p');
    }

    #[test]
    fn white_knight_symbol() {
        assert_eq!(piece_symbol(Piece::Knight, Color::White), 'N');
    }

    #[test]
    fn black_queen_symbol() {
        assert_eq!(piece_symbol(Piece::Queen, Color::Black), 'q');
    }

    #[test]
    fn display_initial_position() {
        let board = Board::new();
        let display = format!("{board}");
        assert!(display.contains("r n b q k b n r"));
        assert!(display.contains("P P P P P P P P"));
        assert!(display.contains("a b c d e f g h"));
    }
}
