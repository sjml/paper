## functionality
* maybe set up unit tests? lol
* double check what the table manipulation that Python was doing in doc_handling.py:91
    - sure it can't be fixed in the Word style?
* docx font override will break if there's interaction between the two
* dev command for windows and release mode?
* progress image should use timezone (meta set?)
* not throwing warnings on missing cite keys?

## paper internal
* move scripts and examples over here, make build scripts
* license, readme, etc
* homebrew tap?
* release script (github action?)
* move content directory to config
* single-file tex build?

## idiomatic rust
- consider wrapping all the `if CONFIG.get().verbose` stuff into a logging system
- need another pass of sweeping the `unwrap()`s
    - also look for use of `?` without setting a context
    - expects, too
- pandoc_args needing to be a `Vec<String>` makes a lot of code messy -- better way to handle a list of string-like objects?

## cli functionality burndown list
* `build`
  - let the metadata set a default format

## handover
* change old repo to `python-paper`
* change current repo to `paper`, along with all project references
