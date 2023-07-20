# How to Start

1. Create a new GitHub (only GitHub works) repository from
   [this template](https://docs.github.com/en/enterprise-cloud@latest/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template#creating-a-repository-from-a-template)

1. Clone your new repository & cd into it

   ```shell
   git clone https://github.com/{YOUR_USERNAME}/{YOUR_REPOSITORY_NAME}
   cd {YOUR_REPOSITORY_NAME}
   ```

1. Install python (if you don't have it)

   - On Windows

     a. Download & run the installer from their
     [official site](https://www.python.org/downloads/windows/)

     b. Install it using winget (replace 3.11 with whatever version is the
     latest)

     ```shell
     winget install -e --id Python.Python.3.11
     ```

     Note: This may or may not add python3 to path, if not do it
     [manually](https://realpython.com/add-python-to-path/).

   - On Linux

     Use your distribution's package manager, for example on Arch Linux:

     ```shell
     sudo pacman -Sy python3
     ```

     or if you are on NixOS just do the following and skip the next step

     ```shell
     nix-shell
     ```

   - On macOS

     a. Download & run the installer from their
     [official site](https://www.python.org/downloads/macos/)

     b. Install it using homebrew ([install homebrew](https://brew.sh/))

     ```shell
     brew install python3
     ```

1. Install/Upgrade pip and install the
   [Pillow](https://pypi.org/project/Pillow/) library

   ```shell
   python3 -m pip install --upgrade pip
   python3 -m pip install --upgrade Pillow
   ```

1. Replace the default emails in the scripts/data/PMAIL & scripts/data/SMAIL
   files

   PMAIL - primary email address associated with your user.

   SMAIL - secondary email where you should receive ISSUE & SECURITY reports
   from users

   You could just use the same email in both files, I just like a little
   fragmentation.

1. Run the following command in the root of your project

   ```shell
   python3 scripts/repo.py init
   ```

   This will initialize the repository by replacing falion, falion,
   Obscurely, obscurely.adrian@protonmail.com, obscurely.message@protonmail.com placeholders in every file with
   the according values and rename all the folders & files that have a CHANGEME
   tag as well as converting and copying the default logo.png where needed.

1. Finish by pushing your changes

   ```shell
   git commit -am "init repo"
   git push
   ```

## Additional Resources

- [Update the Logo](UPDATE_LOGO.md)
- [Update the Description](UPDATE_DESC.md)
