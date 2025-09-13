use assert_cmd::Command;
use insta::assert_snapshot;

#[test]
fn cli_help_output() {
    let mut cmd = Command::cargo_bin("num-peek").unwrap();
    let assert = cmd.arg("--help").assert().success();

    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();

    assert_snapshot!(output);
}

fn cli_run_with_file(file_path: &str, snapshot_name: &str) {
    let mut cmd = Command::cargo_bin("num-peek").unwrap();
    let assert = cmd.arg(file_path).assert().success();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_snapshot!(snapshot_name, output);
}

#[test]
fn cli_run_all_types() {
    let files = ["assets/int64.npy", "assets/float64.npy"];
    for file in files {
        let snapshot_name = format!(
            "{}_snapshot",
            std::path::Path::new(file)
                .file_stem()
                .unwrap()
                .to_string_lossy()
        );
        cli_run_with_file(file, &snapshot_name);
    }
}
