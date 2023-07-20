import glob
import shutil

from PIL import Image


def update_logo(path_sep: str, cwd: str):
    # Get bin name
    bin_name = ""
    with open(f"{cwd}{path_sep}.git{path_sep}config", "r") as f:
        content = f.read()
        user_name = content.split("url = https://github.com/")[1].split("/")[0]
        bin_name = (
            content.split(f"url = https://github.com/{user_name}/")[1]
            .split("\n")[0]
            .lower()
        )

    # Open base logo png file
    logo = Image.open(f"assets{path_sep}images{path_sep}logo.png")

    # Save it as ico
    logo.save(
        f"assets{path_sep}images{path_sep}logo.ico", format="ICO", size=[(256, 256)]
    )

    # Save it as icns
    logo.save(
        f"assets{path_sep}images{path_sep}logo.icns",
        format="ICNS",
        size=[(256, 256, 2)],
    )

    # Copy icns to macos folder
    macos_app_folder = glob.glob(f"resources{path_sep}macos{path_sep}*.app")[0]
    shutil.copyfile(
        f"assets{path_sep}images{path_sep}logo.icns",
        f"{macos_app_folder}{path_sep}Contents{path_sep}Resources"
        f"{path_sep}AppIcon.icns",
    )

    # Copy png logo to appdir linux folder
    linux_appdir = glob.glob(f"resources{path_sep}linux{path_sep}*.AppDir")[0]
    shutil.copyfile(
        f"assets{path_sep}images{path_sep}logo.png",
        f"{linux_appdir}{path_sep}{bin_name}.png",
    )

    # Resize png logo and copy to appdir & desktop folders
    sizes = [16, 32, 64, 128, 256, 512]

    # appdir folder
    with Image.open(f"assets{path_sep}images{path_sep}logo.png") as logo:
        for size in sizes:
            resized = logo.resize((size, size))
            resized.save(
                f"{linux_appdir}{path_sep}usr{path_sep}share{path_sep}icons"
                f"{path_sep}hicolor{path_sep}{size}x{size}{path_sep}apps"
                f"{path_sep}{bin_name}.png"
            )
            resized.save(
                f"resources{path_sep}linux{path_sep}desktop{path_sep}icons"
                f"{path_sep}hicolor{path_sep}{size}x{size}{path_sep}apps"
                f"{path_sep}{bin_name}.png"
            )
