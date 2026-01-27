//! Audio synthesis module - generates waveforms from frequencies.
//!
//! # Sine Wave Formula
//!
//! ```text
//! sample[i] = AMPLITUDE × sin(2π × frequency × i / SAMPLE_RATE)
//!
//!   Amplitude
//!      28000 │      ╭─╮      ╭─╮
//!            │    ╭╯   ╰╮  ╭╯   ╰╮
//!          0 │───╯       ╰╯       ╰───
//!            │
//!     -28000 │
//!            └────────────────────────→ time
//!                 │← 1 cycle →│
//! ```

use std::f64::consts::PI;

use crate::audio::SAMPLE_RATE;

/// ~85% of i16::MAX to avoid clipping
const AMPLITUDE: f64 = 28000.0;

/// Generates a sine wave at the given frequency.
///
/// Returns a vector of 16-bit samples.
pub fn sine(freq: u32, duration_ms: u32) -> Vec<i16> {
    let num_samples = (SAMPLE_RATE * duration_ms / 1000) as usize;
    let angular_freq = 2.0 * PI * freq as f64 / SAMPLE_RATE as f64;

    (0..num_samples)
        .map(|i| (AMPLITUDE * (angular_freq * i as f64).sin()) as i16)
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
            assert!(s >= -28000 && s <= 28000);
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
