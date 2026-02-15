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

use std::fmt;
use std::io::{self, Write};

use crate::board::{Board, Color};
use crate::chess::Piece;

const RESET: &str = "\x1b[0m";

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

/// Half-block pixel art display with ANSI colored backgrounds.
///
/// Each square is 7 characters wide and 3 rows tall, using Unicode
/// half-block characters (▄ ▀ █) to create 7x6 effective pixel
/// resolution per square. Piece foreground and square background
/// colors are rendered via ANSI escape sequences.
pub struct SpriteDisplay {
    color_mode: ColorMode,
}

impl SpriteDisplay {
    pub fn new(color_mode: ColorMode) -> Self {
        Self { color_mode }
    }
}

impl DisplayStrategy for SpriteDisplay {
    fn square_height(&self) -> usize {
        SPRITE_HEIGHT
    }

    fn square_width(&self) -> usize {
        SPRITE_SQUARE_WIDTH
    }

    fn render_square_row(
        &self,
        writer: &mut dyn Write,
        square: Option<(Piece, Color)>,
        shade: SquareShade,
        row: usize,
    ) -> io::Result<()> {
        let bg = square_background(shade, self.color_mode);
        match square {
            None => write!(writer, "{bg}{SPRITE_EMPTY}{RESET}"),
            Some((piece, color)) => {
                let fg = piece_foreground(color, self.color_mode);
                let sprite_row = sprite_for(piece)[row];
                write!(writer, "{bg}{fg}{sprite_row}{RESET}")
            }
        }
    }

    fn render_rank_label(
        &self,
        writer: &mut dyn Write,
        rank: u8,
        row: usize,
    ) -> io::Result<()> {
        let label_fg = label_foreground(self.color_mode);
        if row == 1 {
            write!(writer, "{label_fg} {} {RESET}", rank + 1)
        } else {
            write!(writer, "   ")
        }
    }

    fn render_file_labels(&self, writer: &mut dyn Write) -> io::Result<()> {
        let label_fg = label_foreground(self.color_mode);
        write!(writer, "   ")?;
        for label in FILE_LABELS {
            write!(writer, "{label_fg}   {label}   {RESET}")?;
        }
        writeln!(writer)
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

const UNICODE_EMPTY: &str = "   ";

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

pub fn color_mode_from_env(colorterm: &str) -> ColorMode {
    match colorterm {
        "truecolor" | "24bit" => ColorMode::TrueColor,
        _ => ColorMode::Color256,
    }
}

/// ANSI foreground escape for piece color (white=#FFF, black=#000).
fn piece_foreground(color: Color, mode: ColorMode) -> &'static str {
    match (color, mode) {
        (Color::White, ColorMode::TrueColor) => "\x1b[38;2;255;255;255m",
        (Color::Black, ColorMode::TrueColor) => "\x1b[38;2;0;0;0m",
        (Color::White, ColorMode::Color256) => "\x1b[38;5;231m",
        (Color::Black, ColorMode::Color256) => "\x1b[38;5;16m",
    }
}

/// ANSI background escape for square shade (light=#EBECD0, dark=#779556).
fn square_background(shade: SquareShade, mode: ColorMode) -> &'static str {
    match (shade, mode) {
        (SquareShade::Light, ColorMode::TrueColor) => "\x1b[48;2;235;236;208m",
        (SquareShade::Dark, ColorMode::TrueColor) => "\x1b[48;2;119;149;86m",
        (SquareShade::Light, ColorMode::Color256) => "\x1b[48;5;187m",
        (SquareShade::Dark, ColorMode::Color256) => "\x1b[48;5;65m",
    }
}

