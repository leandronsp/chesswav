use std::io::{self, Write};

use crate::engine::board::Color;
use crate::engine::chess::Piece;

use super::colors::{label_foreground, piece_foreground, square_background, RESET};
use super::{ColorMode, DisplayStrategy, SquareShade, FILE_LABELS};

const UNICODE_EMPTY: &str = "   ";

fn unicode_symbol(piece: Piece, color: Color) -> char {
    match (piece, color) {
        (Piece::King, Color::White) => '♔',
        (Piece::Queen, Color::White) => '♕',
        (Piece::Rook, Color::White) => '♖',
        (Piece::Bishop, Color::White) => '♗',
        (Piece::Knight, Color::White) => '♘',
        (Piece::Pawn, Color::White) => '♙',
        (Piece::King, Color::Black) => '♚',
        (Piece::Queen, Color::Black) => '♛',
        (Piece::Rook, Color::Black) => '♜',
        (Piece::Bishop, Color::Black) => '♝',
        (Piece::Knight, Color::Black) => '♞',
        (Piece::Pawn, Color::Black) => '♟',
    }
}

/// Unicode chess symbol display with ANSI colored backgrounds.
///
/// Each square is 3 characters wide and 1 row tall, using standard
/// Unicode chess symbols (♔♕♖♗♘♙ / ♚♛♜♝♞♟). Squares are shaded
/// with the same ANSI background colors as `SpriteDisplay`, giving
/// a compact colored view.
pub struct UnicodeDisplay {
    color_mode: ColorMode,
}

impl UnicodeDisplay {
    pub fn new(color_mode: ColorMode) -> Self {
        Self { color_mode }
    }
}

impl DisplayStrategy for UnicodeDisplay {
    fn square_height(&self) -> usize {
        1
    }

    fn square_width(&self) -> usize {
        3
    }

    fn render_square_row(
        &self,
        writer: &mut dyn Write,
        square: Option<(Piece, Color)>,
        shade: SquareShade,
        _row: usize,
    ) -> io::Result<()> {
        let bg = square_background(shade, self.color_mode);
        match square {
            None => write!(writer, "{bg}{UNICODE_EMPTY}{RESET}"),
            Some((piece, color)) => {
                let fg = piece_foreground(color, self.color_mode);
                let symbol = unicode_symbol(piece, color);
                write!(writer, "{bg}{fg} {symbol} {RESET}")
            }
        }
    }

    fn render_rank_label(
        &self,
        writer: &mut dyn Write,
        rank: u8,
        _row: usize,
    ) -> io::Result<()> {
        let label_fg = label_foreground(self.color_mode);
        write!(writer, "{label_fg} {} {RESET}", rank + 1)
    }

    fn render_file_labels(&self, writer: &mut dyn Write) -> io::Result<()> {
        let label_fg = label_foreground(self.color_mode);
        write!(writer, "   ")?;
        for label in FILE_LABELS {
            write!(writer, "{label_fg} {label} {RESET}")?;
        }
        writeln!(writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimensions() {
        let strategy = UnicodeDisplay::new(ColorMode::TrueColor);
        assert_eq!(strategy.square_height(), 1);
        assert_eq!(strategy.square_width(), 3);
    }

    #[test]
    fn renders_empty_square() {
        let strategy = UnicodeDisplay::new(ColorMode::TrueColor);
        let mut buf = Vec::new();
        strategy
            .render_square_row(&mut buf, None, SquareShade::Light, 0)
            .unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.starts_with("\x1b[48;2;235;236;208m"));
        assert!(output.ends_with(RESET));
        assert_eq!(output.len(), "\x1b[48;2;235;236;208m   \x1b[0m".len());
    }

    #[test]
    fn renders_white_king() {
        let strategy = UnicodeDisplay::new(ColorMode::TrueColor);
        let mut buf = Vec::new();
        strategy
            .render_square_row(
                &mut buf,
                Some((Piece::King, Color::White)),
                SquareShade::Dark,
                0,
            )
            .unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains('♔'));
    }

    #[test]
    fn renders_black_pawn() {
        let strategy = UnicodeDisplay::new(ColorMode::TrueColor);
        let mut buf = Vec::new();
        strategy
            .render_square_row(
                &mut buf,
                Some((Piece::Pawn, Color::Black)),
                SquareShade::Light,
                0,
            )
            .unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains('♟'));
    }

    #[test]
    fn unicode_symbol_white_pieces() {
        assert_eq!(unicode_symbol(Piece::King, Color::White), '♔');
        assert_eq!(unicode_symbol(Piece::Queen, Color::White), '♕');
        assert_eq!(unicode_symbol(Piece::Rook, Color::White), '♖');
        assert_eq!(unicode_symbol(Piece::Bishop, Color::White), '♗');
        assert_eq!(unicode_symbol(Piece::Knight, Color::White), '♘');
        assert_eq!(unicode_symbol(Piece::Pawn, Color::White), '♙');
    }

    #[test]
    fn unicode_symbol_black_pieces() {
        assert_eq!(unicode_symbol(Piece::King, Color::Black), '♚');
        assert_eq!(unicode_symbol(Piece::Queen, Color::Black), '♛');
        assert_eq!(unicode_symbol(Piece::Rook, Color::Black), '♜');
        assert_eq!(unicode_symbol(Piece::Bishop, Color::Black), '♝');
        assert_eq!(unicode_symbol(Piece::Knight, Color::Black), '♞');
        assert_eq!(unicode_symbol(Piece::Pawn, Color::Black), '♟');
    }
}
