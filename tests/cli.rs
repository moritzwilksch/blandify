use std::fs;
use std::io::Write;
use std::process::{Command as StdCommand, Stdio};

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;

fn blandify_cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("blandify"))
}

#[test]
fn no_input_shows_help() {
    let mut cmd = blandify_cmd();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("Input file path or '-' for stdin"));
}

#[test]
fn stdin_to_stdout_with_dash() {
    let input = "Hello\u{00A0}\u{201C}x\u{201D}\n\tindent\n";
    let expected = "Hello \"x\"\n  indent\n";

    let mut cmd = blandify_cmd();
    cmd.arg("-")
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn input_file_to_stdout() {
    let mut input_file = NamedTempFile::new().expect("temp input");
    write!(input_file, "a\u{2014}b\tc").expect("write input");

    let mut cmd = blandify_cmd();
    cmd.arg(input_file.path())
        .assert()
        .success()
        .stdout("a-b  c");
}

#[test]
fn output_file_is_written() {
    let mut input_file = NamedTempFile::new().expect("temp input");
    let output_file = NamedTempFile::new().expect("temp output");
    write!(input_file, "x\u{00A0}\u{00A0}y").expect("write input");

    let mut cmd = blandify_cmd();
    cmd.arg(input_file.path())
        .arg("--output")
        .arg(output_file.path())
        .assert()
        .success()
        .stdout("");

    let written = fs::read_to_string(output_file.path()).expect("read output");
    assert_eq!(written, "x  y");
}

#[test]
fn output_dash_writes_stdout() {
    let mut cmd = blandify_cmd();
    cmd.arg("-")
        .arg("--output")
        .arg("-")
        .write_stdin("a\u{2014}b")
        .assert()
        .success()
        .stdout("a-b");
}

#[test]
fn in_place_overwrites_input_file() {
    let mut input_file = NamedTempFile::new().expect("temp input");
    write!(input_file, "x\u{00A0}\u{00A0}y").expect("write input");

    let mut cmd = blandify_cmd();
    cmd.arg("--in-place")
        .arg(input_file.path())
        .assert()
        .success();

    let written = fs::read_to_string(input_file.path()).expect("read in-place output");
    assert_eq!(written, "x  y");
}

#[test]
fn in_place_requires_input_file() {
    let mut cmd = blandify_cmd();
    cmd.arg("--in-place")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "--in-place requires an input file path",
        ));
}

#[test]
fn in_place_rejects_stdin() {
    let mut cmd = blandify_cmd();
    cmd.arg("--in-place")
        .arg("-")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "--in-place cannot be used with stdin ('-')",
        ));
}

#[test]
fn in_place_conflicts_with_output() {
    let mut input_file = NamedTempFile::new().expect("temp input");
    let output_file = NamedTempFile::new().expect("temp output");
    write!(input_file, "x\u{00A0}\u{00A0}y").expect("write input");

    let mut cmd = blandify_cmd();
    cmd.arg("--in-place")
        .arg(input_file.path())
        .arg("--output")
        .arg(output_file.path())
        .assert()
        .failure();
}

#[test]
fn no_category_flags_disable_expected_normalization() {
    struct Case {
        flag: &'static str,
        input: &'static str,
        default_output: &'static str,
        flagged_output: &'static str,
    }

    let cases = [
        Case {
            flag: "--no-quotes",
            input: "\u{201C}x\u{201D}",
            default_output: "\"x\"",
            flagged_output: "\u{201C}x\u{201D}",
        },
        Case {
            flag: "--no-dashes",
            input: "a\u{2014}b",
            default_output: "a-b",
            flagged_output: "a\u{2014}b",
        },
        Case {
            flag: "--no-whitespace",
            input: "a\u{00A0}b\tc",
            default_output: "a b  c",
            flagged_output: "a\u{00A0}b\tc",
        },
        Case {
            flag: "--no-zero-width",
            input: "a\u{200B}b",
            default_output: "ab",
            flagged_output: "a\u{200B}b",
        },
        Case {
            flag: "--no-arrows",
            input: "\u{2192}",
            default_output: "->",
            flagged_output: "\u{2192}",
        },
        Case {
            flag: "--no-fractions",
            input: "\u{00BD}",
            default_output: "1/2",
            flagged_output: "\u{00BD}",
        },
        Case {
            flag: "--no-math",
            input: "\u{00D7}",
            default_output: "x",
            flagged_output: "\u{00D7}",
        },
        Case {
            flag: "--no-symbols",
            input: "\u{2026}",
            default_output: "...",
            flagged_output: "\u{2026}",
        },
    ];

    for case in cases {
        let mut default_cmd = blandify_cmd();
        default_cmd
            .arg("-")
            .write_stdin(case.input)
            .assert()
            .success()
            .stdout(case.default_output);

        let mut flagged_cmd = blandify_cmd();
        flagged_cmd
            .arg(case.flag)
            .arg("-")
            .write_stdin(case.input)
            .assert()
            .success()
            .stdout(case.flagged_output);
    }
}

#[test]
fn umlauts_flag_enables_transliteration() {
    let mut cmd = blandify_cmd();
    cmd.arg("--umlauts")
        .arg("-")
        .write_stdin("ä ö ü ß")
        .assert()
        .success()
        .stdout("ae oe ue ss");
}

#[test]
fn broken_pipe_on_stdout_is_success() {
    let bin = assert_cmd::cargo::cargo_bin!("blandify");
    let mut child = StdCommand::new(bin)
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn blandify");

    drop(child.stdout.take());

    let input = "a".repeat(200_000);
    child
        .stdin
        .as_mut()
        .expect("stdin handle")
        .write_all(input.as_bytes())
        .expect("write stdin");
    drop(child.stdin.take());

    let output = child.wait_with_output().expect("wait for blandify");
    assert!(
        output.status.success(),
        "expected success, got: {:?}",
        output.status
    );
    assert!(
        output.stderr.is_empty(),
        "expected empty stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn missing_input_file_exits_non_zero() {
    let mut cmd = blandify_cmd();
    cmd.arg("does-not-exist-12345.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file").or(predicate::str::contains("os error")));
}
