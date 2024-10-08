## burndown
* get bottle building from personal homebrew tap
    - waiting on Apple Silicon runners on GitHub: https://github.com/github/roadmap/issues/528

## lock tectonic and pandoc
* can "just" build tectonic into the executable (would be nice to have a custom bundle but that looks to be a righteous pain at the moment)
* pull pandoc executable from GitHub release based on a specific version
* if tectonic build proves to be a pain, try and do likewise

## functionality
* word count fix?
* watch doesn't build if the output directory doesn't already exist?
* redo wc output to be a valid pandoc table with bottom row delimited that can get piped to GFM?
* maybe do some profiling; `wc` takes 300 ms to run?! performance not **too** important, but that's _slooooow_...
  * seems to go even slower with multiple content files? blergh.
* allow date to only give year or year-month
* biblical citations should insert space before themselves if it's not there
    - while we're at it, any way to make other citations smart about quotes/punctuation/etc? 
* biblical citations mess up ibid for things around them
* figure out if we can pass [multiple] to footmisc somehow to get comma-separated footnotes
* yaml setting for content files to skip when calculating total word count
* set progress image to use timezone? (meta override?)
* maybe set up unit tests? lol
* built-in github action that creates PDF on push?
  - problem: defaults to private repo, would use people's minutes
  - problem: what to do with artifacts? 
  - problem: make new release on every push? 😬

## paper internal
* figure out if it's possible to pre-package the tectonic bundle?

## idiomatic rust
- consider wrapping all the `if CONFIG.get().verbose` stuff into a logging system
- need another pass of sweeping the `unwrap()`s
- do a pass w/clippy
