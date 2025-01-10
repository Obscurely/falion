{ lib, stdenv, appimageTools, desktop-file-utils, fetchurl }:

let
  version = "VERSION_PLACEHOLDER";
  name = "falion-${version}";
  pname = "falion";

  plat = {
    x86_64-linux = "";
  }.${stdenv.hostPlatform.system};

  sha256 = {
    x86_64-linux = "SHA_PLACEHOLDER";
  }.${stdenv.hostPlatform.system};

  src = fetchurl {
    url = "https://github.com/Obscurely/falion/releases/download/v${version}-stable/falion-linux.AppImage";
    inherit sha256;
  };

  appimageContents = appimageTools.extractType2 {
    inherit pname version src;
  };
in
appimageTools.wrapType2 rec {
  inherit pname version src;

  extraInstallCommands = ''
    mkdir -p $out/share/pixmaps $out/share/licenses/falion
    cp ${appimageContents}/falion.png $out/share/pixmaps/
    cp ${appimageContents}/falion.desktop $out
    cp ${appimageContents}/LICENSE $out/share/licenses/falion/LICENSE
    mv $out/bin/${name} $out/bin/falion
    ${desktop-file-utils}/bin/desktop-file-install --dir $out/share/applications \
      --set-key Exec --set-value $out/bin/falion \
      --set-key Comment --set-value "falion Linux" \
      --delete-original $out/falion.desktop
  '';

  meta = {
    homepage = "https://github.com/Obscurely/falion";
    description = "An open source, programmed in rust, privacy focused tool for scraping programming resources (like stackoverflow) fast, efficient and asynchronous/parallel using the CLI or GUI. ";
    license = lib.licenses.mit;
    platforms = lib.platforms.linux;
  };
}
