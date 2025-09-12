use assert_cmd::Command;
use insta::assert_snapshot;

#[test]
fn cli_help_output() {
    let mut cmd = Command::cargo_bin("num-peek").unwrap();
    let assert = cmd.arg("--help").assert().success();

    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();

    assert_snapshot!(output);
}
