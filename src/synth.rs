//! Audio synthesis module - generates waveforms from frequencies.
//!
//! # Sine Wave Formula
//!
//! ```text
//! sample[idx] = AMPLITUDE × sin(2π × frequency × idx / SAMPLE_RATE)
//!
//!   Amplitude
//!      32767 │      ╭─╮      ╭─╮
//!            │    ╭╯   ╰╮  ╭╯   ╰╮
//!          0 │───╯       ╰╯       ╰───
//!            │
//!     -32767 │
//!            └────────────────────────→ time
//!                 │← 1 cycle →│
//! ```

use std::f64::consts::PI;

use crate::audio::{MS_PER_SECOND, SAMPLE_RATE};

const AMPLITUDE: f64 = i16::MAX as f64;

/// Generates a sine wave at the given frequency.
///
/// Returns a vector of 16-bit samples.
pub fn sine(freq: u32, duration_ms: u32) -> Vec<i16> {
    let num_samples = (SAMPLE_RATE * duration_ms / MS_PER_SECOND) as usize;
    let angular_freq = 2.0 * PI * freq as f64 / SAMPLE_RATE as f64;

    (0..num_samples)
        .map(|idx| (AMPLITUDE * (angular_freq * idx as f64).sin()) as i16)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_count_100ms() {
        assert_eq!(sine(440, 100).len(), 4410);
    }

    #[test]
    fn sample_count_300ms() {
        assert_eq!(sine(440, 300).len(), 13230);
    }

    #[test]
    fn samples_within_amplitude_range() {
        for &s in &sine(440, 100) {
            assert!(s >= i16::MIN && s <= i16::MAX);
        }
    }

    #[test]
    fn sine_wave_starts_near_zero() {
        assert!(sine(440, 100)[0].abs() < 100);
    }

    #[test]
    fn different_frequencies_differ() {
        assert_ne!(sine(440, 50), sine(880, 50));
    }
}
