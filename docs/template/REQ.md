# Requirements

<!--toc:start-->

- [Requirements](#requirements)
  - [Public GitHub repository](#public-github-repository)
  - [GitHub actions permissions](#github-actions-permissions)
  - [Version scheme](#version-scheme)
  - [homebrew-tap repository](#homebrew-tap-repository)
  - [Action secrets](#action-secrets)
    - [API_TOKEN_GITHUB](#api_token_github)
    - [AUR](#aur)
      - [AUR_USERNAME](#aur_username)
      - [AUR_EMAIL](#aur_email)
      - [AUR_SSH_PRIVATE_KEY](#aur_ssh_private_key)
    - [CRATES_TOKEN](#crates_token)

<!--toc:end-->

All of the following have to be met in order for this template to work to its
full potential and not encounter any issues. If you for example don't want to
upload to the AUR just edit `.github/workflows/release.yml` accordingly.

## Public GitHub repository

For all of the actions, branch rules etc. to be available (if you are a free
GitHub user) the repository has to be public.

## GitHub actions permissions

On your GitHub repository page go to Settings, Actions, General (the link should
look like this: `https://github.com/{USER}/{REPO}/settings/actions`), scroll
down to `Workflow permissions` and make sure `Read and write permissions` is
selected. This is so the actions can create new releases.

## Version scheme

You must use [SemVer](https://semver.org/). Every release you make should look
something like this: `v0.1.0-stable` or `v0.1.0-beta`. Note, only stable
releases will make external changes like updating AUR stable pkgs or homebrew
pkg.

## homebrew-tap repository

You will need to have public repository on your account named `homebrew-tap`.
When the you create a new stable release the GitHub actions will update/publish
the homebrew version for macOS.

## Action secrets

All of the following have to be added in the secrets tab for actions. To get
here go to your GitHub repository page -> Settings -> Secrets and variables ->
Actions -> Secrets tab (the link should look like this:
`https://github.com/{USER}/{REPO}/settings/secrets/actions`). In order to add a
secret click `New repository secret`.

### API_TOKEN_GITHUB

[Create a new access token](https://github.com/settings/tokens/new), copy it and
save it somewhere safe, you will not able to after. Create a new secret with the
name `API_TOKEN_GITHUB` and paste the token in the secret box (second box). The
token you create will be of type classic and have only the repository scope
selected.

### AUR

You'll need an AUR account with ssh keys for the next steps, if you don't have
one:

1. [Create a new account](https://aur.archlinux.org/register), you only need to
   provide a username, email and password (on the next page). Unfortunately you
   will need to boot an Arch Linux iso (if you don't use Arch) to complete the
   captcha verification, but it's only a one time thing.

1. To generate a new key pair run:
   `ssh-keygen -t ed25519 -C "your_email@example.com"`, replacing the example
   email with the one you used for your AUR account.

1. Open you _.pub_ file in `~/.ssh` and copy everything besides your email, go
   to the _My account_ page on AUR and paste it in the _SSH Public Key_ section.
   Scroll down fill in your password and hit update.

#### AUR_USERNAME

Create a new secret with the name `AUR_USERNAME` fill in your AUR username in
the secret box (second box).

#### AUR_EMAIL

Create a new secret with the name `AUR_EMAIL` fill in your AUR email in the
secret box (second box).

#### AUR_SSH_PRIVATE_KEY

Open your ssh private key file in `~/.ssh` (the one that doesn't end in .pub)
and copy everything. Create a new secret with the name `AUR_SSH_PRIVATE_KEY` and
paste your key in the secret box (second box).

### CRATES_TOKEN

You'll need a crates.io token for the following steps, if you don't have one:

1. Login on crates.io using GitHub.
1. Go to [API tokens pages](https://crates.io/settings/tokens).
1. Click _New Token_ and save this token somewhere safe, you'll not be able
   after.

Create a new secret with the name `CRATES_TOKEN` fill in your crates.io token in
the secret box (second box).
