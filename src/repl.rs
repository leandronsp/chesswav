use std::io::{self, BufRead, BufWriter, Write};

use crate::audio;
use crate::board::{Board, Color};
use crate::chess::NotationMove;
use crate::display;

fn is_white_turn(move_index: usize) -> bool {
    move_index % 2 == 0
}

fn full_move_number(move_index: usize) -> usize {
    move_index / 2 + 1
}

fn render_board(board: &Board, strategy: &dyn display::DisplayStrategy) {
    let mut writer = BufWriter::new(io::stdout());
    if let Err(err) = display::render(board, &mut writer, strategy) {
        eprintln!("  Display error: {err}");
        return;
    }
    if let Err(err) = writer.flush() {
        eprintln!("  Display error: {err}");
    }
}

pub fn run(initial_mode: display::DisplayMode) {
    let mut board = Board::new();
    let mut move_index: usize = 0;

    println!();
    println!("  ChessWAV Interactive Mode");
    println!("  Type moves in algebraic notation. Commands: display, reset, quit");
    println!();

    let color_mode = display::detect_color_mode();
    // Box<dyn ...> stores the strategy on the heap behind a vtable pointer,
    // allowing reassignment to a different concrete type at runtime.
    let mut strategy: Box<dyn display::DisplayStrategy> =
        display::create_strategy(initial_mode, color_mode);
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    render_board(&board, &*strategy);

    loop {
        let side = if is_white_turn(move_index) {
            "White"
        } else {
            "Black"
        };
        let move_num = full_move_number(move_index);
        print!("  [Move {move_num} - {side}] > ");
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

        match input {
            "quit" => break,
            "reset" => {
                board = Board::new();
                move_index = 0;
                println!("  Game reset.\n");
                render_board(&board, &*strategy);
                continue;
            }
            "display" => {
                println!("  Usage: display <mode>. Options: sprite, unicode, ascii\n");
                continue;
            }
            _ if input.starts_with("display ") => {
                let mode_str = &input["display ".len()..];
                match display::parse_display_mode(mode_str) {
                    Some(mode) => {
                        strategy = display::create_strategy(mode, color_mode);
                        println!("  Switched to {mode_str} display.\n");
                        render_board(&board, &*strategy);
                    }
                    None => {
                        println!("  Unknown display mode: {mode_str}. Options: sprite, unicode, ascii\n");
                    }
                }
                continue;
            }
            _ => {}
        }

        let chess_move = match NotationMove::parse(input, move_index) {
            Some(m) => m,
            None => {
                println!("  Invalid move: {input}\n");
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
                println!("  No piece found for: {input}\n");
                continue;
            }
        };

        board.apply_move(&parsed);

        let samples = audio::synthesize_move(&chess_move);
        let wav = audio::to_wav(&samples);
        audio::play(&wav);

        render_board(&board, &*strategy);
        move_index += 1;
    }
}
