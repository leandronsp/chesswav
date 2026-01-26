use crate::audio::{BITS_PER_SAMPLE, NUM_CHANNELS, SAMPLE_RATE};

pub fn header(num_samples: u32) -> Vec<u8> {
    let bytes_per_sample = BITS_PER_SAMPLE / 8;
    let block_align = NUM_CHANNELS * bytes_per_sample;
    let byte_rate = SAMPLE_RATE * block_align as u32;
    let data_size = num_samples * block_align as u32;
    let chunk_size = 36 + data_size;

    let mut buf = Vec::with_capacity(44);

    buf.extend_from_slice(b"RIFF");
    buf.extend_from_slice(&chunk_size.to_le_bytes());
    buf.extend_from_slice(b"WAVE");

    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(&16u32.to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&NUM_CHANNELS.to_le_bytes());
    buf.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    buf.extend_from_slice(&byte_rate.to_le_bytes());
    buf.extend_from_slice(&block_align.to_le_bytes());
    buf.extend_from_slice(&BITS_PER_SAMPLE.to_le_bytes());

    buf.extend_from_slice(b"data");
    buf.extend_from_slice(&data_size.to_le_bytes());

    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_size() {
        let h = header(1000);
        assert_eq!(h.len(), 44);
    }

    #[test]
    fn riff_marker() {
        let h = header(1000);
        assert_eq!(&h[0..4], b"RIFF");
    }

    #[test]
    fn wave_marker() {
        let h = header(1000);
        assert_eq!(&h[8..12], b"WAVE");
    }

    #[test]
    fn fmt_marker() {
        let h = header(1000);
        assert_eq!(&h[12..16], b"fmt ");
    }

    #[test]
    fn data_marker() {
        let h = header(1000);
        assert_eq!(&h[36..40], b"data");
    }

    #[test]
    fn chunk_size_correct() {
        let h = header(1000);
        let chunk_size = u32::from_le_bytes([h[4], h[5], h[6], h[7]]);
        assert_eq!(chunk_size, 36 + 2000);
    }

    #[test]
    fn data_size_correct() {
        let h = header(1000);
        let data_size = u32::from_le_bytes([h[40], h[41], h[42], h[43]]);
        assert_eq!(data_size, 2000);
    }

    #[test]
    fn sample_rate_correct() {
        let h = header(1000);
        let sr = u32::from_le_bytes([h[24], h[25], h[26], h[27]]);
        assert_eq!(sr, 44100);
    }

    #[test]
    fn byte_layout() {
        let expected: [u8; 44] = [
            0x52, 0x49, 0x46, 0x46,
            0xf4, 0x07, 0x00, 0x00,
            0x57, 0x41, 0x56, 0x45,
            0x66, 0x6d, 0x74, 0x20,
            0x10, 0x00, 0x00, 0x00,
            0x01, 0x00,
            0x01, 0x00,
            0x44, 0xac, 0x00, 0x00,
            0x88, 0x58, 0x01, 0x00,
            0x02, 0x00,
            0x10, 0x00,
            0x64, 0x61, 0x74, 0x61,
            0xd0, 0x07, 0x00, 0x00,
        ];
        let h = header(1000);
        assert_eq!(h.as_slice(), &expected);
    }
}
