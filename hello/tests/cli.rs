use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}

fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
}
