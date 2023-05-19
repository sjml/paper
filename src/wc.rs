use std::fs;
use std::io::Read;

use anyhow::{ensure, Context, Result};

use crate::config::CONFIG;
use crate::subprocess;
use crate::util;

fn count_words_in(s: &str) -> usize {
    return s.split_whitespace().collect::<Vec<&str>>().len();
}

// returns tuples of (filename, full_count, stripped_count)
pub fn wc_data() -> Result<Vec<(String, usize, usize)>> {
    let mut counts = vec![];

    let lua_path = CONFIG
        .get()
        .resources_path
        .join("scripts")
        .join("stripped_md.lua");

    for entry in walkdir::WalkDir::new(&CONFIG.get().content_directory_name) {
        let entry = entry.context("Invalid directory entry in walkdir")?;
        if !entry.path().is_file() {
            continue;
        }
        if let Some(ext) = entry.path().extension() {
            if ext != "md" {
                continue;
            }
        }
        let trunc = entry
            .path()
            .strip_prefix(&CONFIG.get().content_directory_name)
            .context("Could not strip prefix from entry path")?;

        let full_pstr = entry.path().as_os_str().to_string_lossy().to_string();
        let trunc_pstr = trunc.as_os_str().to_string_lossy().to_string();
        if full_pstr == CONFIG.get().content_directory_name {
            continue;
        }

        let mut content_file =
            fs::File::open(entry.path()).with_context(|| format!("Could not open {:?}", &entry))?;
        let mut content_string = String::new();
        content_file
            .read_to_string(&mut content_string)
            .with_context(|| format!("Could not read {:?} to string", &content_file))?;

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
                &lua_path.to_string_lossy(),
                &entry.path().as_os_str().to_string_lossy(),
            ],
            None,
            false,
        )?;

        counts.push((
            trunc_pstr,
            count_words_in(&content_string),
            count_words_in(&stripped_content_string),
        ));
    }

    counts.sort();

    Ok(counts)
}

pub fn wc_json() -> Result<String> {
    let wcd = wc_data()?;

    let mut wc_map = serde_json::Map::new();
    let mut sizes = vec![];
    for (file, _, wcs) in wcd {
        wc_map.insert(
            file,
            serde_json::Value::Number(serde_json::Number::from(wcs)),
        );
        sizes.push(wcs);
    }
    let totals: usize = sizes.iter().sum();

    let mut full_map = serde_json::Map::new();
    full_map.insert(
        "total".to_string(),
        serde_json::Value::Number(serde_json::Number::from(totals)),
    );
    full_map.insert("breakdown".to_string(), serde_json::Value::Object(wc_map));

    let full_map_json = serde_json::to_string(&serde_json::Value::Object(full_map))
        .context("Could not convert full_map to JSON string")?;

    Ok(full_map_json)
}

pub fn wc_string(show_full: bool, delimit_final_row: bool) -> Result<String> {
    let wcd = wc_data()?;
    if wcd.is_empty() {
        return Ok(String::new());
    }

    let mut header = vec!["File".to_string(), "Word Count".to_string()];
    if show_full {
        header.push("Stripped".to_string());
    } else {
    }

    let mut table = vec![header];

    table.extend(wcd.iter().map(|datums| {
        if show_full {
            vec![datums.0.clone(), datums.1.to_string(), datums.2.to_string()]
        } else {
            vec![datums.0.clone(), datums.2.to_string()]
        }
    }));

    if wcd.len() > 1 {
        let sums = wcd
            .iter()
            .map(|datums| (datums.1, datums.2))
            .fold((0, 0), |acc, d| (acc.0 + d.0, acc.1 + d.1));
        let mut sum_strs = vec!["**TOTAL**".to_string(), sums.1.to_string()];
        if show_full {
            sum_strs.insert(1, sums.0.to_string());
        }
        table.push(sum_strs);
    }

    let num_cols = table.first().unwrap().len();
    for row in &table {
        ensure!(row.len() == num_cols, "Irregular table topography");
    }
    let widths = table
        .iter()
        .map(|row| {
            row.iter()
                .map(|s| s.chars().count())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let max_widths = widths.iter().fold(vec![0; num_cols], |acc, row| {
        acc.iter()
            .enumerate()
            .map(|(i, len)| std::cmp::max(row[i], *len))
            .collect::<Vec<usize>>()
    });

    let mut out_strings: Vec<String> = vec![];
    for (row_idx, row_data) in table.iter().enumerate() {
        if delimit_final_row {
            if row_idx == table.len() - 1 {
                let divs = max_widths
                    .iter()
                    .map(|w| "-".repeat(*w))
                    .collect::<Vec<String>>();
                out_strings.push(format!("| {} |", divs.join(" | ")));
            }
        }
        let cells: Vec<String> = row_data
            .iter()
            .enumerate()
            .map(|(i, s)| {
                if i > 0 {
                    format!(" {:>w$} ", s, w = max_widths[i])
                } else {
                    format!(" {:w$} ", s, w = max_widths[i])
                }
            })
            .collect();
        out_strings.push(format!("|{}|", cells.join("|")));
        if row_idx == 0 && wcd.len() > 1 {
            let divs = max_widths
                .iter()
                .map(|w| "-".repeat(*w))
                .collect::<Vec<String>>();
            out_strings.push(format!("| {} |", divs.join(" | ")));
        }
    }

    Ok(out_strings.join("\n"))
}

pub fn wc(show_full: bool) -> Result<()> {
    util::ensure_paper_dir()?;
    let out = wc_string(show_full, true)?;
    println!("{}\n", out);
    Ok(())
}
