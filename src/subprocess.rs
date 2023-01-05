// because the popular subprocess crate was causing IOErrors during debugging sessions,
//   and my needs are modest.

use std::process;

use anyhow::{bail, Context, Result};

pub fn run_command<T: AsRef<str> + std::convert::AsRef<std::ffi::OsStr> + std::fmt::Debug>(
    cmd: &str,
    args: &[T],
) -> Result<String> {
    let mut command = process::Command::new(cmd);
    command.args(args);

    let output = command.output().with_context(|| {
        return format!("Could not run command: {} with args <{:?}>", cmd, args);
    })?;

    if output.status.success() {
        let output_str = String::from_utf8(output.stdout)?;
        Ok(output_str)
    } else {
        let output_str = String::from_utf8(output.stderr)?;
        bail!(
            "Failure of command: {} with args <{:?}>`:\n\n{}",
            cmd,
            args,
            output_str
        );
    }
}
