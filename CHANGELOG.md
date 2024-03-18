# CHANGELOG

---

## Master

### Breaking Changes

None

### Added

None

### Changed

None

### Deprecated

None

### Removed

None

### Fixed

None

### Security

None

## v1.0.1

### Fixed v1.0.1

- Fix builds on some platforms not being done without qt.
- Fix arch pkg thanks to setting the backend in ./cargo/config.toml
- Force the theme to be dark. In the future it will adapt to the system and look
  prettier for now this is it
- Fix homebrew pkg
- Fix windows installer not creating desktop or start menu shortcuts
- Fix appimage not taking CLI args and consequently fixing the nix file

### Security v1.0.1

- Fix multiple security issues from creates

## v1.0.0-stable

### Highlights v1.0.0

- Whole project rewritten, now with more than year more of experience, learning
  and building with rust. The cli part of the project provides almost the same
  experience, except for the keybinds which are more VIM like now.
- Heavy, heavy performance improvements across the board.
- Added a GUI that is automatically launched when not running from the cli.
- Program is actually fully cross-platform now.
- Many more checks in place preventing bad results, errors and other ux
  problems. Also results are much better now.
- Client is configured to mimic a browser and a limit of five results per
  resource is set in order to prevent any sort of rate limiting in normal use.
- Added unit tests, fuzzers and tested the program under many conditions to make
  sure it works as intended.
- Bumped crates and rust version to the latest.
- Fixed security issues.
- Better repository structure, thanks to
  [my rust template](https://github.com/Obscurely/RustTemplate).
- Code is much more maintainable with added code documentation, more idiomatic
  design and logging for easier debugging.

### Breaking Changes v1.0.0

- End user experience is as close as to the original as possible, but the whole
  application has been rewritten entirely.
- The whole structure of this repository has changed since I rewritten it using
  [my rust template](https://github.com/Obscurely/RustTemplate).

### Added v1.0.0

- Checks in duckduckgo search with more meaningful errors making it way more
  robust.
- Random generated user-agent for reqwest to avoid getting blocked by
  duckduckgo.
- All of the features present on the
  [features list](https://obscurely.github.io/RustTemplate/template/FEATURES.html)
  of [my rust template](https://github.com/Obscurely/RustTemplate).
- Added cargo fuzz harness for the duckduckgo get_links function in order to
  make sure it will not fail making requests over time.
- General performance improvements.
- Added logging across the whole application. Not too much, just enough to be
  able to debug eventual errors. I tried to keep it simple.
- Added code documenation.
- Added tests.
- Added a GUI made with slint that is used when not running the program from the
  cli.

### Changed v1.0.0

- Bumped rust version to 1.76.0
- Bumped all the crates to the latest version.
- Instead of the many different functions that handled the duckduckgo searches
  now there is one with half the code, way faster and more robust.
- There is one global client that is used across all objects and threads .This
  makes the program way faster than before.
- Regular expressions are not used anymore since all that look back crap is
  slow. Switched to plain splitting the content. This improves the performance
  by quite a bit.
- Rewrote ddg.rs completely. Performance improvements and way better results.
- Rewrote stackoverflow.rs completely. Many performance improvements, using only
  one global client, many checks in place.
- Rewrote stackexchange.rs completely. Same as Stack Overflow, many performance
  improvements, one global client and multiple checks in place.
- Rewrote geeksforgeeks.rs completely. Same as Stack Overflow, performance
  improvements, one global client, multiple checks in place and the page is
  rendered better.
- Rewrote ddg_search.rs completely. Same as Stack Overflow, performance
  improvements, one global client, multiple checks in place and the pages are
  rendered better.
- Rewrote github_gist.rs completely. Same as Stack Overflow, performance
  improvements, one global client, multiple checks in place + actual parallel
  requesting the gist files instead of concurent and simplified process.
- Better error handling.
- Better argument parsing using clap instead of arg_parse.
- Rewrote the cli. More efficient, less error-prone and just better in general.
  Note: it works almost the same, this is just the backend.
- Made code way more maintainable.
- Actual cli code is in it's own module.
- Replaced IndexMap everywhere with either a vector of tuples or dashmap in ui
  and hashbrown in cli for improved performance.

### Fixed v1.0.0

- Fix duckduckgo results, now they are actually good.
- Compiling to windows & macOS doesn't fail now.
- GeeksForGeeks pages don't contain the extra crap now.

### Security v1.0.0

- Fixed a bunch of security issues that appeared over time in the last version.
