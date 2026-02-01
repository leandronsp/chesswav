//! Waveform types and trait definitions.
//!
//! # Waveform Shapes
//!
//! ```text
//! SINE - smooth, pure tone
//!  1 │    ╭──╮
//!    │  ╭╯    ╰╮
//!  0 │─╯        ╰─
//!    │            ╰╮
//! -1 │              ╰──╯
//!    └─────────────────→ phase
//!         0    π    2π
//!
//! SQUARE - abrupt transitions, hollow sound
//!  1 │ ┌────┐      ┌────
//!    │ │    │      │
//!  0 │─┘    │      │
//!    │      │      │
//! -1 │      └──────┘
//!    └─────────────────→ phase
//!
//! TRIANGLE - linear ramps, mellow tone
//!  1 │    ╱╲
//!    │   ╱  ╲
//!  0 │──╱    ╲──
//!    │ ╱      ╲
//! -1 │╱        ╲╱
//!    └─────────────────→ phase
//! ```
//!
//! # Band-Limited Synthesis
//!
//! Raw waveforms with sharp transitions cause aliasing (frequencies above
//! Nyquist fold back as artifacts). Band-limited versions use Fourier series
//! with limited harmonics to produce cleaner sound.
//!
//! ```text
//! Square Fourier series:
//!   f(t) = (4/π) × [sin(ωt) + sin(3ωt)/3 + sin(5ωt)/5 + ...]
//!          └─────────────────────────────────────────────────┘
//!                    only odd harmonics (1, 3, 5, 7...)
//!
//! Triangle Fourier series:
//!   f(t) = (8/π²) × [sin(ωt) - sin(3ωt)/9 + sin(5ωt)/25 - ...]
//!          └─────────────────────────────────────────────────────┘
//!             odd harmonics, alternating sign, amplitude ∝ 1/n²
//! ```

use std::f64::consts::PI;

/// A waveform that can generate samples at a given phase.
pub trait Waveform {
    /// Generate a sample value (-1.0 to 1.0) at the given phase (radians).
    fn sample(&self, phase: f64) -> f64;

    /// Generate a band-limited sample using additive synthesis.
    /// `harmonics` controls how many overtones to include.
    fn sample_band_limited(&self, phase: f64, harmonics: u32) -> f64;
}

/// Pure sine wave - the fundamental building block.
///
/// Formula: `sin(phase)`
#[derive(Clone, Copy)]
pub struct Sine;

/// Square wave - rich in odd harmonics, hollow/woody timbre.
///
/// Raw: `sign(sin(phase))` → +1 or -1
/// Band-limited: sum of odd harmonics with amplitude 1/n
#[derive(Clone, Copy)]
pub struct Square;

/// Triangle wave - odd harmonics only, but amplitude falls as 1/n².
/// Sounds mellower than square due to faster harmonic rolloff.
///
/// Raw: linear interpolation between peaks
/// Band-limited: sum of odd harmonics with amplitude 1/n², alternating sign
#[derive(Clone, Copy)]
pub struct Triangle;

impl Waveform for Sine {
    fn sample(&self, phase: f64) -> f64 {
        phase.sin()
    }

    fn sample_band_limited(&self, phase: f64, _harmonics: u32) -> f64 {
        // Sine is already band-limited (single frequency)
        self.sample(phase)
    }
}

impl Waveform for Square {
    /// Raw square: +1 when sin(phase) >= 0, else -1
    fn sample(&self, phase: f64) -> f64 {
        if phase.sin() >= 0.0 { 1.0 } else { -1.0 }
    }

    /// Band-limited square using Fourier series:
    /// f(t) = (4/π) × Σ sin(n×phase)/n  for n = 1, 3, 5, ...
    fn sample_band_limited(&self, phase: f64, harmonics: u32) -> f64 {
        let mut val = 0.0;
        // Only odd harmonics: 1, 3, 5, 7, ...
        for n in (1..=harmonics).step_by(2) {
            // Each harmonic: sin(n × phase) / n
            val += (phase * n as f64).sin() / n as f64;
        }
        // Scale factor: 4/π normalizes amplitude to [-1, 1]
        val * 4.0 / PI
    }
}

impl Waveform for Triangle {
    /// Raw triangle: linear ramp between -1 and +1
    /// Uses phase normalized to [0, 1) then maps to triangle shape
    fn sample(&self, phase: f64) -> f64 {
        // Normalize phase to [0, 1)
        let normalized = (phase / (2.0 * PI)).fract();
        let adjusted = if normalized < 0.0 { normalized + 1.0 } else { normalized };
        // Map [0, 1) to triangle: peak at 0.5
        // |x - 0.5| gives V shape, scale to [-1, 1]
        4.0 * (adjusted - 0.5).abs() - 1.0
    }

    /// Band-limited triangle using Fourier series:
    /// f(t) = (8/π²) × Σ (-1)^k × sin(n×phase)/n²  for n = 1, 3, 5, ...
    /// where k is the harmonic index (0, 1, 2, ...)
    fn sample_band_limited(&self, phase: f64, harmonics: u32) -> f64 {
        let mut val = 0.0;
        // Only odd harmonics with alternating sign
        for (i, n) in (1..=harmonics).step_by(2).enumerate() {
            // Alternating sign: +, -, +, -, ...
            let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
            // Amplitude falls as 1/n² (faster than square's 1/n)
            val += sign * (phase * n as f64).sin() / (n * n) as f64;
        }
        // Scale factor: 8/π² normalizes amplitude to [-1, 1]
        val * 8.0 / (PI * PI)
    }
}
