// because the popular subprocess crate was causing IOErrors during debugging sessions,
//   and my needs are modest.

use std::{io::Write, process};

use anyhow::{bail, Context, Result};

pub fn run_command<T: AsRef<str> + std::convert::AsRef<std::ffi::OsStr> + std::fmt::Debug>(
    cmd: &str,
    args: &[T],
    stdin_str: Option<&str>,
) -> Result<String> {
    let mut command = process::Command::new(cmd);
    command.args(args);

    let mut running: std::process::Child;

    if let Some(stdin_str) = stdin_str {
        let stdin_str_copy = stdin_str.to_owned();
        command.stdin(process::Stdio::piped());
        command.stdout(process::Stdio::piped());
        running = command.spawn()?;

        let stdin = running.stdin.as_mut().expect("Couldn't get stdin.");
        stdin
            .write_all(stdin_str_copy.as_bytes())
            .expect("Couldn't write to stdin.");

        let output = running.wait_with_output().expect("Couldn't read stdout");
        return Ok(String::from_utf8_lossy(&output.stdout).to_string());
    } else {
        let output = command.output().with_context(|| {
            return format!("Could not run command: {} with args <{:?}>", cmd, args);
        })?;

        let output_str = String::from_utf8(output.stdout)?;
        if output.status.success() {
            Ok(output_str)
        } else {
            let output_stderr = String::from_utf8(output.stderr)?;
            bail!(
                "Failure of command: {} with args <{:?}>`:\n\nstdout: {}\n\nstderr: {}\n",
                cmd,
                args,
                output_str,
                output_stderr
            );
        }
    }
}
