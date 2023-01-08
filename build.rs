use std::process;

use anyhow::Result;

fn run<T: AsRef<str> + std::convert::AsRef<std::ffi::OsStr> + std::fmt::Debug>(
    cmd: &str,
    args: &[T],
) -> Result<(process::ExitStatus, String)> {
    let mut command = process::Command::new(cmd);
    command.args(args);

    let output = command.output()?;
    let output_str = String::from_utf8(output.stdout)?;
    Ok((output.status, output_str))
}

fn main() -> Result<()> {
    let is_git = run("git", &["rev-parse"])?.0.success();
    if is_git {
        let mut hash = run("git", &["rev-parse", "HEAD"])?.1;
        let diffs = run("git", &["diff", "--stat"])?.1;
        if !diffs.trim().is_empty() {
            hash = format!("{}+dev", hash.trim());
        }
        println!("cargo:rustc-env=PAPER_GIT_REV={}", hash);
    } else {
        println!("cargo:rustc-env=PAPER_GIT_REV=");
    }

    let dep_tree = run("cargo", &["tree"])?;
    let dep_single_line = dep_tree.1.replace("\n", "||||||");
    println!("cargo:rustc-env=PAPER_RUST_DEPS={}", dep_single_line);

    Ok(())
}
