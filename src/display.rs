//! Board display formatting.
//!
//! Provides text rendering of the board state for the REPL and debug output.
//!
//! ## Exported functions
//!
//! - `piece_symbol` — maps a piece and color to its display character (uppercase = white, lowercase = black)
//! - `Board::fmt` — renders the board as an 8x8 grid with rank/file labels

use std::fmt;
use std::io::{self, Write};

use crate::board::{Board, Color};
use crate::chess::Piece;

const RESET: &str = "\x1b[0m";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorMode {
    TrueColor,
    Color256,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SquareShade {
    Light,
    Dark,
}

pub fn color_mode_from_env(colorterm: &str) -> ColorMode {
    match colorterm {
        "truecolor" | "24bit" => ColorMode::TrueColor,
        _ => ColorMode::Color256,
    }
}

fn piece_foreground(color: Color, mode: ColorMode) -> &'static str {
    match (color, mode) {
        (Color::White, ColorMode::TrueColor) => "\x1b[38;2;255;255;255m",
        (Color::Black, ColorMode::TrueColor) => "\x1b[38;2;0;0;0m",
        (Color::White, ColorMode::Color256) => "\x1b[38;5;231m",
        (Color::Black, ColorMode::Color256) => "\x1b[38;5;16m",
    }
}

fn square_background(shade: SquareShade, mode: ColorMode) -> &'static str {
    match (shade, mode) {
        (SquareShade::Light, ColorMode::TrueColor) => "\x1b[48;2;235;236;208m",
        (SquareShade::Dark, ColorMode::TrueColor) => "\x1b[48;2;119;149;86m",
        (SquareShade::Light, ColorMode::Color256) => "\x1b[48;5;187m",
        (SquareShade::Dark, ColorMode::Color256) => "\x1b[48;5;65m",
    }
}

type Sprite = [&'static str; 3];

const SPRITE_HEIGHT: usize = 3;
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

const EMPTY_SQUARE: &str = "       ";

fn render_square_row(
    writer: &mut impl Write,
    square: Option<(Piece, Color)>,
    shade: SquareShade,
    mode: ColorMode,
    row: usize,
) -> io::Result<()> {
    let bg = square_background(shade, mode);
    match square {
        None => write!(writer, "{bg}{EMPTY_SQUARE}{RESET}"),
        Some((piece, color)) => {
            let fg = piece_foreground(color, mode);
            let sprite_row = sprite_for(piece)[row];
            write!(writer, "{bg}{fg}{sprite_row}{RESET}")
        }
    }
}

fn label_foreground(mode: ColorMode) -> &'static str {
    match mode {
        ColorMode::TrueColor => "\x1b[38;2;150;150;150m",
        ColorMode::Color256 => "\x1b[38;5;248m",
    }
}

fn render_rank(
    writer: &mut impl Write,
    board: &Board,
    rank: u8,
    mode: ColorMode,
) -> io::Result<()> {
    let label_fg = label_foreground(mode);
    for sprite_row in 0..SPRITE_HEIGHT {
        if sprite_row == 1 {
            write!(writer, "{label_fg} {} {RESET}", rank + 1)?;
        } else {
            write!(writer, "   ")?;
        }
        for file in 0..BOARD_SIZE {
            let shade = square_shade(file, rank);
            let square = board.get(file, rank);
            render_square_row(writer, square, shade, mode, sprite_row)?;
        }
        writeln!(writer)?;
    }
    Ok(())
}

const FILE_LABELS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

fn render_file_labels(writer: &mut impl Write, mode: ColorMode) -> io::Result<()> {
    let label_fg = label_foreground(mode);
    write!(writer, "   ")?;
    for label in FILE_LABELS {
        write!(writer, "{label_fg}   {label}   {RESET}")?;
    }
    writeln!(writer)
}

pub fn detect_color_mode() -> ColorMode {
    let colorterm = std::env::var("COLORTERM").unwrap_or_default();
    color_mode_from_env(&colorterm)
}

pub fn render(board: &Board, writer: &mut impl Write, mode: ColorMode) -> io::Result<()> {
    for rank in (0..BOARD_SIZE).rev() {
        render_rank(writer, board, rank, mode)?;
    }
    render_file_labels(writer, mode)
}

#[cfg(test)]
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

    const SQUARE_WIDTH: usize = 7;

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
                    cell_count, SQUARE_WIDTH,
                    "sprite for {piece:?} row {row_idx} should have {SQUARE_WIDTH} cells, got {cell_count}"
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
    fn render_square_row_empty() {
        let mut buf = Vec::new();
        render_square_row(&mut buf, None, SquareShade::Light, ColorMode::TrueColor, 0).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(
            output,
            format!("\x1b[48;2;235;236;208m{EMPTY_SQUARE}\x1b[0m")
        );
    }

    #[test]
    fn render_square_row_occupied() {
        let mut buf = Vec::new();
        render_square_row(
            &mut buf,
            Some((Piece::Rook, Color::White)),
            SquareShade::Dark,
            ColorMode::TrueColor,
            1,
        )
        .unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains('█'), "should contain full block");
        assert!(output.ends_with(RESET), "should end with reset");
    }

    #[test]
    fn render_rank_three_rows() {
        let board = Board::new();
        let mut buf = Vec::new();
        render_rank(&mut buf, &board, 0, ColorMode::TrueColor).unwrap();
        let output = String::from_utf8(buf).unwrap();
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 3, "expected 3 lines, got {}", lines.len());
        assert!(
            lines[1].contains(" 1 "),
            "middle row should have rank label"
        );
        assert!(
            output.contains('█'),
            "should contain block chars from sprites"
        );
    }

    #[test]
    fn render_full_board_initial_position() {
        let board = Board::new();
        let mut buf = Vec::new();
        render(&board, &mut buf, ColorMode::TrueColor).unwrap();
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
    fn render_file_labels_contains_all_files() {
        let mut buf = Vec::new();
        render_file_labels(&mut buf, ColorMode::TrueColor).unwrap();
        let output = String::from_utf8(buf).unwrap();
        for file_label in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'] {
            assert!(
                output.contains(file_label),
                "missing file label: {file_label}"
            );
        }
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
}
