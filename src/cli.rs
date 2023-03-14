use clap::{arg, value_parser, Command};

pub fn cli() -> Command {
    let build_args = vec![
        arg!(-t --"output-format" <FORMAT> "The desired format of the output file")
            .value_parser(["docx", "latex", "latex+pdf", "json"])
            .default_value("docx"),
        arg!(--"docx-revision" <NUM> "Revision number for docx output format; if unset or negative, will use the number of times the project was saved.")
            .value_parser(value_parser!(i64))
            .default_value("-1"),
    ];
    let wc_args = vec![arg!(--full "Show full pre-stripped word count of each file as well.")];

    Command::new("paper")
        .about("Shane’s little paper-{writing|managing|building} utility\n    <https://github.com/sjml/paper>")
        .arg_required_else_help(true)
        .arg(arg!(--version "Print version information and exit"))
        .arg(arg!(-v --verbose "Spam the output log").global(true))
        .subcommand(
            Command::new("new")
                .about("Create the scaffolding for a new writing/research project.")
                .arg(arg!(<PROJECT_NAME> "The name of the directory to create for the project."))
        )
        .subcommand(
            Command::new("init")
                .about("While in an empty directory, set it up for a project.\n(Called as part of the process for `new`.)")
        )
        .subcommand(
            Command::new("dev")
                .about("Set up a project for development work on paper itself.\nDeletes the local `.paper_resources` directory and symlinks the template’s version, so changes here affect the actual program.")
                .hide(true)
        )
        .subcommand(
            Command::new("build")
                .about("Generate versions of the paper ready for submission.")
                .args(&build_args)
        )
        .subcommand(
            Command::new("save")
                .about("Make a git commit with some extra tracking data.")
                .arg(arg!(--message <MESSAGE> "A memo describing this version of the paper (used in the git commit message)"))
        )
        .subcommand(
            Command::new("push")
                .about("Push local git changes to the remote repository, creating one if necessary.")
        )
        .subcommand(
            Command::new("web")
                .about("Open the remote repository’s GitHub site.")
        )
        .subcommand(
            Command::new("wc")
                .about("Print word count metrics for the project, stripping out metadata, citations, and footnotes.")
                .args(&wc_args)
            )
            .subcommand(
                Command::new("watch")
                .about("Watches the content directory and emits new wordcount data on each change.")
                .args(&wc_args)
                .arg(arg!(--build "Rebuild the project before showing word count"))
                .args(&build_args)
        )
        .subcommand(
            Command::new("fmt")
                .about("Run an automated formatter on all the local Markdown files.")
                .arg(arg!(--"no-wrap" "Do not add linebreaks to wrap the Markdown text."))
                .arg(arg!(--columns <NUM> "The number of characters that can be in each line before wrapping.")
                    .value_parser(value_parser!(u32)).default_value("80"))
        )
}
