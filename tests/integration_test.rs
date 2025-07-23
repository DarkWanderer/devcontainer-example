use std::process::Command;

#[test]
fn test_main_output() {
    let output = Command::new("cargo")
        .args(&["run"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("Hello, world!"));
    assert!(output.status.success());
}