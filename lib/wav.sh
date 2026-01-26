#!/usr/bin/env bash

WAV_SAMPLE_RATE=44100
WAV_BITS_PER_SAMPLE=16
WAV_NUM_CHANNELS=1

_write_le16() {
    local value="$1"
    local lo=$(( value & 0xFF ))
    local hi=$(( (value >> 8) & 0xFF ))
    printf "\\x$(printf '%02x' $lo)\\x$(printf '%02x' $hi)"
}

_write_le32() {
    local value="$1"
    local b0=$(( value & 0xFF ))
    local b1=$(( (value >> 8) & 0xFF ))
    local b2=$(( (value >> 16) & 0xFF ))
    local b3=$(( (value >> 24) & 0xFF ))
    printf "\\x$(printf '%02x' $b0)\\x$(printf '%02x' $b1)\\x$(printf '%02x' $b2)\\x$(printf '%02x' $b3)"
}

wav_header() {
    local num_samples="$1"

    local bytes_per_sample=$(( WAV_BITS_PER_SAMPLE / 8 ))
    local block_align=$(( WAV_NUM_CHANNELS * bytes_per_sample ))
    local byte_rate=$(( WAV_SAMPLE_RATE * block_align ))
    local data_size=$(( num_samples * block_align ))
    local chunk_size=$(( 36 + data_size ))

    # RIFF header
    printf "RIFF"
    _write_le32 "$chunk_size"
    printf "WAVE"

    # fmt subchunk
    printf "fmt "
    _write_le32 16                    # Subchunk1Size (16 for PCM)
    _write_le16 1                     # AudioFormat (1 = PCM)
    _write_le16 "$WAV_NUM_CHANNELS"   # NumChannels
    _write_le32 "$WAV_SAMPLE_RATE"    # SampleRate
    _write_le32 "$byte_rate"          # ByteRate
    _write_le16 "$block_align"        # BlockAlign
    _write_le16 "$WAV_BITS_PER_SAMPLE" # BitsPerSample

    # data subchunk
    printf "data"
    _write_le32 "$data_size"
}
