use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    let res = cmd.output();
    assert!(res.is_ok());
}
