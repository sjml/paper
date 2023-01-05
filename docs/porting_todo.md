## functionality
* maybe set up unit tests? lol
* get_paper_version_stamp should handle a git revision
* output format enums to display string
    - also can they be derived?

## paper internal
* move scripts over here
* license, readme, etc
* homebrew tap?
* release script (github action?)
* libraries?!
  - oh noes!
  - plotting
    - options, in no particular order, based on cursory research:
      - https://crates.io/crates/poloto
      - https://crates.io/crates/plotlib
      - https://crates.io/crates/plotters
      - https://crates.io/crates/criterion-plot
      - https://crates.io/crates/ferrischart
  - docx parsing
    - options, again in no particular order, based on cursory research:
      - https://crates.io/crates/docx
      - https://crates.io/crates/docx-rs
      - https://crates.io/crates/docx-rust
  - pdf manipulation

## idiomatic rust
- using `state` crate, but `once_cell` does something similar (if less ergonomically) and appears to be on its way to inclusion in the standard lib? think about swapping.
- consider wrapping all the `if CONFIG.get().verbose` stuff into a logging system
- need another pass of sweeping the `unwrap()`s
- pandoc_args needing to be a `Vec<String>` makes a lot of code messy -- better way to handle a list of string-like objects?

## cli functionality burndown list
* `build`
  - pull "default_format" from meta
* `save`
* `push`
* `dev`
* `wc`
* `fmt`
* `web`

## handover
* change old repo to `python-paper`
* change current repo to `paper`, along with all project references
