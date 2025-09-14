use std::path::Path;

use assert_cmd::Command;
use insta::assert_snapshot;

#[test]
fn cli_help_output() {
    let mut cmd = Command::cargo_bin("num-peek").unwrap();
    let result = cmd.arg("--help").assert().success();

    let output = String::from_utf8(result.get_output().stdout.clone()).unwrap();

    assert_snapshot!(output);
}

fn cli_run_with_file(file_path: &str, snapshot_name: &str) {
    let mut cmd = Command::cargo_bin("num-peek").unwrap();
    let result = cmd.arg(file_path).assert().success();
    let output = String::from_utf8(result.get_output().stdout.clone()).unwrap();

    assert_snapshot!(snapshot_name, output);
}

#[test]
fn cli_run_all_types() {
    let files = [
        "assets/bool.npy",
        "assets/uint8.npy",
        "assets/uint16.npy",
        "assets/uint32.npy",
        "assets/uint64.npy",
        "assets/int8.npy",
        "assets/int16.npy",
        "assets/int32.npy",
        "assets/int64.npy",
        "assets/float16.npy",
        "assets/float32.npy",
        "assets/float64.npy",
    ];
    for file in files {
        let snapshot_name = format!(
            "{}_snapshot",
            Path::new(file).file_stem().unwrap().to_string_lossy()
        );
        cli_run_with_file(file, &snapshot_name);
    }
}
