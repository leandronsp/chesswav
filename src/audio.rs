use crate::chess::Move;
use crate::{freq, synth, wav};

pub const SAMPLE_RATE: u32 = 44100;
pub const BITS_PER_SAMPLE: u16 = 16;
pub const NUM_CHANNELS: u16 = 1;
pub const NOTE_DURATION_MS: u32 = 300;
pub const SILENCE_MS: u32 = 50;

pub fn generate(moves: &str) -> Vec<i16> {
    let mut samples: Vec<i16> = Vec::new();
    let silence_samples = (SAMPLE_RATE * SILENCE_MS / 1000) as usize;

    for line in moves.lines() {
        for mov in line.split_whitespace() {
            if let Some(parsed) = Move::parse(mov) {
                let freq = freq::from_square(&parsed.dest);
                let mut note_samples = synth::sine(freq, NOTE_DURATION_MS);
                samples.append(&mut note_samples);
                samples.extend(std::iter::repeat_n(0i16, silence_samples));
            }
        }
    }

    samples
}

pub fn to_wav(samples: &[i16]) -> Vec<u8> {
    let header = wav::header(samples.len() as u32);
    let mut data = Vec::with_capacity(header.len() + samples.len() * 2);
    data.extend_from_slice(&header);
    for sample in samples {
        data.extend_from_slice(&sample.to_le_bytes());
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_empty_input() {
        let samples = generate("");
        assert!(samples.is_empty());
    }

    #[test]
    fn generate_single_move() {
        let samples = generate("e4");
        let expected_note = (SAMPLE_RATE * NOTE_DURATION_MS / 1000) as usize;
        let expected_silence = (SAMPLE_RATE * SILENCE_MS / 1000) as usize;
        assert_eq!(samples.len(), expected_note + expected_silence);
    }

    #[test]
    fn generate_two_moves() {
        let samples = generate("e4 e5");
        let expected_per_move = (SAMPLE_RATE * NOTE_DURATION_MS / 1000) as usize
            + (SAMPLE_RATE * SILENCE_MS / 1000) as usize;
        assert_eq!(samples.len(), expected_per_move * 2);
    }

    #[test]
    fn generate_multiline() {
        let samples = generate("e4\ne5");
        let expected_per_move = (SAMPLE_RATE * NOTE_DURATION_MS / 1000) as usize
            + (SAMPLE_RATE * SILENCE_MS / 1000) as usize;
        assert_eq!(samples.len(), expected_per_move * 2);
    }

    #[test]
    fn to_wav_header_size() {
        let samples = generate("e4");
        let wav_data = to_wav(&samples);
        assert_eq!(&wav_data[0..4], b"RIFF");
        assert_eq!(&wav_data[8..12], b"WAVE");
    }

    #[test]
    fn to_wav_contains_samples() {
        let samples = generate("e4");
        let wav_data = to_wav(&samples);
        assert_eq!(wav_data.len(), 44 + samples.len() * 2);
    }
}
