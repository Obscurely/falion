# Manual Changes

<!--toc:start-->

- [Manual Changes](#manual-changes)
  - [GitHub Issue Template Config](#github-issue-template-config)
    - [Discord Server (line 7)](#discord-server-line-7)
    - [Email (line 12)](#email-line-12)
  - [Cargo.toml](#cargotoml)
    - [Categories (line 12)](#categories-line-12)
    - [Keywords (line 13)](#keywords-line-13)
    - [Debian section (line 27)](#debian-section-line-27)
  - [README](#readme)
    - [Project about (line 99)](#project-about-line-99)
    - [Features (line 107)](#features-line-107)
    - [Video showcase (line 113)](#video-showcase-line-113)
    - [Third-party libraries (line 125)](#third-party-libraries-line-125)
    - [Usage basics (line 477)](#usage-basics-line-477)
    - [Usage advanced (line 481)](#usage-advanced-line-481)
    - [Road map link (line 488)](#road-map-link-line-488)
    - [FAQ (line 511)](#faq-line-511)
  - [fuzz/README.md](#fuzzreadmemd)
    - [Fuzz harnesses (line 65)](#fuzz-harnesses-line-65)

<!--toc:end-->

## GitHub Issue Template Config

### Discord Server (line 7)

Ideally you should have a link a server/group on something like discord on
matrix. Discord would be preferred since is more used.

### Email (line 12)

Ideally you should also have an email. Unfortunately GitHub doesn't let you put
emails directly or mailto links so you have to use a site like the one I used
to generate a special link with your email.

## Cargo.toml

### Categories (line 12)

Fill the array with any of these [categories](https://crates.io/category_slugs)
with a maximum of 5. Just make sure they represent your project.

### Keywords (line 13)

Fill the array with any keywords that represent your project, the maximum is 5.
For maximum reach I recommend you write 5 distinct ones.

### Debian section (line 27)

Change changeme tag with your desired section. Go to this
[debian page](https://packages.debian.org/bookworm/) click on a section you want
and up top you'll see _Section_… something, that something is what you need to
put here.

## README

### Project about (line 99)

This is the section where you provide your long description explaining how cool
your project is, what are the highlights of it and problems it fixes.

### Features (line 107)

This is the list of features your project has or will have, written using a
checkbox list. Here you should put all your long time goals, things you want
your project to be able to do eventually in order to attract people to keep
following the progress and give you a star.

### Video showcase (line 113)

You should record a video of you using the project, cut out the useless parts
and speed it up, in order to show a user exactly what it does. This will help
people save time and help you get more users, someone may not think they need
your project, but after they see the showcase they might change their minds.

After you have your video go to GitHub to edit your README, delete the changeme
text and drag your video over inside the parenthesis. The video will get
uploaded to GitHub's servers and a direct link to it will be placed there.

### Third-party libraries (line 125)

Any external crates you're using with a hyperlink to them (on lib.rs or
crates.io) and a short phrase explaining why.

### Usage basics (line 477)

Basic concepts a user needs to know in order to use your app. The basics should
be intuitive, if they aren't you should rethink your interface, this is just in
case some users are in doubt.

### Usage advanced (line 481)

Advanced concepts a user might want to know to take full advantage of your app.

### Road map link (line 488)

The ID of the road map associated with your project on GitHub. If you don't have
one go to your repository page, click on _Projects_ up top and create a new one.

### FAQ (line 511)

The _frequently asked questions_, usual questions the users might have about
your project and their answers. You can write them in the template I gave you or
any other way.

## fuzz/README.md

### Fuzz harnesses (line 65)

Information about every fuzz harness you have, what is tests.
