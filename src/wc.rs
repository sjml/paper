use std::fs;
use std::io::{Read, Write};

use anyhow::Result;
use tempfile;
use walkdir;

use crate::config::CONFIG;
use crate::subprocess;
use crate::util;

static WC_LUA_SCRIPT: &'static [u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/resources/scripts/stripped_md.lua"
));

fn count_words_in(s: &String) -> usize {
    return s.split_whitespace().collect::<Vec<&str>>().len();
}

fn wc_data() -> Result<Vec<(String, usize, usize)>> {
    let mut counts = vec![];

    let mut strip_script = tempfile::NamedTempFile::new()?;
    strip_script.write(WC_LUA_SCRIPT)?;
    let lua_path = strip_script.into_temp_path();
    let lua_path_str = lua_path.as_os_str().clone();

    for entry in walkdir::WalkDir::new("./content") {
        let entry = entry?;
        if !entry.path().is_file() {
            continue;
        }
        if let Some(ext) = entry.path().extension() {
            if ext != "md" {
                continue;
            }
        }
        let trunc = entry.path().strip_prefix("./content")?;

        let full_pstr = entry.path().as_os_str().to_string_lossy().to_string();
        let trunc_pstr = trunc.as_os_str().to_string_lossy().to_string();
        if full_pstr == "./content" {
            continue;
        }

        let mut content_file = fs::File::open(entry.path())?;
        let mut content_string = String::new();
        content_file.read_to_string(&mut content_string)?;

        // pandoc \
        //  --from markdown+bracketed_spans+raw_tex-auto_identifiers \
        //  --to=markdown+bracketed_spans+raw_tex-auto_identifiers \
        //  --lua-filter ../../../resources/writers/stripped_md.lua \
        //  <PATH_TO_FILE>
        let stripped_content_string = subprocess::run_command(
            "pandoc",
            &[
                "--from",
                &CONFIG.get().pandoc_input_format,
                "--to",
                &CONFIG.get().pandoc_input_format,
                "--lua-filter",
                &lua_path_str.to_string_lossy(),
                &entry.path().as_os_str().to_string_lossy(),
            ],
        )?;

        counts.push((
            trunc_pstr,
            count_words_in(&content_string),
            count_words_in(&stripped_content_string),
        ));
    }

    lua_path.close()?;

    Ok(counts)
}

pub fn wc(show_full: bool) -> Result<()> {
    util::ensure_paper_dir()?;
    let wcd = wc_data().unwrap();
    for (path, count, stripped_count) in wcd.iter() {
        if show_full {
            println!("{}: {}, {}", path, stripped_count, count);
        }
        else {
            println!("{}: {}", path, stripped_count);
        }
    }
    Ok(())
}