/// A sprite is 3 rows of 7-character strings using half-block characters
/// (▄ ▀ █). Each character cell is 1 wide × 2 tall in the terminal, so
/// 7 columns × 3 rows = 7×6 effective pixel resolution per square.
type Sprite = [&'static str; 3];

const SPRITE_HEIGHT: usize = 3;
const SPRITE_SQUARE_WIDTH: usize = 7;
const BOARD_SIZE: u8 = 8;

const KING_SPRITE: Sprite = ["   █   ", "  ▀█▀  ", "  ▀▀▀  "];
const QUEEN_SPRITE: Sprite = ["  ▄ ▄  ", "  ▀█▀  ", "  ▀▀▀  "];
const ROOK_SPRITE: Sprite = [" ▄ ▄ ▄ ", "  ███  ", "  ▀▀▀  "];
const BISHOP_SPRITE: Sprite = ["   ▄   ", "  ▄█▄  ", "  ▀▀▀  "];
const KNIGHT_SPRITE: Sprite = ["  ▄▄▄  ", "  ██   ", "  ▀    "];
const PAWN_SPRITE: Sprite = ["       ", "  ▄█▄  ", "  ▀▀▀  "];

fn sprite_for(piece: Piece) -> Sprite {
    match piece {
        Piece::King => KING_SPRITE,
        Piece::Queen => QUEEN_SPRITE,
        Piece::Rook => ROOK_SPRITE,
        Piece::Bishop => BISHOP_SPRITE,
        Piece::Knight => KNIGHT_SPRITE,
        Piece::Pawn => PAWN_SPRITE,
    }
}

fn square_shade(file: u8, rank: u8) -> SquareShade {
    if (file + rank) % 2 != 0 {
        SquareShade::Light
    } else {
        SquareShade::Dark
    }
}

const SPRITE_EMPTY: &str = "       ";

/// ANSI foreground escape for rank/file labels (muted gray).
fn label_foreground(mode: ColorMode) -> &'static str {
    match mode {
        ColorMode::TrueColor => "\x1b[38;2;150;150;150m",
        ColorMode::Color256 => "\x1b[38;5;248m",
    }
}

const FILE_LABELS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

pub fn detect_color_mode() -> ColorMode {
    let colorterm = std::env::var("COLORTERM").unwrap_or_default();
    color_mode_from_env(&colorterm)
}

