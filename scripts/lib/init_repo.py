import os

from lib.update_logo import update_logo


def init_repo(path_sep: str, cwd: str):
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

    # Get user name and repo name
    repo_name = ""
    user_name = ""
    with open(f"{cwd}{path_sep}.git{path_sep}config", "r") as f:
        content = f.read()
        user_name = content.split("url = https://github.com/")[1].split("/")[0]
        repo_name = content.split(f"url = https://github.com/{user_name}/")[1].split(
            "\n"
        )[0]

    # Get primary email address
    pmail = ""
    with open(f"{cwd}{path_sep}scripts{path_sep}data{path_sep}PMAIL", "r") as f:
        pmail = f.readlines()[0].replace("\n", "")

    # Get secondary email address
    smail = ""
    with open(f"{cwd}{path_sep}scripts{path_sep}data{path_sep}SMAIL", "r") as f:
        smail = f.readlines()[0].replace("\n", "")

    # Create a dictionary with the vars
    vars = {
        "CHANGEME_USER": user_name,
        "CHANGEME_NAME": repo_name,
        "CHANGEME_BIN": repo_name.lower(),
        "changeme_bin": repo_name.lower(),
        "CHANGEME_PMAIL": pmail,
        "CHANGEME_SMAIL": smail,
    }

    # Replace the content
    for file in target_files:
        content = ""
        with open(file, "r") as f:
            content = f.read()

        for var in vars:
            content = content.replace(var, vars[var])

        with open(file, "w") as f:
            f.write(content)

    # Get all dirs list to rename the ones needed
    target_dirs = []

    for path, dirs, files in os.walk(cwd):
        stop = 0
        for dir in skip_dirs:
            if dir in path:
                stop = 1
                continue

        if stop == 1:
            continue

        for dir in dirs:
            target_dirs.append(os.path.join(path, dir))

    # Rename folders & files
    for file in target_files:
        file_split = file.split(path_sep)
        if "CHANGEME" in file_split[-1]:
            file_split[-1] = (
                file_split[-1]
                .replace("CHANGEME_BIN", vars["CHANGEME_BIN"])
                .replace("CHANGEME", vars["CHANGEME_NAME"])
            )
            new_file = path_sep.join(file_split)
            os.rename(file, new_file)

    for dir in target_dirs:
        dir_split = dir.split(path_sep)
        if "CHANGEME" in dir_split[-1]:
            dir_split[-1] = dir_split[-1].replace("CHANGEME", vars["CHANGEME_NAME"])
            new_dir = path_sep.join(dir_split)
            os.rename(dir, new_dir)

    # Run update logo to place the default logo in the project
    update_logo(path_sep, cwd)
