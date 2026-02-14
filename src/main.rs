//! ChessWAV CLI - converts chess notation to audio.
//!
//! # Usage
//!
//! ```text
//! # Generate WAV file
//! echo "e4 e5 Nf3 Nc6" | cargo run --release > game.wav
//!
//! # Play audio directly (macOS/Linux)
//! echo "e4 e5 Nf3 Nc6" | cargo run --release -- --play
//! echo "e4 e5 Nf3 Nc6" | cargo run --release -- -p
//!
//! # Interactive mode
//! cargo run --release -- --interactive
//! cargo run --release -- -i
//!
//! # From a file
//! cargo run --release < moves.txt > game.wav
//!
//! # After `cargo install --path .`
//! echo "e4 e5 Nf3 Nc6" | chesswav > game.wav
//! echo "e4 e5 Nf3 Nc6" | chesswav --play
//! chesswav --interactive
//! ```

use std::io::{self, Read, Write};

use chesswav::audio;
use chesswav::repl;

fn main() {
    // CLI modes: --play/-p for audio playback, --interactive/-i for REPL
    let args: Vec<String> = std::env::args().collect();
    let play_mode: bool = args.iter().any(|a| a == "--play" || a == "-p");
    let interactive: bool = args.iter().any(|a| a == "--interactive" || a == "-i");

    if interactive {
        repl::run();
        return;
    }

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let samples: Vec<i16> = audio::generate(&input);
    let wav: Vec<u8> = audio::to_wav(&samples);

    if play_mode {
        audio::play(&wav);
    } else {
        io::stdout().lock().write_all(&wav).ok();
    }
}