pub fn render(
    board: &Board,
    writer: &mut impl Write,
    strategy: &impl DisplayStrategy,
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

    #[test]
    fn sprite_for_returns_three_rows_of_seven_cells() {
        for piece in [
            Piece::King,
            Piece::Queen,
            Piece::Rook,
            Piece::Bishop,
            Piece::Knight,
            Piece::Pawn,
        ] {
            let sprite = sprite_for(piece);
            assert_eq!(sprite.len(), 3, "sprite for {piece:?} should have 3 rows");
            for (row_idx, row) in sprite.iter().enumerate() {
                let cell_count = row.chars().count();
                assert_eq!(
                    cell_count, SPRITE_SQUARE_WIDTH,
                    "sprite for {piece:?} row {row_idx} should have {SPRITE_SQUARE_WIDTH} cells, got {cell_count}"
                );
            }
        }
    }

    #[test]
    fn sprites_are_distinct() {
        let all_sprites = [
            sprite_for(Piece::King),
            sprite_for(Piece::Queen),
            sprite_for(Piece::Rook),
            sprite_for(Piece::Bishop),
            sprite_for(Piece::Knight),
            sprite_for(Piece::Pawn),
        ];
        for i in 0..all_sprites.len() {
            for j in (i + 1)..all_sprites.len() {
                assert_ne!(
                    all_sprites[i], all_sprites[j],
                    "sprites {i} and {j} should differ"
                );
            }
        }
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
    fn piece_foreground_truecolor() {
        assert_eq!(
            piece_foreground(Color::White, ColorMode::TrueColor),
            "\x1b[38;2;255;255;255m"
        );
        assert_eq!(
            piece_foreground(Color::Black, ColorMode::TrueColor),
            "\x1b[38;2;0;0;0m"
        );
    }

    #[test]
    fn piece_foreground_256() {
        assert_eq!(
            piece_foreground(Color::White, ColorMode::Color256),
            "\x1b[38;5;231m"
        );
        assert_eq!(
            piece_foreground(Color::Black, ColorMode::Color256),
            "\x1b[38;5;16m"
        );
    }

    #[test]
    fn square_background_truecolor() {
        let light = square_background(SquareShade::Light, ColorMode::TrueColor);
        assert_eq!(light, "\x1b[48;2;235;236;208m");
        let dark = square_background(SquareShade::Dark, ColorMode::TrueColor);
        assert_eq!(dark, "\x1b[48;2;119;149;86m");
    }

    #[test]
    fn square_background_256() {
        let light = square_background(SquareShade::Light, ColorMode::Color256);
        assert_eq!(light, "\x1b[48;5;187m");
        let dark = square_background(SquareShade::Dark, ColorMode::Color256);
        assert_eq!(dark, "\x1b[48;5;65m");
    }

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
    fn display_initial_position() {
        let board = Board::new();
        let display = format!("{board}");
        assert!(display.contains("r n b q k b n r"));
        assert!(display.contains("P P P P P P P P"));
        assert!(display.contains("a b c d e f g h"));
    }

    #[test]
    fn ascii_display_renders_empty_square() {
        let strategy = AsciiDisplay;
        let mut buf = Vec::new();
        strategy
            .render_square_row(&mut buf, None, SquareShade::Light, 0)
            .unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(output, " . ");
    }

    #[test]
    fn ascii_display_renders_occupied_square() {
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

    #[test]
    fn ascii_display_dimensions() {
        let strategy = AsciiDisplay;
        assert_eq!(strategy.square_height(), 1);
        assert_eq!(strategy.square_width(), 3);
    }

    #[test]
    fn sprite_display_dimensions() {
        let strategy = SpriteDisplay::new(ColorMode::TrueColor);
        assert_eq!(strategy.square_height(), 3);
        assert_eq!(strategy.square_width(), 7);
    }

    #[test]
    fn sprite_display_renders_empty_square() {
        let strategy = SpriteDisplay::new(ColorMode::TrueColor);
        let mut buf = Vec::new();
        strategy
            .render_square_row(&mut buf, None, SquareShade::Light, 0)
            .unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(
            output,
            format!("\x1b[48;2;235;236;208m       \x1b[0m")
        );
    }

    #[test]
    fn sprite_display_renders_occupied_square() {
        let strategy = SpriteDisplay::new(ColorMode::TrueColor);
        let mut buf = Vec::new();
        strategy
            .render_square_row(
                &mut buf,
                Some((Piece::Rook, Color::White)),
                SquareShade::Dark,
                1,
            )
            .unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains('█'), "should contain full block");
        assert!(output.ends_with(RESET), "should end with reset");
    }

    #[test]
    fn unicode_display_dimensions() {
        let strategy = UnicodeDisplay::new(ColorMode::TrueColor);
        assert_eq!(strategy.square_height(), 1);
        assert_eq!(strategy.square_width(), 3);
    }

    #[test]
    fn unicode_display_renders_empty_square() {
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
    fn unicode_display_renders_white_king() {
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
    fn unicode_display_renders_black_pawn() {
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
    fn render_with_sprite_strategy_matches_old_output() {
        let board = Board::new();
        let strategy = SpriteDisplay::new(ColorMode::TrueColor);

        let mut new_buf = Vec::new();
        render(&board, &mut new_buf, &strategy).unwrap();
        let new_output = String::from_utf8(new_buf).unwrap();

        for rank in 1..=8 {
            assert!(
                new_output.contains(&format!(" {rank} ")),
                "missing rank {rank}"
            );
        }
        assert!(new_output.contains('█'), "should contain full blocks");
        assert!(new_output.contains('▄'), "should contain lower half blocks");
        assert!(new_output.contains('▀'), "should contain upper half blocks");
        let line_count = new_output.lines().count();
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
