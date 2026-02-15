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

fn render_board(board: &Board, color_mode: display::ColorMode) {
    let mut writer = BufWriter::new(io::stdout());
    if let Err(err) = display::render(board, &mut writer, color_mode) {
        eprintln!("  Display error: {err}");
        return;
    }
    if let Err(err) = writer.flush() {
        eprintln!("  Display error: {err}");
    }
}

pub fn run() {
    let mut board = Board::new();
    let mut move_index: usize = 0;

    println!();
    println!("  ChessWAV Interactive Mode");
    println!("  Type moves in algebraic notation. Commands: reset, quit");
    println!();

    let color_mode = display::detect_color_mode();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    render_board(&board, color_mode);

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

        render_board(&board, color_mode);
        move_index += 1;
    }
}
