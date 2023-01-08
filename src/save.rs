use std::fs;
use std::io::Write;

use anyhow::{bail, Result};
use chrono::prelude::*;
use dialoguer;
use plotters::prelude::*;
use serde_json;

use crate::metadata::PaperMeta;
use crate::subprocess;
use crate::util;
use crate::wc;

const METADATA_START_SENTINEL: &str = "<!-- begin paper metadata -->";
const METADATA_END_SENTINEL: &str = "<!-- end paper metadata -->";
const SVG_STYLE_FONT: &str = "-apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji'";

pub fn save() -> Result<()> {
    util::ensure_paper_dir()?;

    let message: String = dialoguer::Input::new()
        .with_prompt("Commit message?")
        .interact_text()?;

    util::stamp_local_dir()?;

    let meta = PaperMeta::new()?;

    let readme_path = std::env::current_dir()?.join("README.md");
    if !readme_path.exists() {
        let mut readme_file = fs::File::create(&readme_path)?;
        match meta.get_string(&["data", "class_mnemonic"]) {
            Some(mnemonic) => {
                writeln!(readme_file, "# {}: {}\n", mnemonic, util::get_assignment()?)?;
            }
            None => {
                writeln!(readme_file, "# {}\n", util::get_assignment()?)?;
            }
        }
        writeln!(readme_file, "{}", METADATA_START_SENTINEL)?;
        writeln!(readme_file, "{}", METADATA_END_SENTINEL)?;
    }

    let readme_text = fs::read_to_string(&readme_path)?;
    let readme_meta_start_idx = readme_text.find(METADATA_START_SENTINEL);
    let readme_meta_end_idx = readme_text.find(METADATA_END_SENTINEL);

    if readme_meta_start_idx.is_some() && readme_meta_end_idx.is_some() {
        let readme_meta_start_idx = readme_meta_start_idx.unwrap();
        let readme_meta_end_idx = readme_meta_end_idx.unwrap() + METADATA_END_SENTINEL.len();

        let readme_before = &readme_text[0..readme_meta_start_idx];
        let readme_after = &readme_text[readme_meta_end_idx..];

        let progress_img_str = get_progress_image_str(&meta)?;
        fs::write(
            std::env::current_dir()?
                .join(".paper_data")
                .join("progress.svg"),
            progress_img_str,
        )?;

        let wcs = wc::wc_string(false)?;

        let readme_out_text = format!(
            "{}{}\n{}\n\n![WordCountProgress](./.paper_data/progress.svg)\n{}{}",
            readme_before, METADATA_START_SENTINEL, wcs, METADATA_END_SENTINEL, readme_after
        );

        fs::write(readme_path, readme_out_text)?;
    }

    let message = format!("{}\n\nPAPER_DATA\n{}", message, wc::wc_json()?);

    subprocess::run_command("git", &["add", "."], None)?;
    subprocess::run_command("git", &["commit", "-m", &message], None)?;

    Ok(())
}

pub fn push() -> Result<()> {
    util::ensure_paper_dir()?;

    let remote = subprocess::run_command("git", &["remote", "-v"], None)?;
    if remote.is_empty() {
        let meta = PaperMeta::new()?;
        // default_repo = f"{meta['data']['class_mnemonic'].replace(' ', '')} {get_assignment()}"
        let mnemonic = meta
            .get_string(&["data", "class_mnemonic"])
            .unwrap_or_default();
        let default_name = format!("{} {}", mnemonic, util::get_assignment()?);
        let default_name = default_name.trim();

        println!(
            "(Note that GitHub will do some mild renaming, so it may not be this exact string.)"
        );
        let repo_name: String = dialoguer::Input::new()
            .with_prompt("What should be the repository name?")
            .default(default_name.into())
            .interact_text()?;
        let is_private = dialoguer::Confirm::new()
            .with_prompt("Private repository?")
            .default(true)
            .interact()?;

        let mut args = vec!["repo", "create", &repo_name, "--source=.", "--push"];
        if is_private {
            args.push("--private");
        }
        subprocess::run_command("gh", &args, None)?;
    } else {
        subprocess::run_command("git", &["push"], None)?;
    }

    Ok(())
}

pub fn web() -> Result<()> {
    util::ensure_paper_dir()?;

    let remote = subprocess::run_command("git", &["remote", "-v"], None)?;
    if remote.is_empty() {
        bail!("No remote repository set up.")
    }

    let origin_url = subprocess::run_command("git", &["remote", "get-url", "origin"], None)?;
    if !origin_url.contains("github.com") {
        // not entirely reliable as you could have a different remote repository containing a string
        //   referencing "github.com" but this is already error-check-y enough for my purposes.
        bail!("This repository is not on GitHub.");
    }

    subprocess::run_command("gh", &["repo", "view", "--web"], None)?;

    Ok(())
}

