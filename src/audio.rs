//! Audio generation - orchestrates chess moves to WAV output.
//!
//! # Pipeline
//!
//! ```text
//! "e4 Nf3"
//!     │
//!     ▼ NotationMove::parse()
//! [NotationMove, NotationMove]
//!     │
//!     ▼ freq::from_square()
//! [392 Hz, 349 Hz]
//!     │
//!     ▼ synth::sine()
//! [samples...] + silence
//!     │
//!     ▼ wav::header()
//! [WAV file bytes]
//! ```

use crate::blend::Blend;
use crate::chess::{NotationMove, Piece, Threat};
use crate::{freq, synth, wav};

// Audio format constants
pub const SAMPLE_RATE: u32 = 44100;
pub const BITS_PER_SAMPLE: u16 = 16;
pub const BYTES_PER_SAMPLE: usize = (BITS_PER_SAMPLE / 8) as usize;
pub const NUM_CHANNELS: u16 = 1;
pub const MS_PER_SECOND: u32 = 1000;

// Timing constants
const NOTE_MS: u32 = 300;
const SILENCE_MS: u32 = 50;

/// Converts chess notation to audio samples. Input is a string of chess moves,
/// e.g. "e4 e5 Nf3 Nc6".
pub fn generate(input: &str) -> Vec<i16> {
    // Generates silence samples for the specified duration.
    // E.g vec![0, 0, 0, ...] for 50 ms.
    let silence: Vec<i16> = vec![0; (SAMPLE_RATE * SILENCE_MS / MS_PER_SECOND) as usize];

    input
        .split_whitespace()
        .enumerate()
        .filter_map(|(idx, notation)| NotationMove::parse(notation, idx))
        .flat_map(|m| move_to_samples(&m, &silence))
        .collect()
}

pub fn synthesize_move(m: &NotationMove) -> Vec<i16> {
    let silence: Vec<i16> = vec![0; (SAMPLE_RATE * SILENCE_MS / MS_PER_SECOND) as usize];
    move_to_samples(m, &silence)
}

pub fn play(wav: &[u8]) {
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

    std::fs::remove_file(&path).ok();
}

fn move_to_samples(m: &NotationMove, silence: &[i16]) -> Vec<i16> {
    let freq: u32 = freq::from_square(&m.dest);
    let piece = m.promotion.unwrap_or(m.piece);
    let note: Vec<i16> = match (piece, m.threat) {
        (Piece::Pawn, Threat::None) => synth::sine(freq, NOTE_MS),
        (Piece::Pawn, Threat::Check) => synth::triangle(freq, NOTE_MS, Blend::with_sine(0.7)),
        (Piece::Pawn, Threat::Checkmate) => synth::triangle(freq, NOTE_MS, Blend::with_sine(0.9)),
        (Piece::Knight, Threat::None) => synth::triangle(freq, NOTE_MS, Blend::none()),
        (Piece::Knight, Threat::Check) => synth::triangle(freq, NOTE_MS, Blend::with_sine(0.4)),
        (Piece::Knight, Threat::Checkmate) => synth::triangle(freq, NOTE_MS, Blend::with_sine(0.7)),
        (Piece::Rook, Threat::None) => synth::square(freq, NOTE_MS, Blend::with_sine_and_band_limit(0.4, 7)),
        (Piece::Rook, Threat::Check) => synth::square(freq, NOTE_MS, Blend::with_sine_and_band_limit(0.6, 3)),
        (Piece::Rook, Threat::Checkmate) => synth::square(freq, NOTE_MS, Blend::with_sine_and_band_limit(0.8, 2)),
        (Piece::Bishop, Threat::None) => synth::sawtooth(freq, NOTE_MS, Blend::with_sine_and_band_limit(0.3, 8)),
        (Piece::Bishop, Threat::Check) => synth::sawtooth(freq, NOTE_MS, Blend::with_sine_and_band_limit(0.5, 3)),
        (Piece::Bishop, Threat::Checkmate) => synth::sawtooth(freq, NOTE_MS, Blend::with_sine_and_band_limit(0.7, 2)),
        (Piece::Queen, Threat::None) => synth::composite(freq, NOTE_MS, Blend::none()),
        (Piece::Queen, Threat::Check) => synth::composite(freq, NOTE_MS, Blend::with_sine_and_band_limit(0.4, 3)),
        (Piece::Queen, Threat::Checkmate) => synth::composite(freq, NOTE_MS, Blend::with_sine_and_band_limit(0.6, 2)),
        (Piece::King, Threat::None) => synth::harmonics(freq, NOTE_MS, Blend::none()),
        (Piece::King, Threat::Check) => synth::harmonics(freq, NOTE_MS, Blend::none()),
        (Piece::King, Threat::Checkmate) => synth::harmonics(freq, NOTE_MS, Blend::with_sine(0.5)),
    };

    note.into_iter().chain(silence.iter().copied()).collect()
}

/// Converts samples to WAV file format.
pub fn to_wav(samples: &[i16]) -> Vec<u8> {
    let mut data = Vec::with_capacity(wav::HEADER_SIZE + samples.len() * BYTES_PER_SAMPLE);
    data.extend_from_slice(&wav::header(samples.len() as u32));
    data.extend(samples.iter().flat_map(|s| s.to_le_bytes()));
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLES_PER_MOVE: usize = (SAMPLE_RATE * (NOTE_MS + SILENCE_MS) / MS_PER_SECOND) as usize;

    #[test]
    fn empty_input() {
        assert!(generate("").is_empty());
    }

    #[test]
    fn single_move() {
        assert_eq!(generate("e4").len(), SAMPLES_PER_MOVE);
    }

    #[test]
    fn two_moves() {
        assert_eq!(generate("e4 e5").len(), SAMPLES_PER_MOVE * 2);
    }

    #[test]
    fn multiline() {
        assert_eq!(generate("e4\ne5").len(), SAMPLES_PER_MOVE * 2);
    }

    #[test]
    fn wav_has_riff_header() {
        let wav = to_wav(&generate("e4"));
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
    }

    #[test]
    fn wav_size() {
        let samples = generate("e4");
        let wav = to_wav(&samples);
        assert_eq!(
            wav.len(),
            wav::HEADER_SIZE + samples.len() * BYTES_PER_SAMPLE
        );
    }

    #[test]
    fn check_produces_different_samples() {
        let normal = generate("Nf3");
        let check = generate("Nf3+");
        assert_ne!(normal, check);
    }

    #[test]
    fn check_same_length_as_normal() {
        let normal = generate("Nf3");
        let check = generate("Nf3+");
        assert_eq!(normal.len(), check.len());
    }

    #[test]
    fn checkmate_produces_different_samples() {
        let check = generate("Qf7+");
        let checkmate = generate("Qf7#");
        assert_ne!(check, checkmate);
    }

    #[test]
    fn promotion_uses_promoted_piece_timbre() {
        let pawn = generate("e8");
        let promoted = generate("e8=Q");
        assert_ne!(pawn, promoted);
    }
}
