{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            exa
            fd
            rust-bin.stable.latest.default
            llvmPackages_latest.llvm
            llvmPackages_latest.bintools
            zlib.out
            xorriso
            grub2
            llvmPackages_latest.lld
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
            # falion specific
            openssl
          ];

          shellHook = ''
            alias ls=exa
            alias find=fd
          '';
        };
      }
    );
}
