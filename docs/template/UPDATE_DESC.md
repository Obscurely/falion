# Update the Description (short)

**This section assumes you followed [How to Start](START.md) completely which you
should.**

**!!! This command can only be run once, after that you'll have to change it
manually !!!**

This is the short description that you would put in your GitHub repository about
section.

1. Put your desired description in scripts/data/DESC.

   It shouldn't contain any new lines, they will get replaced anyway.

1. In the root of your repository run the following command

   ```shell
   python3 scripts/repo.py udesc
   ```

   This will replace the placeholder for the description everywhere.
