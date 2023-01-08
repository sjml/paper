## functionality
* maybe set up unit tests? lol
* get_paper_version_stamp should handle a git revision
* double check what the table manipulation that Python was doing in doc_handling.py:91
    - sure it can't be fixed in the Word style?
* docx font override will break if there's interaction between the two
* dev command for windows and release mode?
* progress image should use timezone (meta set?)

## paper internal
* move scripts and examples over here, make build scripts
* license, readme, etc
* homebrew tap?
* release script (github action?)
* move content directory to config
* libraries?!
  - oh noes!
  - pdf manipulation

## idiomatic rust
- using `state` crate, but `once_cell` does something similar (if less ergonomically) and appears to be on its way to inclusion in the standard lib? think about swapping.
- consider wrapping all the `if CONFIG.get().verbose` stuff into a logging system
- need another pass of sweeping the `unwrap()`s
    - also look for use of `?` without setting a context
    - expects, too
- pandoc_args needing to be a `Vec<String>` makes a lot of code messy -- better way to handle a list of string-like objects?
- the independent impl for DocxBuilder is kinda yucky

## cli functionality burndown list
* `build`
  - let the metadata set a default format
  - other builders (docx+pdf, latex+pdf)
  - _record_build_data

## handover
* change old repo to `python-paper`
* change current repo to `paper`, along with all project references
