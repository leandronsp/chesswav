use std::f64::consts::PI;

use crate::audio::SAMPLE_RATE;

const AMPLITUDE: f64 = 28000.0;

pub fn sine(frequency: u32, duration_ms: u32) -> Vec<i16> {
    let num_samples = (SAMPLE_RATE * duration_ms / 1000) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    let angular_freq = 2.0 * PI * frequency as f64 / SAMPLE_RATE as f64;

    for i in 0..num_samples {
        let t = i as f64;
        let sample = (AMPLITUDE * (angular_freq * t).sin()) as i16;
        samples.push(sample);
    }

    samples
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_count_100ms() {
        let samples = sine(440, 100);
        assert_eq!(samples.len(), 4410);
    }

    #[test]
    fn sample_count_300ms() {
        let samples = sine(440, 300);
        assert_eq!(samples.len(), 13230);
    }

    #[test]
    fn output_non_empty() {
        let samples = sine(440, 10);
        assert!(!samples.is_empty());
    }

    #[test]
    fn different_frequencies_produce_different_output() {
        let samples_440 = sine(440, 50);
        let samples_880 = sine(880, 50);
        assert_ne!(samples_440, samples_880);
    }

    #[test]
    fn samples_within_amplitude_range() {
        let samples = sine(440, 100);
        for &s in &samples {
            assert!(s >= -28000 && s <= 28000);
        }
    }

    #[test]
    fn sine_wave_starts_at_zero() {
        let samples = sine(440, 100);
        assert!(samples[0].abs() < 100);
    }
}