fn get_progress_image_str(meta: &PaperMeta) -> Result<String> {
    let mut img = String::new();

    let target_wc = meta.get_int(&["data", "target_word_count"]).unwrap_or(-1);
    let due_date = match meta.get_string(&["data", "date"]) {
        None => None,
        Some(ds) => {
            let dd = DateTime::parse_from_str(&ds, "%Y-%m-%d")?.with_timezone(&Utc);
            Some(dd)
        }
    };

    let mut commits = get_commit_data()?;
    commits.reverse();
    let mut wc_data: Vec<(DateTime<Utc>, usize)> = commits
        .iter()
        .map(|cdata| (Utc.timestamp_opt(cdata.1, 0).single().unwrap(), cdata.3))
        .collect();

    let current_total: usize = wc::wc_data()?
        .iter()
        .map(|d| d.2)
        .collect::<Vec<usize>>()
        .iter()
        .sum();
    let current_date = Utc::now();
    wc_data.push((current_date, current_total));

    let (timestamps, wcs): (Vec<DateTime<Utc>>, Vec<usize>) = wc_data.iter().cloned().unzip();

    let earliest = *timestamps.iter().min().unwrap();
    let mut latest = *timestamps.iter().max().unwrap();
    if let Some(dd) = due_date {
        if dd > latest {
            latest.clone_from(&dd);
        }
    }
    let date_range = latest.signed_duration_since::<Utc>(earliest);
    let date_buffer = date_range.num_days() as u64 / 20;
    let date_buffer = std::cmp::max(date_buffer, 2);
    let latest = latest
        .checked_add_days(chrono::Days::new(date_buffer))
        .unwrap();

    let min_wc = *wcs.iter().min().unwrap();
    let mut max_wc = *wcs.iter().max().unwrap();
    if target_wc >= 0 {
        max_wc = std::cmp::max(max_wc, target_wc.try_into()?);
    }
    let wc_max_buffer = std::cmp::max(max_wc / 20, 100);
    let max_wc = max_wc + wc_max_buffer;

    let root = SVGBackend::with_string(&mut img, (500, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        // plotters's notion of "centering" is a bit off, so non-breaking spaces to the rescue
        .caption("                 Progress", (SVG_STYLE_FONT, 25))
        .set_label_area_size(LabelAreaPosition::Left, 70)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(earliest..latest, min_wc..max_wc)?;
    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_x_mesh()
        .x_labels(10)
        .x_label_formatter(&|dt| dt.format("%b-%y").to_string())
        .max_light_lines(4)
        .y_desc("Word Count")
        .label_style((SVG_STYLE_FONT, 15))
        .draw()?;

    chart.draw_series(LineSeries::new(
        wc_data.iter().map(|(dt, wc)| (*dt, *wc)),
        plotters::style::full_palette::BLUE_600.stroke_width(2),
    ))?;

    if target_wc >= 0 {
        let wcu: usize = target_wc.try_into()?;
        let wordcount_series = vec![(earliest, wcu), (latest, wcu)];
        chart.draw_series(LineSeries::new(
            wordcount_series.iter().map(|(dt, wc)| (*dt, *wc)),
            plotters::style::full_palette::GREEN_700.stroke_width(2),
        ))?;
    }

    if let Some(due_date) = due_date {
        let duedate_series = vec![(due_date, min_wc), (due_date, max_wc)];
        chart.draw_series(LineSeries::new(
            duedate_series.iter().map(|(dt, wc)| (*dt, *wc)),
            plotters::style::full_palette::RED_A700.stroke_width(2),
        ))?;
    }

    root.present()?;
    drop(chart);
    drop(root);

    Ok(img)
}

fn get_commit_data() -> Result<Vec<(String, i64, String, usize)>> {
    let log = subprocess::run_command("git", &["log", "--format=%P|||%ct|||%B||-30-||"], None)?;
    let commits_raw: Vec<String> = log
        .split("||-30-||")
        .filter_map(|c| {
            if c.is_empty() {
                None
            } else {
                Some(c.trim().to_string())
            }
        })
        .collect();
    let mut commits = vec![];
    for c in commits_raw {
        let datums: Vec<&str> = c.split("|||").collect();
        if datums.len() != 3 {
            continue;
        }
        let git_hash = datums[0];
        let timestamp = datums[1];
        let message = datums[2];

        let wc_splits: Vec<&str> = message.split("\nPAPER_DATA\n").collect();
        if wc_splits.len() < 2 {
            continue;
        }
        let wc_data: serde_json::Value = match serde_json::from_str(wc_splits[1]) {
            Err(_) => continue,
            Ok(data) => data,
        };
        let wc = match wc_data {
            serde_json::Value::Object(wco) => match wco.get("total") {
                Some(wcv) => {
                    if let Some(wcvi) = wcv.as_u64() {
                        wcvi as usize
                    } else {
                        continue;
                    }
                }
                None => continue,
            },
            _ => continue,
        };

        commits.push((
            git_hash.to_owned(),
            timestamp.parse::<i64>()?,
            message.to_owned(),
            wc,
        ));
    }

    Ok(commits)
}
