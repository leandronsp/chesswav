use std::io::{self, BufRead, BufWriter, Write};

use crate::audio;
use crate::engine::board::{Board, Color};
use crate::engine::chess::{NotationMove, Threat};
use super::display;

fn is_white_turn(move_index: usize) -> bool {
    move_index % 2 == 0
}

fn full_move_number(move_index: usize) -> usize {
    move_index / 2 + 1
}

#[derive(Debug, PartialEq)]
enum PostGameAction {
    NewGame,
    Quit,
}

/// Returns a message based on precomputed check/checkmate state.
fn format_threat_message(in_check: bool, is_checkmate: bool, opponent_color: Color) -> Option<String> {
    if is_checkmate {
        let winner = match opponent_color {
            Color::White => "Black",
            Color::Black => "White",
        };
        Some(format!("  Checkmate! {winner} wins!"))
    } else if in_check {
        Some("  Check!".to_string())
    } else {
        None
    }
}

fn parse_post_game_choice(input: &str) -> Option<PostGameAction> {
    match input {
        "new" | "reset" => Some(PostGameAction::NewGame),
        "quit" => Some(PostGameAction::Quit),
        _ => None,
    }
}

/// Validates notation threat annotation against precomputed board state.
/// Returns a warning message on mismatch.
fn validate_threat(notation_threat: Threat, in_check: bool, is_checkmate: bool) -> Option<String> {
    match notation_threat {
        Threat::Checkmate if !is_checkmate => {
            Some("  Warning: move annotated as checkmate but board disagrees".to_string())
        }
        Threat::Check if !in_check => {
            Some("  Warning: move annotated as check but board disagrees".to_string())
        }
        Threat::None if in_check || is_checkmate => {
            Some("  Warning: check/checkmate detected but not annotated in notation".to_string())
        }
        _ => None,
    }
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
    let mut extra_lines: usize = 0;

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

        let redraw_height = display::layout_height(&*strategy) + 1 + extra_lines;
        extra_lines = 0;

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

        let opponent_color = color.opponent();
        let in_check = board.is_in_check(opponent_color);
        let checkmate = in_check && board.is_checkmate(opponent_color);

        if let Some(warning) = validate_threat(chess_move.threat, in_check, checkmate) {
            writeln!(stdout, "{warning}").ok();
            stdout.flush().ok();
            extra_lines += 1;
        }

        if let Some(message) = format_threat_message(in_check, checkmate, opponent_color) {
            writeln!(stdout, "{message}").ok();
            stdout.flush().ok();
            extra_lines += 1;

            if checkmate {
                writeln!(stdout, "  New game (new) or quit? >").ok();
                stdout.flush().ok();
                // prompt line
                extra_lines += 1;

                loop {
                    let mut post_line = String::new();
                    match stdin.lock().read_line(&mut post_line) {
                        Ok(0) => return,
                        Err(_) => return,
                        _ => {}
                    }
                    // user input line
                    extra_lines += 1;
                    match parse_post_game_choice(post_line.trim()) {
                        Some(PostGameAction::NewGame) => {
                            board = Board::new();
                            move_index = 0;
                            move_history.clear();
                            let height = display::layout_height(&*strategy)
                                + 1
                                + extra_lines;
                            extra_lines = 0;
                            if let Err(err) = render_board(
                                &board,
                                &mut stdout,
                                &*strategy,
                                &move_history,
                                RenderMode::Redraw(height),
                            ) {
                                eprintln!("  Display error: {err}");
                            }
                            break;
                        }
                        Some(PostGameAction::Quit) => return,
                        None => {
                            writeln!(stdout, "  Please type 'new' or 'quit'").ok();
                            stdout.flush().ok();
                            extra_lines += 1;
                        }
                    }
                }
                continue;
            }
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
    fn format_threat_no_check() {
        let message = format_threat_message(false, false, Color::Black);
        assert_eq!(message, None);
    }

    #[test]
    fn format_threat_check_when_in_check() {
        let message = format_threat_message(true, false, Color::Black);
        assert_eq!(message, Some("  Check!".to_string()));
    }

    #[test]
    fn format_threat_checkmate_message() {
        let message = format_threat_message(true, true, Color::White);
        assert_eq!(message, Some("  Checkmate! Black wins!".to_string()));
    }

    #[test]
    fn parse_post_game_new() {
        assert_eq!(parse_post_game_choice("new"), Some(PostGameAction::NewGame));
    }

    #[test]
    fn parse_post_game_reset() {
        assert_eq!(parse_post_game_choice("reset"), Some(PostGameAction::NewGame));
    }

    #[test]
    fn parse_post_game_quit() {
        assert_eq!(parse_post_game_choice("quit"), Some(PostGameAction::Quit));
    }

    #[test]
    fn parse_post_game_invalid() {
        assert_eq!(parse_post_game_choice("blah"), None);
    }

    #[test]
    fn validate_threat_checkmate_annotation_but_not_actual() {
        let warning = validate_threat(Threat::Checkmate, false, false);
        assert!(warning.is_some());
    }

    #[test]
    fn validate_threat_check_annotation_confirmed() {
        let warning = validate_threat(Threat::Check, true, false);
        assert!(warning.is_none());
    }

    #[test]
    fn validate_threat_missing_check_annotation() {
        let warning = validate_threat(Threat::None, true, false);
        assert!(warning.is_some());
    }

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
