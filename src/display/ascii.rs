use std::io::{self, Write};

use crate::board::Color;
use crate::chess::Piece;

use super::{piece_symbol, DisplayStrategy, SquareShade, FILE_LABELS};

/// Plain ASCII display — no colors, no Unicode.
///
/// Renders pieces as uppercase (white) or lowercase (black) letters.
/// Empty squares show as dots. Useful for terminals without color support
/// or for piping output to text files.
pub struct AsciiDisplay;

impl DisplayStrategy for AsciiDisplay {
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
        _shade: SquareShade,
        _row: usize,
    ) -> io::Result<()> {
        match square {
            None => write!(writer, " . "),
            Some((piece, color)) => {
                let symbol = piece_symbol(piece, color);
                write!(writer, " {symbol} ")
            }
        }
    }

    fn render_rank_label(
        &self,
        writer: &mut dyn Write,
        rank: u8,
        _row: usize,
    ) -> io::Result<()> {
        write!(writer, " {} ", rank + 1)
    }

    fn render_file_labels(&self, writer: &mut dyn Write) -> io::Result<()> {
        write!(writer, "   ")?;
        for label in FILE_LABELS {
            write!(writer, " {label} ")?;
        }
        writeln!(writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimensions() {
        let strategy = AsciiDisplay;
        assert_eq!(strategy.square_height(), 1);
        assert_eq!(strategy.square_width(), 3);
    }

    #[test]
    fn renders_empty_square() {
        let strategy = AsciiDisplay;
        let mut buf = Vec::new();
        strategy
            .render_square_row(&mut buf, None, SquareShade::Light, 0)
            .unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(output, " . ");
    }

    #[test]
    fn renders_occupied_square() {
        let strategy = AsciiDisplay;
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
        assert_eq!(output, " K ");
    }
}
