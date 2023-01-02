old todo from Python version
* have word count skip stuff in brackets?
* watcher functionality for word count
* maybe just watcher stuff in general? 
* portable version (use pyoxidizer?)
  - still runs into the problem of needing LaTeX installed... maybe can probe the environment at setup? hrm. 
* flag to skip title page (for anonymous submission)
  - probably need an alternate docx template... :-/
  - if no title given, derive it from filename? (PDF [and possibly docx] metadata still shows it...)
* save should exit earlier if it's not in a paper directory
* look at multiple footnotes (`\usepackage[multiple]{footmisc}` doesn't work because `footmisc` is already pulled in by the turabian package)
* built-in github action that creates PDF on push?
  - problem: defaults to private repo, would use people's minutes
  - problem: what to do with artifacts? 
  - problem: make new release on every push? 😬
