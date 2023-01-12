## burndown
* get bottle building from personal homebrew tap

## functionality
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
