---
name: synth
description: Audio synthesis expertise - waveforms, frequencies, WAV format, DSP. Use when: audio, sound, waveform, frequency, WAV, sine wave, square wave, triangle wave, sawtooth, synthesizer, generate sound, Hz, samples, amplitude, envelope, ADSR.
---

# Audio Synthesis Expert

## Fundamentals

### Frequency & Pitch

```
A4 = 440 Hz (concert pitch)
Equal temperament: f(n) = 440 * 2^(n/12)
Where n = semitones from A4
```

### Chess Square Mapping

```
Column -> Note: a=C, b=D, c=E, d=F, e=G, f=A, g=B, h=C(+1)
Rank -> Octave: 1-8 -> octaves 2-5 (playable range)

e4 -> G4 -> 392 Hz
a1 -> C2 -> 65.41 Hz
h8 -> C6 -> 1046.50 Hz
```

## Waveforms in Rust

```rust
fn sine_sample(phase: f64) -> f64 {
    phase.sin()
}

fn triangle_sample(phase: f64) -> f64 {
    (2.0 / std::f64::consts::PI) * phase.sin().asin()
}

fn square_sample(phase: f64) -> f64 {
    if phase.sin() >= 0.0 { 1.0 } else { -1.0 }
}

fn sawtooth_sample(phase: f64) -> f64 {
    2.0 * (phase / std::f64::consts::TAU - (phase / std::f64::consts::TAU + 0.5).floor())
}
```

## WAV Format (44-byte header)

```
Offset  Size  Description
0       4     "RIFF"
4       4     File size - 8 (little-endian)
8       4     "WAVE"
12      4     "fmt "
16      4     16 (PCM format chunk size)
20      2     1 (PCM audio format)
22      2     1 (mono) or 2 (stereo)
24      4     Sample rate (44100)
28      4     Byte rate (sample_rate * channels * bits/8)
32      2     Block align (channels * bits/8)
34      2     Bits per sample (16)
36      4     "data"
40      4     Data size (little-endian)
44      ...   Sample data (16-bit signed, little-endian)
```

### Writing in Rust

```rust
use std::io::Write;

fn write_le16(writer: &mut impl Write, value: i16) -> std::io::Result<()> {
    writer.write_all(&value.to_le_bytes())
}

fn write_le32(writer: &mut impl Write, value: u32) -> std::io::Result<()> {
    writer.write_all(&value.to_le_bytes())
}
```

## ADSR Envelope

```
     /\
    /  \____
   /        \
  /          \
 A  D   S    R
```

```rust
fn envelope(position: f64, attack: f64, decay: f64, sustain: f64, release: f64) -> f64 {
    let total = attack + decay + sustain + release;
    let t = position * total;

    if t < attack {
        t / attack
    } else if t < attack + decay {
        1.0 - (1.0 - sustain) * (t - attack) / decay
    } else if t < attack + decay + sustain {
        sustain
    } else {
        sustain * (1.0 - (t - attack - decay - sustain) / release)
    }
}
```

## Common Pitfalls

- **Clipping**: clamp samples to `i16::MIN..=i16::MAX` before writing
- **Aliasing**: frequencies above Nyquist (sample_rate / 2) cause artifacts
- **DC offset**: waveform should be centered on zero
- **Phase continuity**: maintain phase across note boundaries to avoid clicks
