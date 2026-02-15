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

use crate::engine::board::{Board, Color};
use crate::engine::chess::Piece;

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

/// Returns a heap-allocated strategy chosen at runtime.
/// `dyn DisplayStrategy` enables dynamic dispatch — the concrete type
/// (Sprite, Unicode, or Ascii) is resolved through a vtable at runtime,
/// which lets the REPL swap strategies via the `display` command.
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

const SIDEBAR_HEADER: &str = "Moves";
const SIDEBAR_DIVIDER: &str = "─────────────";

pub fn format_move_list<S: AsRef<str>>(half_moves: &[S]) -> Vec<String> {
    half_moves
        .chunks(2)
        .enumerate()
        .map(|(index, pair)| {
            let move_number = index + 1;
            let white_move = pair[0].as_ref();
            match pair.get(1) {
                Some(black_move) => {
                    format!("{move_number}. {white_move:<6}{}", black_move.as_ref())
                }
                None => format!("{move_number}. {white_move}"),
            }
        })
        .collect()
}

pub fn cursor_up_and_clear(writer: &mut impl Write, line_count: usize) -> io::Result<()> {
    write!(writer, "\x1b[{line_count}A\x1b[J")
}

pub fn layout_height(strategy: &dyn DisplayStrategy) -> usize {
    1 + BOARD_SIZE as usize * strategy.square_height() + 1
}

pub fn sidebar_lines<S: AsRef<str>>(half_moves: &[S], available_height: usize) -> Vec<String> {
    let mut lines = vec![SIDEBAR_HEADER.to_string(), SIDEBAR_DIVIDER.to_string()];
    let move_lines = format_move_list(half_moves);
    let max_move_lines = available_height.saturating_sub(2);
    let skip_count = move_lines.len().saturating_sub(max_move_lines);
    lines.extend(move_lines.into_iter().skip(skip_count));
    lines
}

fn square_shade(file: u8, rank: u8) -> SquareShade {
    if (file + rank) % 2 != 0 {
        SquareShade::Light
    } else {
        SquareShade::Dark
    }
}

/// `&dyn DisplayStrategy` accepts any strategy behind a trait object,
/// matching the `Box<dyn DisplayStrategy>` the REPL holds.
pub fn render<S: AsRef<str>>(
    board: &Board,
    writer: &mut impl Write,
    strategy: &dyn DisplayStrategy,
    moves: &[S],
) -> io::Result<()> {
    strategy.render_file_labels(writer)?;
    let board_height = BOARD_SIZE as usize * strategy.square_height();
    let sidebar = if moves.is_empty() {
        vec![]
    } else {
        sidebar_lines(moves, board_height)
    };
    let mut board_line_index = 0;
    for rank in (0..BOARD_SIZE).rev() {
        for row in 0..strategy.square_height() {
            strategy.render_rank_label(writer, rank, row)?;
            for file in 0..BOARD_SIZE {
                let shade = square_shade(file, rank);
                let square = board.get(file, rank);
                strategy.render_square_row(writer, square, shade, row)?;
            }
            if let Some(sidebar_text) = sidebar.get(board_line_index) {
                write!(writer, "   {sidebar_text}")?;
            }
            board_line_index += 1;
            writeln!(writer)?;
        }
    }
    strategy.render_file_labels(writer)
}

#[cfg(test)]
mod tests {
    use super::*;

    const NO_MOVES: &[&str] = &[];

    #[test]
    fn format_move_list_empty_input() {
        let result = format_move_list(NO_MOVES);
        assert!(result.is_empty());
    }

    #[test]
    fn format_move_list_single_move() {
        let moves = vec!["e4".to_string()];
        let result = format_move_list(&moves);
        assert_eq!(result, vec!["1. e4"]);
    }

    #[test]
    fn format_move_list_complete_pair() {
        let moves = vec!["e4".to_string(), "e5".to_string()];
        let result = format_move_list(&moves);
        assert_eq!(result, vec!["1. e4    e5"]);
    }

    #[test]
    fn format_move_list_multiple_pairs() {
        let moves = vec![
            "e4".to_string(),
            "e5".to_string(),
            "Nf3".to_string(),
            "Nc6".to_string(),
        ];
        let result = format_move_list(&moves);
        assert_eq!(result, vec!["1. e4    e5", "2. Nf3   Nc6"]);
    }

    #[test]
    fn format_move_list_odd_count() {
        let moves = vec![
            "e4".to_string(),
            "e5".to_string(),
            "Nf3".to_string(),
        ];
        let result = format_move_list(&moves);
        assert_eq!(result, vec!["1. e4    e5", "2. Nf3"]);
    }

    #[test]
    fn sidebar_lines_empty_moves() {
        let result = sidebar_lines(NO_MOVES, 8);
        assert_eq!(result, vec!["Moves", "─────────────"]);
    }

    #[test]
    fn sidebar_lines_with_moves() {
        let moves = vec!["e4".to_string(), "e5".to_string()];
        let result = sidebar_lines(&moves, 8);
        assert_eq!(result, vec!["Moves", "─────────────", "1. e4    e5"]);
    }

