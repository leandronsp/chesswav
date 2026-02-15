use std::io::{self, Write};

use crate::engine::board::Color;
use crate::engine::chess::Piece;

use super::colors::{label_foreground, piece_foreground, square_background, RESET};
use super::{ColorMode, DisplayStrategy, SquareShade, FILE_LABELS};

/// A sprite is 3 rows of 7-character strings using half-block characters
/// (▄ ▀ █). Each character cell is 1 wide × 2 tall in the terminal, so
/// 7 columns × 3 rows = 7×6 effective pixel resolution per square.
type Sprite = [&'static str; 3];

const SPRITE_HEIGHT: usize = 3;
pub(super) const SPRITE_SQUARE_WIDTH: usize = 7;

const KING_SPRITE: Sprite = ["   █   ", "  ▀█▀  ", "  ▀▀▀  "];
const QUEEN_SPRITE: Sprite = ["  ▄ ▄  ", "  ▀█▀  ", "  ▀▀▀  "];
const ROOK_SPRITE: Sprite = [" ▄ ▄ ▄ ", "  ███  ", "  ▀▀▀  "];
const BISHOP_SPRITE: Sprite = ["   ▄   ", "  ▄█▄  ", "  ▀▀▀  "];
const KNIGHT_SPRITE: Sprite = ["  ▄▄▄  ", "  ██   ", "  ▀    "];
const PAWN_SPRITE: Sprite = ["       ", "  ▄█▄  ", "  ▀▀▀  "];

const SPRITE_EMPTY: &str = "       ";

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimensions() {
        let strategy = SpriteDisplay::new(ColorMode::TrueColor);
        assert_eq!(strategy.square_height(), 3);
        assert_eq!(strategy.square_width(), 7);
    }

    #[test]
    fn renders_empty_square() {
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
    fn renders_occupied_square() {
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
}
