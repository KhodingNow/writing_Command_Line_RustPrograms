use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("writing__command_lineRust").unwrap();
    cmd.assert().success();
    
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();

    }
