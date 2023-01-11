use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use filetime;
use sxd_document;
use sxd_document::QName;
use sxd_xpath;
use sxd_xpath::Value::Nodeset;
use tempfile::{self, NamedTempFile};
use walkdir;
use zip::{self, ZipArchive};

use crate::build;
use crate::config::CONFIG;
use crate::formats::Builder;
use crate::metadata::PaperMeta;
use crate::subprocess;
use crate::util;

const DOCX_SCHEMA: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
const PROP_SCHEMA: &str = "http://schemas.openxmlformats.org/package/2006/metadata/core-properties";
const DCMD_SCHEMA: &str = "http://purl.org/dc/elements/1.1/";

pub struct DocxBuilder {
    tmp_prefix_files: Vec<NamedTempFile>,
}

impl Default for DocxBuilder {
    fn default() -> Self {
        DocxBuilder {
            tmp_prefix_files: vec![],
        }
    }
}

impl Builder for DocxBuilder {
    fn get_output_file_suffix(&self) -> String {
        "docx".to_string()
    }

    fn prepare(&mut self, args: &mut Vec<String>, meta: &PaperMeta) -> Result<()> {
        let cmds = [
            "--to=docx".to_string(),
            "--reference-doc".to_string(),
            "./.paper_resources/ChicagoStyle_Template.docx".to_string(),
        ];
        args.extend_from_slice(&cmds);

        if !meta.get_bool(&["no_title_page"]).unwrap_or_else(|| false) {
            let outpath = Path::new(&CONFIG.get().output_directory_name);
            let mut title_page_file = tempfile::Builder::new()
                .prefix("title-page")
                .suffix(".md")
                .tempfile_in(outpath)
                .context("Could not create temporary title page file.")?;

            if CONFIG.get().verbose {
                println!("Generating title page into {:?}...", title_page_file.path());
            }

            let mut title_string_coll: Vec<String> = vec![];

            title_string_coll.push("::: title-page\n\n".to_string());

            let title = meta.get_string(&["data", "title"]);
            let subtitle = meta.get_string(&["data", "subtitle"]);

            if title.is_some() || subtitle.is_some() {
                title_string_coll.push("::: {custom-style=\"Title\"}\n".to_string());
                match title {
                    Some(title_str) => {
                        title_string_coll.push(title_str);
                        match subtitle {
                            Some(subtitle_str) => {
                                title_string_coll.push(":\\\n".to_string());
                                title_string_coll.push(subtitle_str);
                                title_string_coll.push("\n".to_string());
                            }
                            None => title_string_coll.push("\n".to_string()),
                        }
                    }
                    None => {
                        if let Some(subtitle_str) = subtitle {
                            title_string_coll.push(subtitle_str);
                            title_string_coll.push("\n".to_string());
                        }
                    }
                }
                title_string_coll.push(":::\n".to_string());
            }
            title_string_coll.push("::: {custom-style=\"Author\"}\nby\n:::\n".to_string());
            title_string_coll.push("::: {custom-style=\"Author\"}\n".to_string());
            if let Some(author_str) = meta.get_string(&["data", "author"]) {
                title_string_coll.push(author_str);
                title_string_coll.push("\n".to_string());
            }
            title_string_coll.push(":::\n".to_string());

            title_string_coll.push("::: {custom-style=\"Author\"}\n".to_string());

            meta.get_string(&["data", "professor"])
                .map(|prof_str| title_string_coll.push(format!("{}\\\n", prof_str)));

            // I wonder if there's some more elegant way of handling this, but it's a pretty awkward
            //    bit of logic no matter what
            match meta.get_string(&["data", "class_mnemonic"]) {
                Some(mnemonic_str) => {
                    title_string_coll.push(mnemonic_str);
                    if let Some(classname_str) = meta.get_string(&["data", "class_name"]) {
                        title_string_coll.push(format!(" --- {}", classname_str));
                    }
                }
                None => {
                    if let Some(classname_str) = meta.get_string(&["data", "class_name"]) {
                        title_string_coll.push(classname_str);
                    }
                }
            }

            title_string_coll.push("\\\n".to_string());
            title_string_coll.push(util::get_date_string(&meta)?);

            title_string_coll.push("\n:::\n".to_string());

            title_string_coll.push("\n:::\n".to_string());

            write!(title_page_file, "{}", title_string_coll.join(""))
                .context("Could not write to temporary title page file.")?;

            self.tmp_prefix_files.push(title_page_file);
        }

        Ok(())
    }

