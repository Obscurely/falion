# CHANGELOG

---

## Master

### Breaking Changes

- End user experience is as close as to the original as possible, but the whole
  application has been re-written entirely.
- The whole structure of this repository has changed since I re-written it using
  [my rust template](https://github.com/Obscurely/RustTemplate).

### Added

- Checks in duckduckgo search with more meaningful errors making it way more
  robust.
- Random generated user-agent for reqwest to avoid getting blocked by
  duckduckgo.
- All of the features present on the
  [features list](https://obscurely.github.io/RustTemplate/template/FEATURES.html)
  of [my rust template](https://github.com/Obscurely/RustTemplate).
- Added cargo fuzz harness for the duckduckgo get_links function in order to make
  sure it will not fail making requests over time.
- General performance improvements.
- Added logging across the whole application. Not too much, just enough to be able
  to debug eventual errors. I tried to keep it simple.

### Changed

- Bumped rust version to 1.70.0
- Bumped all the crates to the latest version.
- Instead of the many different functions that handled the duckduckgo searches
  now there is one with half the code, way faster and more robust.
- There is one global client that is used across all objects and threads .This
  makes the program way faster than before.
- Regex is not used anymore since all that look back crap is slow. Switched to
  plain splitting the content. This improves the performance by quite a bit.
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

### Deprecated

None

### Removed

None

### Fixed

- Fix duckduckgo results, now they are actually good.
- Compiling to windows & macOS doesn't fail now.
- GeeksForGeeks pages don't contain the extra crap now.

### Security

- Fixed a bunch of security issues that appeared over time in the last version.