    #[test]
    fn sidebar_lines_scrolling() {
        let moves: Vec<String> = (0..20)
            .map(|i| format!("m{i}"))
            .collect();
        let result = sidebar_lines(&moves, 8);
        assert_eq!(result.len(), 8);
        assert_eq!(result[0], "Moves");
        assert_eq!(result[1], "─────────────");
        assert_eq!(result.last().unwrap(), "10. m18   m19");
    }

    #[test]
    fn sidebar_lines_exact_fit() {
        let moves = vec![
            "e4".to_string(), "e5".to_string(),
            "Nf3".to_string(), "Nc6".to_string(),
            "Bb5".to_string(), "a6".to_string(),
        ];
        let result = sidebar_lines(&moves, 5);
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], "Moves");
        assert_eq!(result[1], "─────────────");
        assert_eq!(result[2], "1. e4    e5");
        assert_eq!(result[3], "2. Nf3   Nc6");
        assert_eq!(result[4], "3. Bb5   a6");
    }

    #[test]
    fn render_with_empty_moves_has_no_sidebar() {
        let board = Board::new();
        let mut buf = Vec::new();
        render(&board, &mut buf, &AsciiDisplay, NO_MOVES).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(!output.contains("Moves"));
    }

    #[test]
    fn render_with_moves_shows_sidebar() {
        let board = Board::new();
        let moves = vec!["e4".to_string(), "e5".to_string()];
        let mut buf = Vec::new();
        render(&board, &mut buf, &AsciiDisplay, &moves).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Moves"), "should contain sidebar header");
        assert!(output.contains("─────────────"), "should contain sidebar divider");
        assert!(output.contains("1. e4    e5"), "should contain move line");
    }

    #[test]
    fn render_sidebar_not_on_file_label_lines() {
        let board = Board::new();
        let moves = vec!["e4".to_string(), "e5".to_string()];
        let mut buf = Vec::new();
        render(&board, &mut buf, &AsciiDisplay, &moves).unwrap();
        let output = String::from_utf8(buf).unwrap();
        let lines: Vec<&str> = output.lines().collect();
        let first_line = lines[0];
        let last_line = lines.last().unwrap();
        assert!(!first_line.contains("Moves"), "top file labels should have no sidebar");
        assert!(!last_line.contains("Moves"), "bottom file labels should have no sidebar");
    }

    #[test]
    fn render_with_moves_same_line_count() {
        let board = Board::new();
        let moves = vec!["e4".to_string(), "e5".to_string()];
        let mut buf_no_moves = Vec::new();
        let mut buf_with_moves = Vec::new();
        render(&board, &mut buf_no_moves, &AsciiDisplay, NO_MOVES).unwrap();
        render(&board, &mut buf_with_moves, &AsciiDisplay, &moves).unwrap();
        let lines_no_moves = String::from_utf8(buf_no_moves).unwrap().lines().count();
        let lines_with_moves = String::from_utf8(buf_with_moves).unwrap().lines().count();
        assert_eq!(lines_no_moves, lines_with_moves, "sidebar should not add extra lines");
    }

    #[test]
    fn cursor_up_and_clear_ten_lines() {
        let mut buf = Vec::new();
        cursor_up_and_clear(&mut buf, 10).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(output, "\x1b[10A\x1b[J");
    }

    #[test]
    fn cursor_up_and_clear_one_line() {
        let mut buf = Vec::new();
        cursor_up_and_clear(&mut buf, 1).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(output, "\x1b[1A\x1b[J");
    }

    #[test]
    fn layout_height_ascii() {
        let strategy = AsciiDisplay;
        assert_eq!(layout_height(&strategy), 10);
    }

    #[test]
    fn layout_height_sprite() {
        let strategy = SpriteDisplay::new(ColorMode::TrueColor);
        assert_eq!(layout_height(&strategy), 26);
    }

    #[test]
    fn layout_height_unicode() {
        let strategy = UnicodeDisplay::new(ColorMode::TrueColor);
        assert_eq!(layout_height(&strategy), 10);
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
        render(&board, &mut buf, &AsciiDisplay, NO_MOVES).unwrap();
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
        render(&board, &mut buf, &strategy, NO_MOVES).unwrap();
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
        assert_eq!(line_count, 26, "expected 26 lines, got {line_count}");
    }

    #[test]
    fn render_with_ascii_strategy() {
        let board = Board::new();
        let strategy = AsciiDisplay;
        let mut buf = Vec::new();
        render(&board, &mut buf, &strategy, NO_MOVES).unwrap();
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
        assert_eq!(line_count, 10, "top labels + 8 ranks + bottom labels = 10 lines");
    }

    #[test]
    fn render_with_sprite_strategy() {
        let board = Board::new();
        let strategy = SpriteDisplay::new(ColorMode::TrueColor);
        let mut buf = Vec::new();
        render(&board, &mut buf, &strategy, NO_MOVES).unwrap();
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
        assert_eq!(line_count, 26, "expected 26 lines, got {line_count}");
    }

    #[test]
    fn render_with_unicode_strategy() {
        let board = Board::new();
        let strategy = UnicodeDisplay::new(ColorMode::TrueColor);
        let mut buf = Vec::new();
        render(&board, &mut buf, &strategy, NO_MOVES).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains('♔'), "should contain white king");
        assert!(output.contains('♟'), "should contain black pawn");
        let line_count = output.lines().count();
        assert_eq!(line_count, 10, "top labels + 8 ranks + bottom labels = 10 lines");
    }
}
