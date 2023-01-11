find and fix "My Great Title"

## functionality
* maybe set up unit tests? lol
* double check what the table manipulation that Python was doing in doc_handling.py:91
    - sure it can't be fixed in the Word style?
* docx font override will break if there's interaction between the two
* progress image should use timezone (meta set?)
* not throwing warnings on missing cite keys?

## paper internal
* move scripts and examples over here, make build scripts
* license, readme, etc
* homebrew tap?
* release script (github action?)
* move content directory to config
* single-file tex build?
* parallelize the pandoc variable expansion?

## idiomatic rust
- consider wrapping all the `if CONFIG.get().verbose` stuff into a logging system
- need another pass of sweeping the `unwrap()`s

## handover
* change old repo to `python-paper`
* change current repo to `paper`, along with all project references
