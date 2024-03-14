# Workflow

<!--toc:start-->

- [Workflow](#workflow)
  - [Git commands](#git-commands)
  - [Project start](#project-start)
    - [Planning](#planning)
    - [Beginning development](#beginning-development)
  - [Usual development](#usual-development)
  - [Making a release](#making-a-release)
  - [Keeping a changelog](#keeping-a-changelog)
  - [Coding workflow](#coding-workflow)
  - [Extras](#extras)
    - [Road map](#road-map)
    - [Third-party libraries](#third-party-libraries)
    - [Fuzzing](#fuzzing)
  - [Advice](#advice)
    - [Commits](#commits)
    - [Multiple branches](#multiple-branches)

<!--toc:end-->

This section assumes you have fully followed [START.md](START.md),
[REQ.md](REQ.md) and the 2 required sections for the Workflow in
[EXTRAS.md](EXTRAS.md)

If don't have a first version follow [project start](#project-start) after that
you should follow [usual development](#usual-development).

Keep in mind [extras](#extras) and [advice](#advice).

For an optimal coding experience I suggest you follow a
[test driven development model](#coding-workflow).

## Git commands

This are the used git commands in [my](https://github.com/Obscurely) Workflow or
some that you may need and an explanation to what they do.\
unstaged - before doing `git add .`\
staged/index - after doing `git add .`\
remote - the target branch on GitHub

- `git add .` Stages all changed files to the index.
- `git status` See what files you've changed
- `git commit -m "message"` commits the changes to the repository.
- `git push` or `git push -u origin some-branch` pushes the changes to the
  remote (on GitHub)
- `git diff` view all the unstaged changes you've made; add `--staged` to
  compare to index; also you can do `git diff some_file` to view changes made to
  only one file
- 'git restore some_file' to restore a file, add `--staged` to restore a staged
  file
- 'git clean -fd' to completly remove any changes you've made that **are not**
  staged
- `git log --oneline` to nicely view the log of commits
- `git ls-tree commit-id` to view what files a commit changed, where `commit-id`
  is the ID of the target commit or if you want the commit before the last one
  use `HEAD~1` (can be a different number)
- `git show HEAD~1:some_file` view the stage of a file in the commit before the
  last commit, of course `HEAD~1` can be any commit ID and you don't have to
  specify a file.
- `git restore --source=HEAD~1 some_file` restore some_file to the state it was
  in 2 commits ago, of course `HEAD~1` can be any commit ID
- `git checkout -b next` create a new branch next and swtich to it
- `git switch master` to switch back to master
- `git diff next` to see the differences between the next branch and master
- `git branch -d next` to delete the next branch
- `git revert HEAD~1` to revert to the changes made to 2 commits ago, you may
  encounter conflicts and have to do something like
  `git rm some_file && git revert --continue`, of course `HEAD~1` can be any
  commit ID.

## Project start

### Planning

When beginning a new project you should start by writting your
[project about](MANUAL.md#project-about-line-99) to reflect and get an idea of
what you want it to do. Don't go overkill just write enough to solve the problem
you had originally. Also write the [features](MANUAL.md#features-line-107) your
project needs to have in order to solve your original problem.

The above is the basic idea. I like to go beyond that and write personal, more
detailed docs and make Excalidraw diagrams to help me see my vision better.

### Beginning development

For the first basic version of the software, my workflow is basically the
following

1. Checkout the code to a branch called v0.1.0

   ```shell
   git checkout -b v0.1.0
   ```

1. Start making changes and with every new meaningful change do the following

   - Complete your [features](MANUAL.md#features-line-107) list if you add any
     new.
   - Write any new [usage](MANUAL.md#usage-basics-line-477) the app has
     available.
   - Add a new q&a on your [FAQ](MANUAL.md#faq-line-511) if needed.

   If all is good commmit the new changes.

   ```shell
   git add .
   git diff --staged # check changes
   git commit -m "add new feature"
   git push
   ```

1. When you are done with making v0.1.0 do the following

   First make a [video showcase](MANUAL.md#video-showcase-line-113) for your
   project.\
   Then merge the branch v0.1.0 into master by doing a PR on GitHub.\
   And then in order to make a release:

   ```shell
   git switch master
   git pull
   git checkout -d v0.1.0
   git tag v0.1.0-stable # it's important your tag has stable in it
   git push --tags
   ```

   This will create a new release and CD pipelines will start compiling, making
   & publishing your releases. More about [making releases](#making-a-release)

## Usual development

After the first version this is how I would continue

1. Checkout the code to a new branch, like feature-x

```shell
git checkout -b feature-x
```

1. Add the new feature on the [features](MANUAL.md#features-line-107) list

1. With every meaningful change

   - Write any new [usage](MANUAL.md#usage-basics-line-477) the app has
     available.
   - Add a new q&a on your [FAQ](MANUAL.md#faq-line-511) if needed.

   Check the new changes

   ```shell
   git add .
   git diff --staged
   ```

   If everything is good follow [keeping a changelog](#keeping-a-changelog) and
   once making a changelog push the changes.

   ```shell
   git add CHANGELOG.md
   git commit -m "change something"
   git push
   ```

1. Once done implementing the feature do a PR on GitHub to master

1. Once the PR was accepted create a new release

   ```shell
   git switch master
   git pull
   git checkout -d feature-x
   git tag v0.2.0-stable
   git push --tags
   ```

   More info about [making a release](#making-a-release)

## Making a release

Because of all the automation this template has you don't have to fear or be
tired of making new releases just because of a tiny bug since everything is
really fast and simple.

First you have to keep in mind release tags have to look like the following.\
`v0.1.0-stable` - for a stable release where homebrew pkg, AUR stable pkgs &
crates.io get updated\
`v0.1.0-beta` - where beta can be anything and the only external thing that gets
updated is the AUR git pkg.

Second, while mostly optional since there are things in the CD pipelines that
take care of this to an extent is updating the version of your software inside
Cargo.toml to match the one you are releasing, don't worry if you make a
mistake, the CD pipeline will fix it for you.

Third once you have all the code you want for your next release on the master
branch run the following

```shell
git tag v0.1.0-stable
git push --tags
```

This will make a new tag which will trigger the CD pipeline and start making the
releases for you. Now that you have a new release on GitHub you should copy the
contents of your CHANGELOG for the latest version and dump them in your release
by doing an edit.

## Keeping a changelog

While the release system generates a simple changelog from PRs and commits it's
not enough, we want our own, human written, changelog so our users can easily
and exactly know what changed. [I](https://github.com/Obscurely) know there are
also GitHub action that beautify this way of making changelog, but still it's
not good enough.

So what I do is everytime I have some changes staged

```shell
git add .
```

I run the following

```shell
git diff --staged
```

Look at all the changes I've made and start filling the CHANGELOG file

Once I'm about to release a new version I will copy the CHANGELOG and make it
fresh (no changes written) and keep the tag master, while for the current
version one I will delete any unfilled section and change the text from master
to whatever the current version tag is.

## Coding workflow

Coding in rust is all about functions so the way I do it is the following:

1. Write a test.
1. Write just enough code, no matter how bad, just to pass the test.
1. Refactor the code
1. Improve performance

Where needed I will also write integration tests and [fuzz harnesses](#fuzzing)

## Extras

Things to do while programming.

### Road map

As you think of a new feature you want your project to have don't hesitate to
add it on the road map so you eventually get to making it. Also add that feature
as unchecked on your README in the [features](MANUAL.md#features-line-107)
section.

### Third-party libraries

When adding a new crate to your project the first thing you should do is run
`cargo deny check` to see if the crate complies with your requirements. Second
thing is adding the crates to your README
[third-party libraries](MANUAL.md#third-party-libraries-line-125) section.

### Fuzzing

Any time you have a crucial function that you want to make sure it works
properly under any circumstances you will want to create a fuzz harness and run
it for some time. For this you can read the README inside the fuzz folder and
look at the example. Additionally there are a few YouTube videos you can watch
about cargo-fuzz.

## Advice

### Commits

Every commit should solve **one type of issue and one only**. You shouldn't have
commits fixing typos, bugs and making code changes together.

One common example would be you are making some code changes and then you
suddenly find a bug or a typo, what you should do is fix that and then do
something like this:

```shell
git add file_with_bug
git commit -m "fix bug x"
git push
```

After which you can continue with making your code changes and then commit those
separately.

### Multiple branches

You should use multiple branches to separate your work (hence why if you use the
settings proposed you can't even push to master directly).

One example of why you should do this is for example you are working on
feature-x and then you suddenly find a bug or god forbids a vulnerability in
your production code. What you can now do is create a new branch to fix that and
merge it fast into master and make a new hot fix release while not having to
commit any unfinished feature. Also you would merge that bugfix into your
feature branch as well (or not if you prefer).