    fn get_file_list(&self) -> Vec<String> {
        let mut file_list = vec![];

        file_list.extend(
            self.tmp_prefix_files
                .iter()
                .map(|ntf| ntf.path().to_string_lossy().to_string()),
        );

        file_list.extend(build::get_content_file_list());

        file_list
    }

    fn finish_file(&self, output_file_path: &Path, meta: &PaperMeta) -> Result<Vec<String>> {
        if CONFIG.get().verbose {
            println!("Packinging docx...");
        }

        let mut archive: ZipArchive<fs::File>;
        {
            let zipped_file = fs::File::open(output_file_path)
                .with_context(|| format!("Could not open file: {:?}", output_file_path))?;
            archive = zip::ZipArchive::new(zipped_file)
                .with_context(|| format!("Could not open zip archive: {:?}", output_file_path))?;
        }

        if CONFIG.get().verbose {
            println!("Unzipping docx contents into temporary directory...");
        }
        let output_dir =
            tempfile::tempdir_in("").context("Could not create temporary directory")?;
        let output_path = output_dir.path();

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .with_context(|| format!("Could not get file {} from zip archive", i))?;
            let filepath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };
            let creation_path = output_path.join(filepath);

            if (*file.name()).ends_with('/') {
                fs::create_dir_all(&creation_path).with_context(|| {
                    format!(
                        "Could not create directory during unzipping {:?}",
                        &creation_path
                    )
                })?;
            } else {
                if let Some(p) = creation_path.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p).with_context(|| {
                            format!("Could not create directory during unzipping {:?}", &p)
                        })?;
                    }
                }
                let mut outfile = fs::File::create(&creation_path).with_context(|| {
                    format!(
                        "Could not create file during unzipping {:?}",
                        &creation_path
                    )
                })?;
                std::io::copy(&mut file, &mut outfile).with_context(|| {
                    format!("Could not copy file during unzipping {:?}", &file.name())
                })?;
            }

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&creation_path, fs::Permissions::from_mode(mode))
                        .with_context(|| {
                            format!(
                                "Could not set permissions on path during unzipping {:?}",
                                &creation_path
                            )
                        })?;
                }
            }
        }

        let mut context = sxd_xpath::Context::new();
        context.set_namespace("w", DOCX_SCHEMA);
        context.set_namespace("cp", PROP_SCHEMA);
        context.set_namespace("dc", DCMD_SCHEMA);
        let factory = sxd_xpath::Factory::new();

        // the pandoc writer doesn't check the "total row" box for tables unless
        //   they explicitly have a footer, and we use that to do the spacing since
        //   Word doesn't have a good way to add spacing after the *whole* table
        if CONFIG.get().verbose {
            println!("Fixing docx table styles...");
        }
        let doc_doc = self
            .get_file_root(&output_path.to_path_buf(), "word/document.xml")
            .with_context(|| format!("Could not get file root for {:?}", &output_path))?;
        let doc_doc = doc_doc.as_document();
        let root = doc_doc.root();

        let xpath = self.get_xpath(&factory, "//w:tblLook")?;
        let val = xpath
            .evaluate(&context, root)
            .context("Could not evaluate xpath")?;
        if let Nodeset(ns) = val {
            for node in ns {
                if let Some(el) = node.element() {
                    let name = QName::with_namespace_uri(Some("http://schemas.openxmlformats.org/wordprocessingml/2006/main"), "lastRow");
                    el.set_attribute_value(name, "1");
                }
            }
        }
        else {
            bail!("XPath did not return Nodeset");
        }

        self.write_document(&doc_doc, &output_path.to_path_buf(), "word/document.xml")?;


        // change fonts (if needed) in Normal and Verbatim Char styles
        if meta.contains(&["base_font_override"]) || meta.contains(&["mono_font_override"]) {
            // I wanted to do this more cleverly with XML, but
            //  sxd-document doesn't fully round-trip and MS Word chokes on the
            //  result even though it's semantically identical. (Unused xmlns attributes
            //  get trimmed, which Word expects for some reason.)
            // Anyway, the XML-based solution is still in the git history
            //  if this makes you sad, as it does me.
            let styles_path = output_path.join("word/styles.xml");
            let mut styles_datums = String::new();
            {
                let mut styles_file = fs::File::open(&styles_path)
                    .with_context(|| format!("Could not open styles file {:?}", &styles_path))?;
                styles_file
                    .read_to_string(&mut styles_datums)
                    .with_context(|| format!("Could not read styles file {:?}", &styles_path))?;
            }

            if let Some(base_override) = meta.get_string(&["base_font_override"]) {
                if CONFIG.get().verbose {
                    println!("Changing base font to {}...", base_override);
                }
                styles_datums = styles_datums.replace("Times New Roman", &base_override);
            }
            if let Some(mono_override) = meta.get_string(&["mono_font_override"]) {
                if CONFIG.get().verbose {
                    println!("Changing mono font to {}...", mono_override);
                }
                styles_datums = styles_datums.replace("Consolas", &mono_override);
            }

            fs::write(&styles_path, styles_datums)
                .with_context(|| format!("Could not write styles file {:?}", &styles_path))?;
        }

        if CONFIG.get().verbose {
            println!("Fixing docx metadata...");
        }
        let props_pkg = self
            .get_file_root(&output_path.to_path_buf(), "docProps/core.xml")
            .with_context(|| format!("Could not get file root for {:?}", &output_path))?;
        let props_doc = props_pkg.as_document();
        let root = props_doc.root();

        if let Some(title) = meta.get_string(&["data", "title"]) {
            self.set_prop(&root, &factory, &context, "dc:title", &title)?;
        }
        if let Some(author) = meta.get_string(&["data", "author"]) {
            self.set_prop(&root, &factory, &context, "dc:creator", &author)?;
            self.set_prop(&root, &factory, &context, "cp:lastModifiedBy", &author)?;
        }
        let mut rev = meta.get_int(&["docx", "revision"]).unwrap_or(-1);
        if rev <= 0 {
            let git_rev_output =
                subprocess::run_command("git", &["rev-list", "--all", "--count"], None, false)?;
            let git_rev = git_rev_output
                .trim()
                .parse::<i64>()
                .context("Could not parse revision count from git output")?;
            rev = std::cmp::max(1, git_rev - 1);
        }
        self.set_prop(&root, &factory, &context, "cp:revision", &rev.to_string())?;

        self.write_document(&props_doc, &output_path.to_path_buf(), "docProps/core.xml")?;

        if let Ok(epoch_str) = std::env::var("SOURCE_DATE_EPOCH") {
            if CONFIG.get().verbose {
                println!("Correcting interior timestamps...");
            }
            for entry in walkdir::WalkDir::new(&output_path) {
                let entry = entry.context("Invalid directory entry in walkdir")?;
                filetime::set_file_mtime(
                    entry.path(),
                    filetime::FileTime::from_unix_time(
                        epoch_str
                            .parse()
                            .context("Could not parse epoch string into u64")?,
                        0,
                    ),
                )
                .context("Could not set file mtime")?;
            }
        }

        if CONFIG.get().verbose {
            println!("Rezipping docx...");
        }
        let outfile = fs::File::create(output_file_path)
            .with_context(|| format!("Could not create file {:?}", &output_file_path))?;
        let mut zipper = zip::ZipWriter::new(outfile);
        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        let mut buffer = Vec::new();
        for entry in walkdir::WalkDir::new(&output_path) {
            let entry = entry.context("Invalid directory entry in walkdir")?;
            let path = entry.path();
            let name = path
                .strip_prefix(&output_path)
                .context("Could not strip output prefix")?;

            if path.is_file() {
                #[allow(deprecated)]
                zipper
                    .start_file_from_path(name, options)
                    .with_context(|| format!("Could not start file {:?}", name))?;
                let mut f = fs::File::open(path)
                    .with_context(|| format!("Could not open file {:?}", &path))?;
                f.read_to_end(&mut buffer)
                    .with_context(|| format!("Could not read file {:?}", &path))?;
                zipper
                    .write_all(&*buffer)
                    .with_context(|| format!("Could not write zipped file {:?}", &path))?;
                buffer.clear();
            } else if !name.as_os_str().is_empty() {
                #[allow(deprecated)]
                zipper
                    .add_directory_from_path(name, options)
                    .with_context(|| format!("Could not add directory to zip {:?}", &name))?;
            }
        }
        zipper.finish().context("Could not finish zip file")?;

        Ok(vec![])
    }
}

