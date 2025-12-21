// Testing Program output

use assert_cmd::Command;
use pretty_assertions::assert_eq;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    let output = cmd.output().expect("fail"); // 2
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)
    .expect("invalid UTF-8");
    assert_eq!(stdout, "Hello, world"); // 5

}


    /*
    
2 = Calling the 'Command::output' to execute the 'hello' command. Use the (Result::expect) to get output of the command or die with the message "fail".

5 = Compare the output from the program to an expected value. This will use the pretty_assertions version of the assert_eq macro.
   
    */
    


