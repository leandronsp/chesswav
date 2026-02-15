//! WAV file format encoder.
//!
//! # RIFF/WAVE Structure (44-byte header)
//!
//! ```text
//! Offset  Size  Description
//! ──────────────────────────────────────────
//! 0       4     "RIFF" marker
//! 4       4     File size - 8
//! 8       4     "WAVE" marker
//! ──────────────────────────────────────────
//! 12      4     "fmt " marker
//! 16      4     Format chunk size (16)
//! 20      2     Audio format (1 = PCM)
//! 22      2     Number of channels
//! 24      4     Sample rate
//! 28      4     Byte rate
//! 32      2     Block align
//! 34      2     Bits per sample
//! ──────────────────────────────────────────
//! 36      4     "data" marker
//! 40      4     Data size
//! 44      ...   Sample data (little-endian)
//! ```

use super::{BITS_PER_SAMPLE, NUM_CHANNELS, SAMPLE_RATE};

pub const HEADER_SIZE: usize = 44;

/// Generates a 44-byte WAV header for the given number of samples.
pub fn header(num_samples: u32) -> [u8; HEADER_SIZE] {
    let block_align = NUM_CHANNELS * (BITS_PER_SAMPLE / 8);
    let byte_rate = SAMPLE_RATE * block_align as u32;
    let data_size = num_samples * block_align as u32;

    let mut h = [0u8; HEADER_SIZE];

    // RIFF chunk
    h[0..4].copy_from_slice(b"RIFF");
    h[4..8].copy_from_slice(&(36 + data_size).to_le_bytes());
    h[8..12].copy_from_slice(b"WAVE");

    // fmt subchunk
    h[12..16].copy_from_slice(b"fmt ");
    h[16..20].copy_from_slice(&16u32.to_le_bytes());
    h[20..22].copy_from_slice(&1u16.to_le_bytes()); // PCM
    h[22..24].copy_from_slice(&NUM_CHANNELS.to_le_bytes());
    h[24..28].copy_from_slice(&SAMPLE_RATE.to_le_bytes());
    h[28..32].copy_from_slice(&byte_rate.to_le_bytes());
    h[32..34].copy_from_slice(&block_align.to_le_bytes());
    h[34..36].copy_from_slice(&BITS_PER_SAMPLE.to_le_bytes());

    // data subchunk
    h[36..40].copy_from_slice(b"data");
    h[40..44].copy_from_slice(&data_size.to_le_bytes());

    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn riff_marker() {
        assert_eq!(&header(1000)[0..4], b"RIFF");
    }

    #[test]
    fn wave_marker() {
        assert_eq!(&header(1000)[8..12], b"WAVE");
    }

    #[test]
    fn fmt_marker() {
        assert_eq!(&header(1000)[12..16], b"fmt ");
    }

    #[test]
    fn data_marker() {
        assert_eq!(&header(1000)[36..40], b"data");
    }

    #[test]
    fn chunk_size() {
        let h = header(1000);
        let size = u32::from_le_bytes([h[4], h[5], h[6], h[7]]);
        assert_eq!(size, 36 + 2000); // 1000 samples * 2 bytes
    }

    #[test]
    fn data_size() {
        let h = header(1000);
        let size = u32::from_le_bytes([h[40], h[41], h[42], h[43]]);
        assert_eq!(size, 2000);
    }

    #[test]
    fn sample_rate() {
        let h = header(1000);
        let sr = u32::from_le_bytes([h[24], h[25], h[26], h[27]]);
        assert_eq!(sr, 44100);
    }
}
