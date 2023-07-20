{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    llvmPackages_latest.llvm
    llvmPackages_latest.bintools
    zlib.out
    xorriso
    grub2
    llvmPackages_latest.lld
    rustup
    rustc
    cargo
    rustfmt
    clippy
    cargo-audit # audit dependencies in order to scan for supply chain attacks 
    cargo-fuzz # fuzzing tool
    cargo-deny # tool to deny crates based on checks.
    cargo-edit # manage cargo dependencies
    cargo-deb # pkg rust apps for debian
    cmake
    git
    gcc
    pkg-config
    python311
    python311Packages.pillow # this is for python repo script
  ];

  RUST_BACKTRACE = 1;
}
