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

### Changed

- Bumped rust version to 1.70.0
- Bumped all the crates to the latest version.
- Results are now got through the HTML version of duckduckgo making it faster
  and less prone to breaking over time.
- Instead of the many different functions that handled the duckduckgo searches
  now there is one with half the code, way faster and more robust.

### Deprecated

None

### Removed

None

### Fixed

- Switching to the HTML version of duckduckgo made it so the results we get are
  in order of importance.
- Compiling to windows & macOS doesn't fail now.

### Security

- Fixed a bunch of security issues appeared over time in the last version.