impl DocxBuilder {
    fn get_file_root(&self, base: &PathBuf, path_str: &str) -> Result<sxd_document::Package> {
        let path = base.join(path_str);
        let pstr = path.as_os_str();
        let mut file =
            fs::File::open(&path).with_context(|| format!("Could not open file {:?}.", pstr))?;

        let mut zbuff = String::new();
        file.read_to_string(&mut zbuff)
            .with_context(|| format!("Could not read file {:?}.", pstr))?;

        let docx = sxd_document::parser::parse(&zbuff)
            .with_context(|| format!("Could not parse {:?} as XML.", pstr))?;

        Ok(docx)
    }

    fn write_document(
        &self,
        doc: &sxd_document::dom::Document,
        base: &PathBuf,
        path_str: &str,
    ) -> Result<()> {
        let path = base.join(path_str);
        let pstr = path.as_os_str();

        let mut file = fs::File::create(&path)
            .with_context(|| format!("Could not create file {:?}.", pstr))?;

        let writer = sxd_document::writer::Writer::new().set_single_quotes(false);
        writer
            .format_document(&doc, &mut file)
            .context("Unable to output XML document.")?;
        Ok(())
    }

    fn get_xpath(&self, factory: &sxd_xpath::Factory, path: &str) -> Result<sxd_xpath::XPath> {
        let xp = factory
            .build(path)
            .with_context(|| format!("Could not parse XPath: {}", path))?
            .with_context(|| format!("Could not build XPath: {}", path))?;
        Ok(xp)
    }

    fn set_prop(
        &self,
        root: &sxd_document::dom::Root,
        fact: &sxd_xpath::Factory,
        cont: &sxd_xpath::Context,
        id: &str,
        value: &str,
    ) -> Result<()> {
        let xpath_str = format!("//{}", id);
        let xpath = self.get_xpath(&fact, &xpath_str)?;
        let val = xpath
            .evaluate(&cont, *root)
            .context("Could not evaluate xpath")?;
        if let Nodeset(ns) = val {
            let el = match ns.document_order_first() {
                Some(e) => e
                    .element()
                    .with_context(|| format!("Could not convert to Element with {}", id))?,
                None => {
                    let new_el = root.document().create_element(id);
                    root.children()
                        .first()
                        .context("Core properties root has no children")?
                        .element()
                        .context("Core properties root has a first child that is not an Element")?
                        .append_child(new_el);
                    new_el
                }
            };
            el.set_text(value);
        } else {
            bail!("XPath {} did not return Nodeset.", id);
        }

        Ok(())
    }
}
