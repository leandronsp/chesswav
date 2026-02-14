use std::io::{self, BufRead, Write};

use crate::audio;
use crate::board::{Board, Color, ParsedMove};
use crate::chess::{Move, Piece, Square};

pub fn run() {
    let mut board = Board::new();
    let mut move_index: usize = 0;

    println!();
    println!("  ChessWAV Interactive Mode");
    println!("  Type moves in algebraic notation. Commands: reset, quit");
    println!();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let side = if move_index % 2 == 0 {
            "White"
        } else {
            "Black"
        };
        let move_num = move_index / 2 + 1;
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

        let chess_move = match Move::parse(input, move_index) {
            Some(m) => m,
            None => {
                println!("  Invalid move: {input}\n");
                continue;
            }
        };

        let color = if move_index % 2 == 0 {
            Color::White
        } else {
            Color::Black
        };

        let parsed = match resolve_move(&board, &chess_move, input, color) {
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

        println!("{board}");
        move_index += 1;
    }
}

fn resolve_move(
    board: &Board,
    chess_move: &Move,
    notation: &str,
    color: Color,
) -> Option<ParsedMove> {
    let clean = strip_for_hints(notation);

    if is_castling(notation) {
        return resolve_castling(chess_move, color);
    }

    let (file_hint, rank_hint) = extract_hints(&clean, chess_move.piece);

    let origin = board.find_origin(
        chess_move.piece,
        &chess_move.dest,
        color,
        file_hint,
        rank_hint,
    )?;

    Some(ParsedMove {
        origin,
        dest: chess_move.dest,
        promotion: chess_move.promotion,
        castling_rook: None,
    })
}

fn is_castling(notation: &str) -> bool {
    let clean: String = notation
        .chars()
        .filter(|c| !matches!(c, '+' | '#'))
        .collect();
    clean == "O-O" || clean == "O-O-O"
}

fn resolve_castling(chess_move: &Move, color: Color) -> Option<ParsedMove> {
    let rank = match color {
        Color::White => 0,
        Color::Black => 7,
    };

    let kingside = chess_move.dest.file == 6;
    let (rook_from, rook_to) = if kingside {
        (Square { file: 7, rank }, Square { file: 5, rank })
    } else {
        (Square { file: 0, rank }, Square { file: 3, rank })
    };

    Some(ParsedMove {
        origin: Square { file: 4, rank },
        dest: chess_move.dest,
        promotion: None,
        castling_rook: Some((rook_from, rook_to)),
    })
}

fn strip_for_hints(notation: &str) -> String {
    notation
        .split('=')
        .next()
        .unwrap_or(notation)
        .chars()
        .filter(|c| !matches!(c, '+' | '#' | '!' | '?' | 'x' | '-'))
        .collect()
}

fn extract_hints(clean: &str, piece: Piece) -> (Option<u8>, Option<u8>) {
    if piece == Piece::Pawn {
        return extract_pawn_hints(clean);
    }

    // For pieces: first char is piece letter, last 2 are destination.
    // Anything in between is disambiguation.
    if clean.len() <= 3 {
        return (None, None);
    }

    let middle = &clean[1..clean.len() - 2];
    let mut file_hint = None;
    let mut rank_hint = None;

    for c in middle.chars() {
        if ('a'..='h').contains(&c) {
            file_hint = Some(c as u8 - b'a');
        } else if ('1'..='8').contains(&c) {
            rank_hint = Some(c as u8 - b'1');
        }
    }

    (file_hint, rank_hint)
}

fn extract_pawn_hints(clean: &str) -> (Option<u8>, Option<u8>) {
    // Pawn captures like "exd5" → clean is "ed5", file hint is 'e' (file 4)
    if clean.len() > 2 {
        let first = clean.chars().next().unwrap();
        if ('a'..='h').contains(&first) {
            return (Some(first as u8 - b'a'), None);
        }
    }
    (None, None)
}
