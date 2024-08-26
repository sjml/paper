# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_paper_global_optspecs
	string join \n version v/verbose h/help
end

function __fish_paper_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_paper_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_paper_using_subcommand
	set -l cmd (__fish_paper_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c paper -n "__fish_paper_needs_command" -l version -d 'Print version information and exit'
complete -c paper -n "__fish_paper_needs_command" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_paper_needs_command" -s h -l help -d 'Print help'
complete -c paper -n "__fish_paper_needs_command" -f -a "new" -d 'Create a new directory with the scaffolding for a new writing/research project.'
complete -c paper -n "__fish_paper_needs_command" -f -a "init" -d 'While in an empty directory, set it up for a project. (Called as part of the process for `new`.)'
complete -c paper -n "__fish_paper_needs_command" -f -a "dev" -d 'Set up a project for development work on paper itself. Deletes the local `.paper_resources` directory and symlinks the template’s version, so changes here affect the actual program.'
complete -c paper -n "__fish_paper_needs_command" -f -a "build" -d 'Generate versions of the paper ready for submission.'
complete -c paper -n "__fish_paper_needs_command" -f -a "save" -d 'Make a git commit with some extra tracking data.'
complete -c paper -n "__fish_paper_needs_command" -f -a "push" -d 'Push local git changes to the remote repository, creating one if necessary.'
complete -c paper -n "__fish_paper_needs_command" -f -a "web" -d 'Open the remote repository’s GitHub site.'
complete -c paper -n "__fish_paper_needs_command" -f -a "wc" -d 'Print word count metrics for the project, stripping out metadata, citations, and footnotes.'
complete -c paper -n "__fish_paper_needs_command" -f -a "watch" -d 'Watches the content directory and emits new wordcount data on each change, optionally rebuilding.'
complete -c paper -n "__fish_paper_needs_command" -f -a "fmt" -d 'Run an automated formatter on all the local Markdown files.'
complete -c paper -n "__fish_paper_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c paper -n "__fish_paper_using_subcommand new" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_paper_using_subcommand new" -s h -l help -d 'Print help'
complete -c paper -n "__fish_paper_using_subcommand init" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_paper_using_subcommand init" -s h -l help -d 'Print help'
complete -c paper -n "__fish_paper_using_subcommand dev" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_paper_using_subcommand dev" -s h -l help -d 'Print help'
complete -c paper -n "__fish_paper_using_subcommand build" -s t -l output-format -d 'The desired format of the output file' -r -f -a "{docx\t'',latex\t'',latex+pdf\t'',json\t''}"
complete -c paper -n "__fish_paper_using_subcommand build" -l docx-revision -d 'Revision number for docx output format; if unset or negative, will use the number of times the project was saved.' -r
complete -c paper -n "__fish_paper_using_subcommand build" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_paper_using_subcommand build" -s h -l help -d 'Print help'
complete -c paper -n "__fish_paper_using_subcommand save" -l message -d 'A memo describing this version of the paper (used in the git commit message)' -r
complete -c paper -n "__fish_paper_using_subcommand save" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_paper_using_subcommand save" -s h -l help -d 'Print help'
complete -c paper -n "__fish_paper_using_subcommand push" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_paper_using_subcommand push" -s h -l help -d 'Print help'
complete -c paper -n "__fish_paper_using_subcommand web" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_paper_using_subcommand web" -s h -l help -d 'Print help'
complete -c paper -n "__fish_paper_using_subcommand wc" -l full -d 'Show full pre-stripped word count of each file as well.'
complete -c paper -n "__fish_paper_using_subcommand wc" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_paper_using_subcommand wc" -s h -l help -d 'Print help'
complete -c paper -n "__fish_paper_using_subcommand watch" -s t -l output-format -d 'The desired format of the output file' -r -f -a "{docx\t'',latex\t'',latex+pdf\t'',json\t''}"
complete -c paper -n "__fish_paper_using_subcommand watch" -l docx-revision -d 'Revision number for docx output format; if unset or negative, will use the number of times the project was saved.' -r
complete -c paper -n "__fish_paper_using_subcommand watch" -l full -d 'Show full pre-stripped word count of each file as well.'
complete -c paper -n "__fish_paper_using_subcommand watch" -l build -d 'Rebuild the project before showing word count'
complete -c paper -n "__fish_paper_using_subcommand watch" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_paper_using_subcommand watch" -s h -l help -d 'Print help'
complete -c paper -n "__fish_paper_using_subcommand fmt" -l columns -d 'The number of characters that can be in each line before wrapping.' -r
complete -c paper -n "__fish_paper_using_subcommand fmt" -l no-wrap -d 'Do not add linebreaks to wrap the Markdown text.'
complete -c paper -n "__fish_paper_using_subcommand fmt" -s v -l verbose -d 'Spam the output log'
complete -c paper -n "__fish_paper_using_subcommand fmt" -s h -l help -d 'Print help'
complete -c paper -n "__fish_paper_using_subcommand help; and not __fish_seen_subcommand_from new init dev build save push web wc watch fmt help" -f -a "new" -d 'Create a new directory with the scaffolding for a new writing/research project.'
complete -c paper -n "__fish_paper_using_subcommand help; and not __fish_seen_subcommand_from new init dev build save push web wc watch fmt help" -f -a "init" -d 'While in an empty directory, set it up for a project. (Called as part of the process for `new`.)'
complete -c paper -n "__fish_paper_using_subcommand help; and not __fish_seen_subcommand_from new init dev build save push web wc watch fmt help" -f -a "dev" -d 'Set up a project for development work on paper itself. Deletes the local `.paper_resources` directory and symlinks the template’s version, so changes here affect the actual program.'
complete -c paper -n "__fish_paper_using_subcommand help; and not __fish_seen_subcommand_from new init dev build save push web wc watch fmt help" -f -a "build" -d 'Generate versions of the paper ready for submission.'
complete -c paper -n "__fish_paper_using_subcommand help; and not __fish_seen_subcommand_from new init dev build save push web wc watch fmt help" -f -a "save" -d 'Make a git commit with some extra tracking data.'
complete -c paper -n "__fish_paper_using_subcommand help; and not __fish_seen_subcommand_from new init dev build save push web wc watch fmt help" -f -a "push" -d 'Push local git changes to the remote repository, creating one if necessary.'
complete -c paper -n "__fish_paper_using_subcommand help; and not __fish_seen_subcommand_from new init dev build save push web wc watch fmt help" -f -a "web" -d 'Open the remote repository’s GitHub site.'
complete -c paper -n "__fish_paper_using_subcommand help; and not __fish_seen_subcommand_from new init dev build save push web wc watch fmt help" -f -a "wc" -d 'Print word count metrics for the project, stripping out metadata, citations, and footnotes.'
complete -c paper -n "__fish_paper_using_subcommand help; and not __fish_seen_subcommand_from new init dev build save push web wc watch fmt help" -f -a "watch" -d 'Watches the content directory and emits new wordcount data on each change, optionally rebuilding.'
complete -c paper -n "__fish_paper_using_subcommand help; and not __fish_seen_subcommand_from new init dev build save push web wc watch fmt help" -f -a "fmt" -d 'Run an automated formatter on all the local Markdown files.'
complete -c paper -n "__fish_paper_using_subcommand help; and not __fish_seen_subcommand_from new init dev build save push web wc watch fmt help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
