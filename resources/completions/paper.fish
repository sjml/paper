complete -c paper -n "__fish_use_subcommand" -l version -d 'Print version information and exit'
complete -c paper -n "__fish_use_subcommand" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c paper -n "__fish_use_subcommand" -f -a "new" -d 'Create the scaffolding for a new writing/research project.'
complete -c paper -n "__fish_use_subcommand" -f -a "init" -d 'While in an empty directory, set it up for a project.
(Called as part of the process for `new`.)'
complete -c paper -n "__fish_use_subcommand" -f -a "dev" -d 'Set up a project for development work on paper itself.
Deletes the local `.paper_resources` directory and symlinks the template’s version, so changes here affect the actual program.'
complete -c paper -n "__fish_use_subcommand" -f -a "build" -d 'Generate versions of the paper ready for submission.'
complete -c paper -n "__fish_use_subcommand" -f -a "save" -d 'Make a git commit with some extra tracking data.'
complete -c paper -n "__fish_use_subcommand" -f -a "push" -d 'Push local git changes to the remote repository, creating one if necessary.'
complete -c paper -n "__fish_use_subcommand" -f -a "web" -d 'Open the remote repository’s GitHub site.'
complete -c paper -n "__fish_use_subcommand" -f -a "wc" -d 'Print word count metrics for the project, stripping out metadata, citations, and footnotes.'
complete -c paper -n "__fish_use_subcommand" -f -a "watch" -d 'Watches the content directory and emits new wordcount data on each change.'
complete -c paper -n "__fish_use_subcommand" -f -a "fmt" -d 'Run an automated formatter on all the local Markdown files.'
complete -c paper -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c paper -n "__fish_seen_subcommand_from new" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_seen_subcommand_from new" -s h -l help -d 'Print help information'
complete -c paper -n "__fish_seen_subcommand_from init" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_seen_subcommand_from init" -s h -l help -d 'Print help information'
complete -c paper -n "__fish_seen_subcommand_from dev" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_seen_subcommand_from dev" -s h -l help -d 'Print help information'
complete -c paper -n "__fish_seen_subcommand_from build" -s t -l output-format -d 'The desired format of the output file' -r -f -a "{docx	,latex	,latex+pdf	,json	}"
complete -c paper -n "__fish_seen_subcommand_from build" -l docx-revision -d 'Revision number for docx output format; if unset or negative, will use the number of times the project was saved.' -r
complete -c paper -n "__fish_seen_subcommand_from build" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_seen_subcommand_from build" -s h -l help -d 'Print help information'
complete -c paper -n "__fish_seen_subcommand_from save" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_seen_subcommand_from save" -s h -l help -d 'Print help information'
complete -c paper -n "__fish_seen_subcommand_from push" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_seen_subcommand_from push" -s h -l help -d 'Print help information'
complete -c paper -n "__fish_seen_subcommand_from web" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_seen_subcommand_from web" -s h -l help -d 'Print help information'
complete -c paper -n "__fish_seen_subcommand_from wc" -l full -d 'Show full pre-stripped word count of each file as well.'
complete -c paper -n "__fish_seen_subcommand_from wc" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_seen_subcommand_from wc" -s h -l help -d 'Print help information'
complete -c paper -n "__fish_seen_subcommand_from watch" -s t -l output-format -d 'The desired format of the output file' -r -f -a "{docx	,latex	,latex+pdf	,json	}"
complete -c paper -n "__fish_seen_subcommand_from watch" -l docx-revision -d 'Revision number for docx output format; if unset or negative, will use the number of times the project was saved.' -r
complete -c paper -n "__fish_seen_subcommand_from watch" -l full -d 'Show full pre-stripped word count of each file as well.'
complete -c paper -n "__fish_seen_subcommand_from watch" -l build -d 'Rebuild the project before showing word count'
complete -c paper -n "__fish_seen_subcommand_from watch" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_seen_subcommand_from watch" -s h -l help -d 'Print help information'
complete -c paper -n "__fish_seen_subcommand_from fmt" -l columns -d 'The number of characters that can be in each line before wrapping.' -r
complete -c paper -n "__fish_seen_subcommand_from fmt" -l no-wrap -d 'Do not add linebreaks to wrap the Markdown text.'
complete -c paper -n "__fish_seen_subcommand_from fmt" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_seen_subcommand_from fmt" -s h -l help -d 'Print help information'
complete -c paper -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from new; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from dev; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from save; and not __fish_seen_subcommand_from push; and not __fish_seen_subcommand_from web; and not __fish_seen_subcommand_from wc; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from fmt; and not __fish_seen_subcommand_from help" -f -a "new" -d 'Create the scaffolding for a new writing/research project.'
complete -c paper -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from new; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from dev; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from save; and not __fish_seen_subcommand_from push; and not __fish_seen_subcommand_from web; and not __fish_seen_subcommand_from wc; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from fmt; and not __fish_seen_subcommand_from help" -f -a "init" -d 'While in an empty directory, set it up for a project.
(Called as part of the process for `new`.)'
complete -c paper -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from new; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from dev; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from save; and not __fish_seen_subcommand_from push; and not __fish_seen_subcommand_from web; and not __fish_seen_subcommand_from wc; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from fmt; and not __fish_seen_subcommand_from help" -f -a "dev" -d 'Set up a project for development work on paper itself.
Deletes the local `.paper_resources` directory and symlinks the template’s version, so changes here affect the actual program.'
complete -c paper -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from new; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from dev; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from save; and not __fish_seen_subcommand_from push; and not __fish_seen_subcommand_from web; and not __fish_seen_subcommand_from wc; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from fmt; and not __fish_seen_subcommand_from help" -f -a "build" -d 'Generate versions of the paper ready for submission.'
complete -c paper -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from new; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from dev; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from save; and not __fish_seen_subcommand_from push; and not __fish_seen_subcommand_from web; and not __fish_seen_subcommand_from wc; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from fmt; and not __fish_seen_subcommand_from help" -f -a "save" -d 'Make a git commit with some extra tracking data.'
complete -c paper -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from new; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from dev; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from save; and not __fish_seen_subcommand_from push; and not __fish_seen_subcommand_from web; and not __fish_seen_subcommand_from wc; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from fmt; and not __fish_seen_subcommand_from help" -f -a "push" -d 'Push local git changes to the remote repository, creating one if necessary.'
complete -c paper -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from new; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from dev; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from save; and not __fish_seen_subcommand_from push; and not __fish_seen_subcommand_from web; and not __fish_seen_subcommand_from wc; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from fmt; and not __fish_seen_subcommand_from help" -f -a "web" -d 'Open the remote repository’s GitHub site.'
complete -c paper -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from new; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from dev; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from save; and not __fish_seen_subcommand_from push; and not __fish_seen_subcommand_from web; and not __fish_seen_subcommand_from wc; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from fmt; and not __fish_seen_subcommand_from help" -f -a "wc" -d 'Print word count metrics for the project, stripping out metadata, citations, and footnotes.'
complete -c paper -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from new; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from dev; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from save; and not __fish_seen_subcommand_from push; and not __fish_seen_subcommand_from web; and not __fish_seen_subcommand_from wc; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from fmt; and not __fish_seen_subcommand_from help" -f -a "watch" -d 'Watches the content directory and emits new wordcount data on each change.'
complete -c paper -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from new; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from dev; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from save; and not __fish_seen_subcommand_from push; and not __fish_seen_subcommand_from web; and not __fish_seen_subcommand_from wc; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from fmt; and not __fish_seen_subcommand_from help" -f -a "fmt" -d 'Run an automated formatter on all the local Markdown files.'
complete -c paper -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from new; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from dev; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from save; and not __fish_seen_subcommand_from push; and not __fish_seen_subcommand_from web; and not __fish_seen_subcommand_from wc; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from fmt; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'