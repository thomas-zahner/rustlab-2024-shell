use std::time::Duration;

use crate::utils::ShellRunner;

const SHELL_TIMEOUT: Duration = Duration::from_secs(3);

#[test]
fn shell_runs_pwd_twice() {
    let output = ShellRunner::new()
        .with_stdin("pwd; pwd")
        .kill_after(SHELL_TIMEOUT)
        .run();

    let stdout_str = String::from_utf8(output.stdout).unwrap();
    let curr_dir_path = std::env::current_dir().unwrap();
    let curr_dir = curr_dir_path.to_str().unwrap();
    assert_eq!(stdout_str, format!("{curr_dir}\n{curr_dir}\n"));
}

#[test]
fn shell_understands_and_operator() {
    let output = ShellRunner::new()
        .with_stdin("echo hello && echo world")
        .kill_after(SHELL_TIMEOUT)
        .run();

    let stdout_str = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout_str, "hello\nworld\n");
}

#[test]
fn shell_understands_or_operator() {
    let output = ShellRunner::new()
        .with_stdin("echo hello || echo world")
        .kill_after(SHELL_TIMEOUT)
        .run();

    let stdout_str = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout_str, "hello\n");
}
