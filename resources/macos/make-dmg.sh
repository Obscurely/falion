#!/bin/sh
create-dmg \
	--volname falion \
	--volicon "../../assets/images/logo.icns" \
	--hide-extension "falion.app" \
	--background "../../assets/images/dmg-background.png" \
	--window-size 600 450 \
	--icon-size 94 \
	--icon "falion.app" 141 249 \
	--app-drop-link 458 249 \
	../../target/osx/falion-macos-installer.dmg \
	./falion.app
