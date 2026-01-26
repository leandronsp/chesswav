use std::process::{Command, Stdio};
use std::io::Write;
use std::sync::Once;

static BUILD: Once = Once::new();

fn ensure_built() {
    BUILD.call_once(|| {
        Command::new("cargo")
            .args(["build", "--quiet"])
            .status()
            .expect("Failed to build");
    });
}

fn run_chesswav(input: &str) -> Vec<u8> {
    ensure_built();

    let mut child = Command::new("./target/debug/chesswav")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to spawn");

    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(input.as_bytes()).unwrap();
    } // stdin dropped here, sends EOF

    let output = child.wait_with_output().expect("Failed to read output");
    output.stdout
}

#[test]
fn basic_pipeline() {
    let output = run_chesswav("e4 e5 Nf3 Nc6");
    assert!(output.len() > 44, "Should have header + sample data");
}

#[test]
fn wav_valid_header() {
    let output = run_chesswav("e4 e5");
    assert_eq!(&output[0..4], b"RIFF");
    assert_eq!(&output[8..12], b"WAVE");
}

#[test]
fn four_moves_size() {
    let output = run_chesswav("e4 e5 Nf3 Nc6");
    // 4 moves * 300ms + 4 silences * 50ms = 1400ms
    // At 44100 Hz, 16-bit mono = ~123,480 bytes + 44 header
    assert!(output.len() > 100000);
}

#[test]
fn empty_input() {
    let output = run_chesswav("");
    assert!(output.len() >= 44, "Should have at least valid header");
}

#[test]
fn single_move() {
    let output = run_chesswav("e4");
    // 1 move = 300ms at 44100 Hz * 2 bytes = 26,460 bytes + header + silence
    assert!(output.len() > 20000);
}

#[test]
fn capture_move() {
    let output = run_chesswav("Bxc6");
    assert!(output.len() > 20000);
}
