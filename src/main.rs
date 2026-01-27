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
//! # From a file
//! cargo run --release < moves.txt > game.wav
//!
//! # After `cargo install --path .`
//! echo "e4 e5 Nf3 Nc6" | chesswav > game.wav
//! echo "e4 e5 Nf3 Nc6" | chesswav --play
//! ```

use std::io::{self, Read, Write};

use chesswav::audio;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let play_mode = args.iter().any(|a| a == "--play" || a == "-p");

    // Read chess notation from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    // Generate WAV audio
    let samples = audio::generate(&input);
    let wav = audio::to_wav(&samples);

    if play_mode {
        play(&wav);
    } else {
        // Write WAV to stdout (for piping to file)
        io::stdout().lock().write_all(&wav).ok();
    }
}

/// Plays WAV audio using system player.
///
/// Creates a temp file because audio players need a file path.
fn play(wav: &[u8]) {
    let path = std::env::temp_dir().join("chesswav.wav");
    std::fs::write(&path, wav).expect("Failed to write temp file");

    #[cfg(target_os = "macos")]
    std::process::Command::new("afplay")
        .arg(&path)
        .status()
        .expect("Failed to play audio");

    #[cfg(target_os = "linux")]
    std::process::Command::new("aplay")
        .args(["-f", "S16_LE", "-r", "44100", "-c", "1"])
        .arg(&path)
        .status()
        .expect("Failed to play audio");

    // Cleanup
    std::fs::remove_file(&path).ok();
}
