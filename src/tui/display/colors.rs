use crate::engine::board::Color;

use super::{ColorMode, SquareShade};

pub const RESET: &str = "\x1b[0m";

/// ANSI foreground escape for piece color (white=#FFF, black=#000).
pub fn piece_foreground(color: Color, mode: ColorMode) -> &'static str {
    match (color, mode) {
        (Color::White, ColorMode::TrueColor) => "\x1b[38;2;255;255;255m",
        (Color::Black, ColorMode::TrueColor) => "\x1b[38;2;0;0;0m",
        (Color::White, ColorMode::Color256) => "\x1b[38;5;231m",
        (Color::Black, ColorMode::Color256) => "\x1b[38;5;16m",
    }
}

/// ANSI background escape for square shade (light=#EBECD0, dark=#779556).
pub fn square_background(shade: SquareShade, mode: ColorMode) -> &'static str {
    match (shade, mode) {
        (SquareShade::Light, ColorMode::TrueColor) => "\x1b[48;2;235;236;208m",
        (SquareShade::Dark, ColorMode::TrueColor) => "\x1b[48;2;119;149;86m",
        (SquareShade::Light, ColorMode::Color256) => "\x1b[48;5;187m",
        (SquareShade::Dark, ColorMode::Color256) => "\x1b[48;5;65m",
    }
}

/// ANSI foreground escape for rank/file labels (muted gray).
pub fn label_foreground(mode: ColorMode) -> &'static str {
    match mode {
        ColorMode::TrueColor => "\x1b[38;2;150;150;150m",
        ColorMode::Color256 => "\x1b[38;5;248m",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
