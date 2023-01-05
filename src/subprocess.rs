// because the popular subprocess crate was causing IOErrors during debugging sessions,
//   and my needs are modest.

use std::process;

use anyhow::{Context, Result};

pub fn run_command(cmd: &str, args: &[&str]) -> Result<String> {
    let mut command = process::Command::new(cmd);
    command.args(args);

    let output = command.output().with_context(|| {
        return format!("Could not run command: {} with args <{:?}>", cmd, args);
    })?;

    let output_str = String::from_utf8(output.stdout)?;
    Ok(output_str)
}
