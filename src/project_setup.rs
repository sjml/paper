use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::{bail, Context, Result};
use walkdir::WalkDir;
use yaml_rust::{yaml, Yaml, YamlEmitter};

use crate::config::CONFIG;
use crate::subprocess;
use crate::util;

pub fn init_project() -> Result<()> {
    let proj_path_buf = std::env::current_dir().context("Current path is invalid.")?;
    if proj_path_buf
        .read_dir()
        .context("Current path is invalid.")?
        .next()
        .is_some()
    {
        bail!("Directory needs to be empty to initialize project.");
    }

    // have already ensured that directory is empty
    let template_path = CONFIG.get().resources_path.join("project_template");
    let entries = WalkDir::new(&template_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect::<Vec<walkdir::DirEntry>>();

    for e in entries {
        let src = e.path();
        if template_path == src {
            continue;
        }
        let dst = proj_path_buf.join(
            src.strip_prefix(template_path.clone())
                .context("Could not strip prefix")?,
        );
        if e.file_type().is_dir() {
            fs::create_dir(&dst)
                .with_context(|| format!("Could not create dst directory {:?}", &dst))?;
        } else {
            fs::copy(src, dst).with_context(|| format!("Could not copy file {:?}", &src))?;
        }
    }

    let mut meta_chain: Vec<Yaml> = Vec::new();
    let mut current_path_option: Option<&Path> = Some(proj_path_buf.as_path());
    while let Some(current_path) = current_path_option {
        match util::find_meta(Some(current_path)) {
            Ok(meta_path) => {
                if CONFIG.get().verbose {
                    println!("Found meta file at {:?}", meta_path);
                }
                match util::load_yml_file(&meta_path) {
                    Ok(meta_yml) => match meta_yml {
                        Yaml::Hash(_) => meta_chain.push(meta_yml),
                        _ => bail!("Non-hash YAML document found at {:?}", meta_path),
                    },
                    // don't give up on one bad YAML file; just print the error and skip it
                    Err(e) => eprintln!("ERROR: {}", e),
                }
            }
            Err(_) => {}
        }
        current_path_option = current_path.parent();
    }
    meta_chain.reverse();

    let mut meta = yaml::Hash::new();
    for m in meta_chain {
        // already checked that everything is a hash, so this unwrap is safe
        util::merge_yaml_hash(&mut meta, &m.into_hash().unwrap());
    }

    let mut meta_str = String::new();
    let mut yaml_emitter = YamlEmitter::new(&mut meta_str);
    yaml_emitter
        .dump(&Yaml::Hash(meta))
        .context("Could not dump composed meta YAML.")?;

    let mut meta_output_file = fs::File::create("paper_meta.yml")
        .context("Could not create file for composed meta YAML.")?;
    write!(meta_output_file, "{}", meta_str)
        .context("Could not write file for composed meta YAML.")?;

    fs::create_dir("research").context("Could not write research directory.")?;

    subprocess::run_command("git", &["init"], None, false)?;
    subprocess::run_command("git", &["add", "."], None, false)?;
    subprocess::run_command(
        "git",
        &[
            "commit",
            "-m",
            &format!(
                "Initial project creation\n---\n{}",
                util::get_paper_version_stamp()
            ),
        ],
        None,
        false,
    )?;

    Ok(())
}

pub fn new_project(project_name: &str) -> Result<()> {
    let project_path = Path::new(project_name);
    if project_path.exists() {
        bail!("Project path already exists: {}", project_name);
    }

    println!("Starting new project called '{}'...", project_name);
    fs::create_dir(project_path)
        .with_context(|| format!("Could not create directory {:?}", project_path))?;
    std::env::set_current_dir(project_path)
        .with_context(|| format!("Could not move to directory {:?}", project_path))?;

    init_project()
}

pub fn dev() -> Result<()> {
    util::ensure_paper_dir()?;

    if cfg!(debug_assertions) {
        let src_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join("project_template")
            .join(".paper_resources");
        let dst_path_buf = std::env::current_dir()
            .context("Could not get current directory")?
            .join(".paper_resources");
        let dst_path = dst_path_buf.as_path();

        if dst_path.is_symlink() {
            let linked = dst_path
                .read_link()
                .with_context(|| format!("Could not read link {:?}", dst_path))?
                .canonicalize()
                .context("Could not canonicalize dst path")?;
            let src_canon = src_path
                .canonicalize()
                .context("Could not canonicalize src path")?;
            if linked == src_canon {
                bail!("Looks like this project is already set up for dev!")
            }
        }

        if cfg!(windows) {
            bail!("Not supported on Windows, sorry. ¯\\_(ツ)_/¯");
        } else {
            println!("This symlinks the package resource directory to this local one, deleting the local version.");
            println!("It’s meant for development on paper itself.");
            println!();
            let do_it = dialoguer::Confirm::new()
                .with_prompt("Is that what you’re up to?")
                .default(false)
                .interact()?;
            if do_it {
                fs::remove_dir_all(dst_path).context("Could not remove dst path")?;
                std::os::unix::fs::symlink(src_path, dst_path)
                    .context("Could not create symlink")?;
            } else {
                println!("No worries.")
            }
        }
    } else {
        bail!("Not supported in release mode, sorry. ¯\\_(ツ)_/¯");
    }

    Ok(())
}
