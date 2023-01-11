// because the popular subprocess crate was causing IOErrors during debugging sessions,
//   and my needs are modest.

use std::fmt;
use std::io;
use std::io::Write;
use std::process;

#[derive(Debug)]
pub enum RunCommandError {
    IoErr(io::Error),
    RuntimeErr(process::Output),
}

impl std::error::Error for RunCommandError {}

impl fmt::Display for RunCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunCommandError::IoErr(ioe) => {
                write!(f, "{:?}", ioe)
            }
            RunCommandError::RuntimeErr(out) => {
                let stderr = match String::from_utf8(out.stderr.clone()) {
                    Ok(s) => s,
                    Err(_) => "<unprintable>".to_string(),
                };
                let stdout = match String::from_utf8(out.stdout.clone()) {
                    Ok(s) => s,
                    Err(_) => "<unprintable>".to_string(),
                };
                write!(
                    f,
                    "Command Runtime Error!\n\nstatus: {}\nstdout: {}\nstderr: {}",
                    out.status, stdout, stderr
                )
            }
        }
    }
}

pub fn run_command<T: AsRef<str> + std::convert::AsRef<std::ffi::OsStr> + std::fmt::Debug>(
    cmd: &str,
    args: &[T],
    stdin_str: Option<&str>,
    pass_stderr: bool,
) -> Result<String, RunCommandError> {
    let mut command = process::Command::new(cmd);
    command.args(args);

    let mut running: std::process::Child;

    let output: io::Result<process::Output>;

    if let Some(stdin_str) = stdin_str {
        let stdin_str_copy = stdin_str.to_owned();
        command.stdin(process::Stdio::piped());
        command.stdout(process::Stdio::piped());
        running = match command.spawn() {
            Ok(r) => r,
            Err(e) => {
                return Err(RunCommandError::IoErr(e));
            }
        };

        let stdin = running.stdin.as_mut().expect("Couldn't get stdin.");
        stdin
            .write_all(stdin_str_copy.as_bytes())
            .expect("Couldn't write to stdin.");

        output = running.wait_with_output();
    } else {
        output = command.output();
    }

    match output {
        Ok(o) => {
            if !o.status.success() {
                return Err(RunCommandError::RuntimeErr(o));
            }
            if pass_stderr {
                let stderr = String::from_utf8_lossy(&o.stderr).to_string();
                if !stderr.is_empty() {
                    println!("{}", stderr.trim_end());
                }
            }
            Ok(String::from_utf8_lossy(&o.stdout).to_string())
        }
        Err(e) => Err(RunCommandError::IoErr(e)),
    }
}
