import os


def update_desc(path_sep: str, cwd: str):
    # Load desc
    desc = ""
    with open(f"scripts{path_sep}data{path_sep}DESC", "r") as f:
        desc = f.read().replace("\n", "")

    # Directories we can ignore
    skip_dirs = [
        f"{cwd}{path_sep}.git",
        f"{cwd}{path_sep}.mypy_cache",
        f"{cwd}{path_sep}assets",
        f"{cwd}{path_sep}fuzz{path_sep}artifacts",
        f"{cwd}{path_sep}fuzz{path_sep}corpus",
        f"{cwd}{path_sep}fuzz{path_sep}target",
        f"{cwd}{path_sep}scripts",
        f"{cwd}{path_sep}target",
    ]

    # Get all the files we want to manipulate with their full paths.
    target_files = []

    for path, _, files in os.walk(cwd):
        stop = 0
        for dir in skip_dirs:
            if dir in path:
                stop = 1
                continue

        if stop == 1:
            continue

        for file in files:
            if (
                file != "Cargo.lock"
                and ".png" not in file
                and ".ico" not in file
                and ".icns" not in file
            ):
                target_files.append(os.path.join(path, file))

    for file in target_files:
        content = ""

        with open(file, "r") as f:
            content = f.read()

        content = content.replace("CHANGEME_DESC", desc)

        with open(file, "w") as f:
            f.write(content)
