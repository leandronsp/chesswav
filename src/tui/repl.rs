use std::io::{self, BufRead, BufWriter, Write};

use crate::audio;
use crate::engine::board::{Board, Color};
use crate::engine::chess::NotationMove;
use super::display;

fn is_white_turn(move_index: usize) -> bool {
    move_index % 2 == 0
}

fn full_move_number(move_index: usize) -> usize {
    move_index / 2 + 1
}

enum RenderMode {
    Initial,
    Redraw(usize),
}

fn render_board<S: AsRef<str>>(
    board: &Board,
    writer: &mut impl Write,
    strategy: &dyn display::DisplayStrategy,
    moves: &[S],
    mode: RenderMode,
) -> io::Result<()> {
    if let RenderMode::Redraw(clear_height) = mode {
        display::cursor_up_and_clear(writer, clear_height)?;
    }
    display::render(board, writer, strategy, moves)?;
    writer.flush()
}

pub fn run(initial_mode: display::DisplayMode) {
    let mut board = Board::new();
    let mut move_index: usize = 0;
    let mut move_history: Vec<String> = Vec::new();

    println!();
    println!("  ChessWAV Interactive Mode");
    println!("  Type moves in algebraic notation. Commands: display, reset, quit");
    println!();

    let color_mode = display::detect_color_mode();
    let mut strategy: Box<dyn display::DisplayStrategy> =
        display::create_strategy(initial_mode, color_mode);
    let stdin = io::stdin();
    let mut stdout = BufWriter::new(io::stdout());

    if let Err(err) = render_board(&board, &mut stdout, &*strategy, &move_history, RenderMode::Initial) {
        eprintln!("  Display error: {err}");
    }

    loop {
        let side = if is_white_turn(move_index) {
            "White"
        } else {
            "Black"
        };
        let move_num = full_move_number(move_index);
        write!(stdout, "  [Move {move_num} - {side}] > ").ok();
        stdout.flush().ok();

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => break,
            Err(_) => break,
            _ => {}
        }

        let input = line.trim();
        if input.is_empty() {
            continue;
        }

        let redraw_height = display::layout_height(&*strategy) + 1;

        match input {
            "quit" => break,
            "reset" => {
                board = Board::new();
                move_index = 0;
                move_history.clear();
                if let Err(err) = render_board(
                    &board,
                    &mut stdout,
                    &*strategy,
                    &move_history,
                    RenderMode::Redraw(redraw_height),
                ) {
                    eprintln!("  Display error: {err}");
                }
                continue;
            }
            "display" => {
                writeln!(stdout, "  Usage: display <mode>. Options: sprite, unicode, ascii")
                    .ok();
                stdout.flush().ok();
                continue;
            }
            _ if input.starts_with("display ") => {
                let mode_str = &input["display ".len()..];
                match display::parse_display_mode(mode_str) {
                    Some(mode) => {
                        strategy = display::create_strategy(mode, color_mode);
                        if let Err(err) = render_board(
                            &board,
                            &mut stdout,
                            &*strategy,
                            &move_history,
                            RenderMode::Redraw(redraw_height),
                        ) {
                            eprintln!("  Display error: {err}");
                        }
                    }
                    None => {
                        writeln!(
                            stdout,
                            "  Unknown display mode: {mode_str}. Options: sprite, unicode, ascii"
                        )
                        .ok();
                        stdout.flush().ok();
                    }
                }
                continue;
            }
            _ => {}
        }

        let chess_move = match NotationMove::parse(input, move_index) {
            Some(m) => m,
            None => {
                writeln!(stdout, "  Invalid move: {input}").ok();
                stdout.flush().ok();
                continue;
            }
        };

        let color = if is_white_turn(move_index) {
            Color::White
        } else {
            Color::Black
        };

        let parsed = match board.resolve_move(&chess_move, input, color) {
            Some(p) => p,
            None => {
                writeln!(stdout, "  No piece found for: {input}").ok();
                stdout.flush().ok();
                continue;
            }
        };

        board.apply_move(&parsed);
        move_history.push(input.to_string());

        let samples = audio::synthesize_move(&chess_move);
        let wav = audio::to_wav(&samples);
        audio::play(&wav);

        if let Err(err) = render_board(
            &board,
            &mut stdout,
            &*strategy,
            &move_history,
            RenderMode::Redraw(redraw_height),
        ) {
            eprintln!("  Display error: {err}");
        }
        move_index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::display::AsciiDisplay;

    const NO_MOVES: &[&str] = &[];

    #[test]
    fn render_board_with_moves_writes_sidebar() {
        let board = Board::new();
        let moves = vec!["e4".to_string(), "e5".to_string()];
        let mut buf = Vec::new();
        render_board(&board, &mut buf, &AsciiDisplay, &moves, RenderMode::Initial).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Moves"));
        assert!(output.contains("1. e4    e5"));
    }

    #[test]
    fn render_board_redraw_emits_cursor_up() {
        let board = Board::new();
        let mut buf = Vec::new();
        render_board(&board, &mut buf, &AsciiDisplay, NO_MOVES, RenderMode::Redraw(11)).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(
            output.starts_with("\x1b["),
            "redraw should start with ANSI escape"
        );
        assert!(output.contains("\x1b[J"), "redraw should contain clear");
    }

    #[test]
    fn render_board_first_draw_no_cursor_up() {
        let board = Board::new();
        let mut buf = Vec::new();
        render_board(&board, &mut buf, &AsciiDisplay, NO_MOVES, RenderMode::Initial).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(
            !output.starts_with("\x1b["),
            "first draw should not have ANSI escape at start"
        );
    }
}
