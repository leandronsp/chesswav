//! Board display with pluggable rendering strategies.
//!
//! The [`DisplayStrategy`] trait defines how individual squares, rank labels,
//! and file labels are drawn. The [`render`] function iterates the board and
//! delegates all output to the chosen strategy.
//!
//! ## Strategies
//!
//! | Strategy | Rendering | Colors |
//! |----------|-----------|--------|
//! | [`SpriteDisplay`] | Half-block pixel art (7×3 per square) | ANSI |
//! | [`UnicodeDisplay`] | Chess symbols ♔♕♖♗♘♙ (3×1 per square) | ANSI |
//! | [`AsciiDisplay`] | Letters K Q R B N P (3×1 per square) | None |
//!
//! ## Color mode
//!
//! [`ColorMode`] selects between truecolor (24-bit) and 256-color ANSI
//! output. It is detected from the `COLORTERM` environment variable via
//! [`detect_color_mode`]. Both [`SpriteDisplay`] and [`UnicodeDisplay`]
//! accept a `ColorMode`; [`AsciiDisplay`] ignores colors entirely.

mod ascii;
mod colors;
mod sprite;
mod unicode;

pub use ascii::AsciiDisplay;
pub use sprite::SpriteDisplay;
pub use unicode::UnicodeDisplay;

use std::io::{self, Write};

use crate::board::{Board, Color};
use crate::chess::Piece;

const BOARD_SIZE: u8 = 8;
const FILE_LABELS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

/// ANSI color depth for terminal output.
///
/// Detected from the `COLORTERM` environment variable:
/// - `"truecolor"` or `"24bit"` → [`TrueColor`](ColorMode::TrueColor) (RGB)
/// - anything else → [`Color256`](ColorMode::Color256) (xterm palette)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorMode {
    TrueColor,
    Color256,
}

/// Checkerboard square parity — determines the background shade.
///
/// On a standard board, a1 (file=0, rank=0) is dark. Adjacent squares
/// alternate: `(file + rank) % 2 != 0` → Light, otherwise Dark.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SquareShade {
    Light,
    Dark,
}

/// Rendering strategy for board display.
///
/// Each strategy controls how individual squares, rank labels, and file
/// labels are drawn. The `render` function iterates the board and delegates
/// all output to the strategy, enabling different visual representations
/// (sprite pixel art, Unicode symbols, plain ASCII) through the same loop.
pub trait DisplayStrategy {
    fn square_height(&self) -> usize;
    fn square_width(&self) -> usize;
    fn render_square_row(
        &self,
        writer: &mut dyn Write,
        square: Option<(Piece, Color)>,
        shade: SquareShade,
        row: usize,
    ) -> io::Result<()>;
    fn render_rank_label(
        &self,
        writer: &mut dyn Write,
        rank: u8,
        row: usize,
    ) -> io::Result<()>;
    fn render_file_labels(&self, writer: &mut dyn Write) -> io::Result<()>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayMode {
    Sprite,
    Unicode,
    Ascii,
}

pub fn parse_display_mode(value: &str) -> Option<DisplayMode> {
    match value {
        "sprite" => Some(DisplayMode::Sprite),
        "unicode" => Some(DisplayMode::Unicode),
        "ascii" => Some(DisplayMode::Ascii),
        _ => None,
    }
}

pub fn create_strategy(mode: DisplayMode, color_mode: ColorMode) -> Box<dyn DisplayStrategy> {
    match mode {
        DisplayMode::Sprite => Box::new(SpriteDisplay::new(color_mode)),
        DisplayMode::Unicode => Box::new(UnicodeDisplay::new(color_mode)),
        DisplayMode::Ascii => Box::new(AsciiDisplay),
    }
}

pub fn color_mode_from_env(colorterm: &str) -> ColorMode {
    match colorterm {
        "truecolor" | "24bit" => ColorMode::TrueColor,
        _ => ColorMode::Color256,
    }
}

pub fn detect_color_mode() -> ColorMode {
    let colorterm = std::env::var("COLORTERM").unwrap_or_default();
    color_mode_from_env(&colorterm)
}

fn square_shade(file: u8, rank: u8) -> SquareShade {
    if (file + rank) % 2 != 0 {
        SquareShade::Light
    } else {
        SquareShade::Dark
    }
}

pub fn render(
    board: &Board,
    writer: &mut impl Write,
    strategy: &dyn DisplayStrategy,
) -> io::Result<()> {
    for rank in (0..BOARD_SIZE).rev() {
        for row in 0..strategy.square_height() {
            strategy.render_rank_label(writer, rank, row)?;
            for file in 0..BOARD_SIZE {
                let shade = square_shade(file, rank);
                let square = board.get(file, rank);
                strategy.render_square_row(writer, square, shade, row)?;
            }
            writeln!(writer)?;
        }
    }
    strategy.render_file_labels(writer)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_shade_corners() {
        assert_eq!(square_shade(0, 0), SquareShade::Dark); // a1
        assert_eq!(square_shade(1, 0), SquareShade::Light); // b1
        assert_eq!(square_shade(7, 7), SquareShade::Dark); // h8
        assert_eq!(square_shade(0, 1), SquareShade::Light); // a2
    }

    #[test]
    fn color_mode_truecolor_from_env() {
        assert_eq!(color_mode_from_env("truecolor"), ColorMode::TrueColor);
        assert_eq!(color_mode_from_env("24bit"), ColorMode::TrueColor);
    }

    #[test]
    fn color_mode_fallback_to_256() {
        assert_eq!(color_mode_from_env("256color"), ColorMode::Color256);
        assert_eq!(color_mode_from_env(""), ColorMode::Color256);
    }

    #[test]
    fn parse_display_mode_valid_values() {
        assert_eq!(parse_display_mode("sprite"), Some(DisplayMode::Sprite));
        assert_eq!(parse_display_mode("unicode"), Some(DisplayMode::Unicode));
        assert_eq!(parse_display_mode("ascii"), Some(DisplayMode::Ascii));
    }

    #[test]
    fn create_strategy_sprite_dimensions() {
        let strategy = create_strategy(DisplayMode::Sprite, ColorMode::TrueColor);
        assert_eq!(strategy.square_height(), 3);
        assert_eq!(strategy.square_width(), 7);
    }

    #[test]
    fn create_strategy_unicode_dimensions() {
        let strategy = create_strategy(DisplayMode::Unicode, ColorMode::TrueColor);
        assert_eq!(strategy.square_height(), 1);
        assert_eq!(strategy.square_width(), 3);
    }

    #[test]
    fn create_strategy_ascii_dimensions() {
        let strategy = create_strategy(DisplayMode::Ascii, ColorMode::TrueColor);
        assert_eq!(strategy.square_height(), 1);
        assert_eq!(strategy.square_width(), 3);
    }

    #[test]
    fn parse_display_mode_invalid_values() {
        assert_eq!(parse_display_mode("foo"), None);
        assert_eq!(parse_display_mode(""), None);
        assert_eq!(parse_display_mode("SPRITE"), None);
    }

    #[test]
    fn display_initial_position() {
        let board = Board::new();
        let mut buf = Vec::new();
        render(&board, &mut buf, &AsciiDisplay).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains(" r "), "should contain black rook");
        assert!(output.contains(" P "), "should contain white pawn");
        assert!(output.contains('a'), "should contain file labels");
    }

