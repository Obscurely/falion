{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell rec {
  buildInputs = with pkgs; [
    llvmPackages_latest.llvm
    llvmPackages_latest.bintools
    zlib.out
    xorriso
    grub2
    llvmPackages_latest.lld
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
    openssl
    # falion specific, for ui (iced)
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
    # Extra iced dependencies
    expat
    freetype
    freetype.dev
    pkgconfig
  ];

  RUST_BACKTRACE = 1;
  # falion specific for ui (iced)
  LD_LIBRARY_PATH = builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;
}
