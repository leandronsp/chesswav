//! Waveform blending and filtering.
//!
//! # Blending Concept
//!
//! Blending mixes a waveform with sine to soften harsh timbres.
//! The `sine_mix` parameter controls the ratio (0.0 = original, 1.0 = pure sine).
//!
//! ```text
//! sine_mix = 0.0 (pure square)     sine_mix = 0.5 (half blend)     sine_mix = 1.0 (pure sine)
//!
//!  1 │ ┌────┐                       1 │  ╭─╮                         1 │    ╭──╮
//!    │ │    │                         │ ╭╯  ╲                          │  ╭╯    ╰╮
//!  0 │─┘    └────                   0 │╯      ╲                      0 │─╯        ╰─
//!    │                                │        ╲                       │
//! -1 │                             -1 │         ╰─╯                 -1 │
//! ```
//!
//! # Band-Limiting
//!
//! The `harmonics` option limits Fourier series terms to reduce aliasing.
//! Lower values = smoother sound, higher values = closer to original.
//!
//! ```text
//! harmonics = 1 (fundamental only)   harmonics = 7 (smoother)        harmonics = ∞ (raw)
//!
//!  1 │    ╭──╮                       1 │  ╭─╮                         1 │ ┌────┐
//!    │  ╭╯    ╰╮                       │ ╭╯  ╲╮                         │ │    │
//!  0 │─╯        ╰─                   0 │╯      ╲                      0 │─┘    └────
//!    │                                │        ╲╯                      │
//! -1 │                             -1 │          ╰╮                 -1 │
//!    (sounds like sine)              (rounded corners)               (harsh/buzzy)
//! ```
//!
//! # Combination
//!
//! Both options can be combined: band-limit first, then blend with sine.
//! This produces warm, musical timbres without digital harshness.

use crate::waveform::Waveform;

/// Options for blending and filtering waveforms.
#[derive(Clone, Copy)]
pub struct Blend {
    /// Ratio of sine wave to mix in (0.0 = none, 1.0 = pure sine)
    pub sine_mix: f64,
    /// Number of harmonics for band-limiting (None = unlimited/raw)
    pub harmonics: Option<u32>,
}

impl Blend {
    /// No blending - use raw waveform as-is.
    pub fn none() -> Self {
        Self {
            sine_mix: 0.0,
            harmonics: None,
        }
    }

    /// Blend with sine wave only.
    /// `ratio`: 0.0 = original, 0.5 = half-half, 1.0 = pure sine
    pub fn with_sine(ratio: f64) -> Self {
        Self {
            sine_mix: ratio,
            harmonics: None,
        }
    }

    /// Band-limit only (no sine mixing).
    /// `harmonics`: number of Fourier terms (higher = closer to raw)
    pub fn band_limited(harmonics: u32) -> Self {
        Self {
            sine_mix: 0.0,
            harmonics: Some(harmonics),
        }
    }

    /// Both band-limiting and sine mixing.
    pub fn with_sine_and_band_limit(sine_mix: f64, harmonics: u32) -> Self {
        Self {
            sine_mix,
            harmonics: Some(harmonics),
        }
    }

    /// Apply blending to a waveform sample at the given phase.
    ///
    /// # Pipeline
    /// ```text
    /// phase ──→ [Waveform] ──→ [Band-limit?] ──→ [Sine mix?] ──→ output
    ///              │                │                 │
    ///              │    if harmonics.is_some()        │
    ///              │    use sample_band_limited()     │
    ///              │                                  │
    ///              │              if sine_mix > 0     │
    ///              │         output = sine × mix + base × (1 - mix)
    /// ```
    pub fn apply<W: Waveform>(&self, wave: &W, phase: f64) -> f64 {
        // Step 1: Generate base sample (raw or band-limited)
        let base = match self.harmonics {
            Some(h) => wave.sample_band_limited(phase, h),
            None => wave.sample(phase),
        };

        // Step 2: Mix with sine if requested
        // Linear interpolation: result = sine × mix + base × (1 - mix)
        if self.sine_mix == 0.0 {
            base
        } else {
            let sine = phase.sin();
            sine * self.sine_mix + base * (1.0 - self.sine_mix)
        }
    }
}
