//! Audio synthesis module - generates waveforms from frequencies.

use std::f64::consts::PI;

use super::{MS_PER_SECOND, SAMPLE_RATE};
use super::blend::Blend;
use super::waveform::{Composite, Harmonics, Sawtooth, Sine, Square, Triangle, Waveform};

const AMPLITUDE: f64 = i16::MAX as f64;

/// Generate samples from a waveform with blending options.
pub fn generate<W: Waveform>(wave: &W, freq: u32, duration_ms: u32, blend: Blend) -> Vec<i16> {
    let num_samples = (SAMPLE_RATE * duration_ms / MS_PER_SECOND) as usize;
    let angular_freq = 2.0 * PI * freq as f64 / SAMPLE_RATE as f64;

    (0..num_samples)
        .map(|idx| {
            let phase = angular_freq * idx as f64;
            let value = blend.apply(wave, phase);
            (value * AMPLITUDE) as i16
        })
        .collect()
}

/// Generates a sine wave at the given frequency.
pub fn sine(freq: u32, duration_ms: u32) -> Vec<i16> {
    generate(&Sine, freq, duration_ms, Blend::none())
}

/// Generates a square wave with optional blending.
pub fn square(freq: u32, duration_ms: u32, blend: Blend) -> Vec<i16> {
    generate(&Square, freq, duration_ms, blend)
}

/// Generates a triangle wave with optional blending.
pub fn triangle(freq: u32, duration_ms: u32, blend: Blend) -> Vec<i16> {
    generate(&Triangle, freq, duration_ms, blend)
}

/// Generates a sawtooth wave with optional blending.
pub fn sawtooth(freq: u32, duration_ms: u32, blend: Blend) -> Vec<i16> {
    generate(&Sawtooth, freq, duration_ms, blend)
}

/// Generates a composite wave with optional blending.
pub fn composite(freq: u32, duration_ms: u32, blend: Blend) -> Vec<i16> {
    generate(&Composite, freq, duration_ms, blend)
}

/// Generates a harmonics wave with optional blending.
pub fn harmonics(freq: u32, duration_ms: u32, blend: Blend) -> Vec<i16> {
    generate(&Harmonics, freq, duration_ms, blend)
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

    #[test]
    fn triangle_sample_count() {
        assert_eq!(triangle(440, 100, Blend::none()).len(), 4410);
    }

    #[test]
    fn triangle_within_amplitude_range() {
        for &s in &triangle(440, 100, Blend::none()) {
            assert!(s >= i16::MIN && s <= i16::MAX);
        }
    }

    #[test]
    fn triangle_differs_from_sine() {
        assert_ne!(sine(440, 100), triangle(440, 100, Blend::none()));
    }

    #[test]
    fn square_sample_count() {
        assert_eq!(square(440, 100, Blend::none()).len(), 4410);
    }

    #[test]
    fn square_within_amplitude_range() {
        for &s in &square(440, 100, Blend::none()) {
            assert!(s >= i16::MIN && s <= i16::MAX);
        }
    }

    #[test]
    fn square_differs_from_sine() {
        assert_ne!(sine(440, 100), square(440, 100, Blend::none()));
    }

    #[test]
    fn sawtooth_sample_count() {
        assert_eq!(sawtooth(440, 100, Blend::none()).len(), 4410);
    }

    #[test]
    fn sawtooth_within_amplitude_range() {
        for &s in &sawtooth(440, 100, Blend::none()) {
            assert!(s >= i16::MIN && s <= i16::MAX);
        }
    }

    #[test]
    fn sawtooth_differs_from_sine() {
        assert_ne!(sine(440, 100), sawtooth(440, 100, Blend::none()));
    }

    #[test]
    fn composite_sample_count() {
        assert_eq!(composite(440, 100, Blend::none()).len(), 4410);
    }

    #[test]
    fn composite_within_amplitude_range() {
        for &s in &composite(440, 100, Blend::none()) {
            assert!(s >= i16::MIN && s <= i16::MAX);
        }
    }

    #[test]
    fn harmonics_sample_count() {
        assert_eq!(harmonics(440, 100, Blend::none()).len(), 4410);
    }

    #[test]
    fn harmonics_within_amplitude_range() {
        for &s in &harmonics(440, 100, Blend::none()) {
            assert!(s >= i16::MIN && s <= i16::MAX);
        }
    }
}
