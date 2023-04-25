## burndown
* get bottle building from personal homebrew tap
    - waiting on Apple Silicon runners on GitHub: https://github.com/github/roadmap/issues/528

## documentation
* `\Adonai{}`

## functionality
* biblical citations should insert space before themselves if it's not there
    - while we're at it, any way to make other citations smart about quotes/punctuation/etc? 
* biblical citations mess up ibid for things around them
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
