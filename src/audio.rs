//! Audio generation - orchestrates chess moves to WAV output.
//!
//! # Pipeline
//!
//! ```text
//! "e4 Nf3"
//!     │
//!     ▼ Move::parse()
//! [Move, Move]
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

use crate::chess::Move;
use crate::{freq, synth, wav};

// Audio format constants
pub const SAMPLE_RATE: u32 = 44100;
pub const BITS_PER_SAMPLE: u16 = 16;
pub const NUM_CHANNELS: u16 = 1;

// Timing constants
const NOTE_MS: u32 = 300;
const SILENCE_MS: u32 = 50;

/// Converts chess notation to audio samples. Input is a string of chess moves,
/// e.g. "e4 e5 Nf3 Nc6".
pub fn generate(input: &str) -> Vec<i16> {
    // Generates silence samples for the specified duration.
    // E.g vec![0, 0, 0, ...] for 50 ms.
    let silence: Vec<i16> = vec![0; (SAMPLE_RATE * SILENCE_MS / 1000) as usize];

    input
        .split_whitespace()
        .filter_map(Move::parse)
        .flat_map(|m| move_to_samples(&m, &silence))
        .collect()
}

fn move_to_samples(m: &Move, silence: &[i16]) -> Vec<i16> {
    let freq = freq::from_square(&m.dest);
    let note = synth::sine(freq, NOTE_MS);

    note.into_iter().chain(silence.iter().copied()).collect()
}

/// Converts samples to WAV file format.
pub fn to_wav(samples: &[i16]) -> Vec<u8> {
    let mut data = Vec::with_capacity(wav::HEADER_SIZE + samples.len() * 2);
    data.extend_from_slice(&wav::header(samples.len() as u32));
    data.extend(samples.iter().flat_map(|s| s.to_le_bytes()));
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLES_PER_MOVE: usize = (SAMPLE_RATE * (NOTE_MS + SILENCE_MS) / 1000) as usize;

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
        assert_eq!(wav.len(), wav::HEADER_SIZE + samples.len() * 2);
    }
}
