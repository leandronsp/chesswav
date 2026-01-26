use std::io::{self, Read, Write};
use chesswav::audio;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let play_audio = args.iter().any(|a| a == "--play" || a == "-p");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let samples = audio::generate(&input);
    let wav_data = audio::to_wav(&samples);

    if play_audio {
        play(&wav_data);
    } else {
        io::stdout().lock().write_all(&wav_data).ok();
    }
}

fn play(wav_data: &[u8]) {
    let tmpfile = std::env::temp_dir().join("chesswav.wav");
    std::fs::write(&tmpfile, wav_data).expect("Failed to write temp file");

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("afplay")
            .arg(&tmpfile)
            .status()
            .expect("Failed to play audio");
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("aplay")
            .args(["-f", "S16_LE", "-r", "44100", "-c", "1"])
            .arg(&tmpfile)
            .status()
            .expect("Failed to play audio");
    }

    std::fs::remove_file(&tmpfile).ok();
}
