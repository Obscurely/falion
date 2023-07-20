# Features

<!--toc:start-->

- [Features](#features)
  - [Automatic](#automatic)
    - [GitHub automatic](#github-automatic)
    - [Rust automatic](#rust-automatic)
  - [Static](#static)
    - [GitHub static](#github-static)
    - [Rust static](#rust-static)

<!--toc:end-->

## Automatic

### GitHub automatic

- Python script that automatically initializes the repository according to your
  username and project name
- Python script that automatically updates the logo by converting & copying it
- Python script that automatically updates the short description (GitHub about
  description)
- Automatic issue labeling according to which folders have changes
- Automatically greeting users creating issues and PRs
- Weekly scheduled Dependabot checks

### Rust automatic

- Automatic checks running on each push
  - Cargo deny
  - Cargo test
  - Rustfmt check
  - Clippy
  - Super linter
  - Cargo miri
  - Rust docs check
- Automatic releases when pushing a new tag
  - Linux
    - Binary
    - AppImage
    - AUR (stable pkgs only if the tag contains stable)
    - Nix file
    - Deb file
  - MacOS
    - Binary
    - App Folder
    - DMG installer
    - Homebrew (only if the tag contains stable)
  - Windows
    - Executable
    - Msi installer
  - All Platforms
    - Crates.io (only if the tag contains stable)
- Daily scheduled cargo audit runs
- Daily scheduled DevSkim runs

## Static

### GitHub static

- Repository is under [MIT license](https://mit-license.org/)
- One of the best READMEs for Rust projects (based on
  [this README](https://github.com/othneildrew/Best-README-Template))
- Issue Templates (written in modern yml format)
  - Bug report
  - Feature request
  - Config with links to discussion, discord & email
- Pull Request Template
- CHANGELOG template
- CODE OF CONDUCT file
- CONTRIBUTING file
- SECURITY file
- GitHub pages set up

### Rust static

- Cargo.toml fully configured
  - Complete information about package
  - Best binary performance settings
  - A list of amazing crates
- Cargo deny setup
  - Only allow permissive licenses for crates
  - Only allow crates that run at least on x86_64 linux, macOS & windows
  - And all the other default cargo deny checks
- Rustfmt config to enable more formatting options (doesn't change the defaults)
- Cargo-fuzz set up with a special README & an example
- Rust build file that integrates the logo into the Windows binary
- Rust integration & unit test examples
- Shell.nix file for nix users with everything needed
