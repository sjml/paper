## burndown
* watcher functionality
* have docx pagination get fixed if no title page
* make sure missing cite keys print warnings
* set progress image to use timezone? (meta override?)
* use clap to generate completions and add them to dist
* license, readme, etc
* move examples over here, add scripts to update
  * test reproducible builds with tectonic
* change old repo name, change current repo name
* release script (github build action?)
* create homebrew formula, add it to tap

## functionality
* maybe set up unit tests? lol
* double check what the table manipulation that Python was doing in doc_handling.py:91
    - sure it can't be fixed in the Word style?
* docx font override will break if there's interaction between the two
* built-in github action that creates PDF on push?
  - problem: defaults to private repo, would use people's minutes
  - problem: what to do with artifacts? 
  - problem: make new release on every push? 😬

## paper internal
* figure out if it's possible to pre-package the tectonic bundle?
* parallelize the pandoc variable expansion?

## idiomatic rust
- consider wrapping all the `if CONFIG.get().verbose` stuff into a logging system
- need another pass of sweeping the `unwrap()`s
- do a pass w/clippy