    #[test]
    fn render_full_board_initial_position() {
        let board = Board::new();
        let strategy = SpriteDisplay::new(ColorMode::TrueColor);
        let mut buf = Vec::new();
        render(&board, &mut buf, &strategy).unwrap();
        let output = String::from_utf8(buf).unwrap();
        for rank in 1..=8 {
            assert!(output.contains(&format!(" {rank} ")), "missing rank {rank}");
        }
        for file_label in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'] {
            assert!(output.contains(file_label), "missing file {file_label}");
        }
        assert!(output.contains('█'), "should contain full blocks");
        assert!(output.contains('▄'), "should contain lower half blocks");
        assert!(output.contains('▀'), "should contain upper half blocks");
        let line_count = output.lines().count();
        assert_eq!(line_count, 25, "expected 25 lines, got {line_count}");
    }

    #[test]
    fn render_with_ascii_strategy() {
        let board = Board::new();
        let strategy = AsciiDisplay;
        let mut buf = Vec::new();
        render(&board, &mut buf, &strategy).unwrap();
        let output = String::from_utf8(buf).unwrap();
        for rank in 1..=8 {
            assert!(output.contains(&format!(" {rank} ")), "missing rank {rank}");
        }
        for file_label in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'] {
            assert!(output.contains(file_label), "missing file {file_label}");
        }
        assert!(output.contains(" R "), "should contain rook");
        assert!(output.contains(" P "), "should contain pawn");
        assert!(output.contains(" . "), "should contain empty square");
        let line_count = output.lines().count();
        assert_eq!(line_count, 9, "8 ranks + 1 file label row = 9 lines");
    }

    #[test]
    fn render_with_sprite_strategy() {
        let board = Board::new();
        let strategy = SpriteDisplay::new(ColorMode::TrueColor);
        let mut buf = Vec::new();
        render(&board, &mut buf, &strategy).unwrap();
        let output = String::from_utf8(buf).unwrap();
        for rank in 1..=8 {
            assert!(
                output.contains(&format!(" {rank} ")),
                "missing rank {rank}"
            );
        }
        assert!(output.contains('█'), "should contain full blocks");
        assert!(output.contains('▄'), "should contain lower half blocks");
        assert!(output.contains('▀'), "should contain upper half blocks");
        let line_count = output.lines().count();
        assert_eq!(line_count, 25, "expected 25 lines, got {line_count}");
    }

    #[test]
    fn render_with_unicode_strategy() {
        let board = Board::new();
        let strategy = UnicodeDisplay::new(ColorMode::TrueColor);
        let mut buf = Vec::new();
        render(&board, &mut buf, &strategy).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains('♔'), "should contain white king");
        assert!(output.contains('♟'), "should contain black pawn");
        let line_count = output.lines().count();
        assert_eq!(line_count, 9, "8 ranks + 1 file label row = 9 lines");
    }
}
