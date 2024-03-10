# Extras

<!--toc:start-->

- [Extras](#extras)
  - [Signing commits](#signing-commits) (required to follow
    [my](https://github.com/Obscurely) Workflow)
  - [Repository Settings](#repository-settings)
    - [General Section](#general-section)
      - [Social Preview](#social-preview)
      - [Enable Wikis](#enable-wikis)
      - [Sponsorships](#sponsorships)
      - [Discussion](#discussion)
    - [Branches section](#branches-section)
      - [Branch protection rules](#branch-protection-rules) (required to follow
        [my](https://github.com/Obscurely) Workflow)
    - [Rules](#rules)
      - [Rulesets](#rulesets) (required to follow
        [my](https://github.com/Obscurely) Workflow)
    - [Pages](#pages)
    - [Code security and analysis](#code-security-and-analysis)

<!--toc:end-->

## Signing commits

You have to setup commit signing if you want to use
[my](https://github.com/Obscurely) [Workflow](WORKFLOW.md) and in general you
should set this up as it proves your work is actually yours. To actually do this
you can follow the whole _GPG commit signature verification_ section on this
[GitHub article](https://docs.github.com/en/authentication/managing-commit-signature-verification/about-commit-signature-verification#gpg-commit-signature-verification)

## Repository Settings

All of the following assume you are in the Settings page of your repository.
Just click settings on the toolbar up top on your repository page.

### General Section

#### Social Preview

After [I](https://github.com/Obscurely)'ve got a repository about description,
[I](https://github.com/Obscurely) like to generate a social preview with this
[tool](https://www.bannerbear.com/demos/github-social-preview-generator-tool/),
edit it to remove how many stars the repository has, so
[I](https://github.com/Obscurely) don't have to update it, and then add it on
GitHub.

#### Enable Wikis

Enable Wikis and any extra documentation you may have written in the `docs/`
folder publish it here too.

#### Sponsorships

If you haven't added a way to get donations when setting up the template this
might be a good time to do so.

#### Discussion

This is a great way to stay connected with your community. Enable this and just
start by opening a discussion on the general topic beginning with a message like
this:

`Welcome to the General Discussions page! Please don't forget to read our CODE_OF_CONDUCT.md`

### Branches section

#### Branch protection rules

In order to properly follow [my](https://github.com/Obscurely) workflow you will
have to create a new rule with the following settings:

- Branch name pattern: master
- [x] Require a pull request before merging
  - [x] Require approvals (1)
  - [x] Dismiss stale pull request approvals when new commits are pushed
- [x] Require status checks to pass before merging

  - [x] Require branches to be up to date before merging

  Status checks that are required:

  - DevSkim
  - Cargo Deny
  - Test Suite
  - Rustfmt
  - Clippy
  - Super Linter
  - Miri
  - Docs

- [x] Require conversation resolution before merging
- [x] Require signed commits
- [x] Require linear history
- [x] Require deployments to succeed before merging
  - [x] github-pages
- [x] Do not allow bypassing the above settings

### Rules

#### Rulesets

This is going put some restrictions on the branches you work on to then merge to
master, like requiring a linear history and signed commits. So create a new rule
with following options:

- name: Main (or whatever you want)
- Enforcement status: Active
- Target branches: All branches
- [x] Require linear history
- [x] Require signed commits
- [x] Block force pushes

### Pages

The following are settings for the site hosted with the content from the `docs/`
folder

- Source: Deploy from a branch
- Branch: master & `docs/`

### Code security and analysis

- Private vulnerability reporting: Enabled
- Dependency graph: Enabled (it's a must if your repository is public)
- Dependabot alerts: Enabled
- Dismiss low impact alerts: Enabled
- Dependabot security updates: Enabled
- Pull request check failure: High or Higher / Only errors
- Secret scanning: Enabled
  - Push protection: Enabled
