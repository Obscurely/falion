{
  description = "Rust devshell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
        with pkgs; {
          devShells.default = mkShell rec {
            buildInputs = [
              rust-bin.stable.latest.default
              llvmPackages_latest.llvm
              llvmPackages_latest.bintools
              llvmPackages_latest.lld
              openssl
              pkg-config
              fd
              zlib.out
              xorriso
              grub2
              cargo-audit # audit dependencies in order to scan for supply chain attacks
              cargo-fuzz # fuzzing tool
              cargo-deny # tool to deny crates based on checks.
              cargo-edit # manage cargo dependencies
              cargo-deb # pkg rust apps for debian
              cmake
              git
              gcc
              python311
              python311Packages.pillow # this is for python repo script
              openssl
              # falion specific
              libxkbcommon
              libGL
              # WINIT_UNIX_BACKEND=wayland
              wayland
              # WINIT_UNIX_BACKEND=x11
              xorg.libXcursor
              xorg.libXrandr
              xorg.libXi
              xorg.libX11
              # fonts
              fontconfig
              # Extra possible dependencies
              expat
              freetype
              freetype.dev
            ];

            shellHook = ''
              alias find=fd
            '';

            RUST_BACKTRACE = 1;
            # falion specific for ui (iced)
            LD_LIBRARY_PATH = builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;
            # which theme to build the application with
            SLINT_STYLE = "fluent-dark";
          };
        }
    );
}
