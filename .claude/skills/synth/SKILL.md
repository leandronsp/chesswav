---
name: synth
description: Audio synthesis expertise - waveforms, frequencies, WAV format, DSP. Use when: audio, sound, waveform, frequency, WAV, sine wave, square wave, triangle wave, sawtooth, synthesizer, generate sound, play sound, hear it, make noise, audio output, sound generation, Hz, samples, amplitude, envelope, ADSR.
---

# Audio Synthesis Expert

## Fundamentals

### Frequency & Pitch

```
A4 = 440 Hz (concert pitch)
Equal temperament: f(n) = 440 × 2^(n/12)

Where n = semitones from A4
```

### Note Frequencies (Hz)

```
Octave:    2       3       4       5       6
C         65.41   130.81  261.63  523.25  1046.50
D         73.42   146.83  293.66  587.33  1174.66
E         82.41   164.81  329.63  659.26  1318.51
F         87.31   174.61  349.23  698.46  1396.91
G         98.00   196.00  392.00  783.99  1567.98
A        110.00   220.00  440.00  880.00  1760.00
B        123.47   246.94  493.88  987.77  1975.53
```

### Chess Square Mapping

```
Column → Note: a=C, b=D, c=E, d=F, e=G, f=A, g=B, h=C(+1)
Rank → Octave: 1-8 → octaves 2-5 (playable range)

e4 → G4 → 392 Hz
a1 → C2 → 65.41 Hz
h8 → C6 → 1046.50 Hz
```

## Waveforms

### Sine Wave
```bash
# Purest tone, single frequency
# y(t) = A × sin(2π × f × t)
sample=$((amplitude * sin(2 * PI * freq * t / sample_rate)))
```

### Triangle Wave
```bash
# Odd harmonics, 1/n² amplitude
# Softer than square
# y(t) = (2A/π) × arcsin(sin(2π × f × t))
```

### Square Wave
```bash
# All odd harmonics, 1/n amplitude
# Bright, buzzy
# y(t) = A × sign(sin(2π × f × t))
```

### Sawtooth Wave
```bash
# All harmonics, 1/n amplitude
# Bright, rich
# y(t) = 2A × (t×f - floor(t×f + 0.5))
```

## WAV Format

### Header Structure (44 bytes)

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
28      4     Byte rate (sample_rate × channels × bits/8)
32      2     Block align (channels × bits/8)
34      2     Bits per sample (16)
36      4     "data"
40      4     Data size (little-endian)
44      ...   Sample data
```

### Writing Little-Endian

```bash
# 16-bit value
write_le16() {
    local val=$1
    printf "\\x$(printf '%02x' $((val & 0xFF)))"
    printf "\\x$(printf '%02x' $(((val >> 8) & 0xFF)))"
}

# 32-bit value
write_le32() {
    local val=$1
    printf "\\x$(printf '%02x' $((val & 0xFF)))"
    printf "\\x$(printf '%02x' $(((val >> 8) & 0xFF)))"
    printf "\\x$(printf '%02x' $(((val >> 16) & 0xFF)))"
    printf "\\x$(printf '%02x' $(((val >> 24) & 0xFF)))"
}
```

### Sample Format

```
16-bit signed PCM: -32768 to 32767
Stored as little-endian pairs
```

## ADSR Envelope

```
     /\
    /  \____
   /        \
  /          \
 A  D   S    R

A (Attack):  0 → max amplitude
D (Decay):   max → sustain level
S (Sustain): held at level
R (Release): sustain → 0
```

### Simple Envelope

```bash
apply_envelope() {
    local sample=$1
    local position=$2      # 0.0 to 1.0
    local attack=0.1       # 10% of duration
    local release=0.2      # 20% of duration

    if (( $(echo "$position < $attack" | bc -l) )); then
        # Attack phase
        envelope=$(echo "$position / $attack" | bc -l)
    elif (( $(echo "$position > 1 - $release" | bc -l) )); then
        # Release phase
        envelope=$(echo "(1 - $position) / $release" | bc -l)
    else
        # Sustain phase
        envelope=1
    fi

    echo "$((sample * envelope))"
}
```

## Common Issues

### Clipping
```bash
# BAD: can exceed 16-bit range
sample=$((amplitude * wave_value))

# GOOD: clamp to valid range
sample=$((amplitude * wave_value))
((sample > 32767)) && sample=32767
((sample < -32768)) && sample=-32768
```

### Aliasing
```bash
# Frequencies above Nyquist (sample_rate/2) cause aliasing
# 44100 Hz → max frequency = 22050 Hz
# Keep fundamentals below 20000 Hz for safety
```

### DC Offset
```bash
# Waveform should be centered on zero
# Sum of all samples should approach zero
# Check with: awk '{sum+=$1} END {print sum/NR}'
```

## Performance Tips

```bash
# Pre-calculate constants
TWO_PI=$(echo "8 * a(1)" | bc -l)  # 2π

# Use lookup tables for trig
# Pre-generate one cycle, index into it

# Avoid bc in tight loops
# Use integer arithmetic where possible
```
