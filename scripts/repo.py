import os
import platform
import sys

from lib.init_repo import init_repo
from lib.update_desc import update_desc
from lib.update_logo import update_logo

# Get current directory
cwd = os.getcwd()

# Make sure we are in the root folder and not in the scripts folder
# + get the platform path separator \ for Windows and / for Linux.
path_sep = ""
if platform.system() == "Windows":
    path_split = cwd.split("\\")
    path_sep = "\\"
    if "scripts" == path_split[-1]:
        path_split.pop()
        os.chdir("\\".join(path_split))
else:
    path_split = cwd.split("/")
    path_sep = "/"
    if "scripts" == path_split[-1]:
        path_split.pop()
        os.chdir("/".join(path_split))

# Reset the current directory
cwd = os.getcwd()
print(cwd)

# get arguments
try:
    run_arg = sys.argv[1]
except IndexError:
    print(
        """
Available arguments:

init  - change variables according to your repo.
ulogo - take assets/logo.png as base, convert it and copy it anywhere needed
udesc - take the description in scripts/data/DESC and update it everywhere.
    """
    )
    exit()

# Main script run
if run_arg == "init":
    init_repo(path_sep, cwd)
elif run_arg == "ulogo":
    update_logo(path_sep, cwd)
elif run_arg == "udesc":
    update_desc(path_sep, cwd)
else:
    print("Argument not recognized!")
