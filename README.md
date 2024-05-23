# GIT-ACP: A Git Command Line Tool to manage your git workflow.

## Description

This is a small rust code to automate the github workflow. It includes the following git commands to add files/changes to a GitHub Repository:

- `git add .`
- `git commit -m "Commit Message"`
- `git push origin main`

## Usage

Navigate into your repository and run the following command:
`git-acp "Your Commit Message"`

## Installation

Inorder to build this project you need to have Rust and Cargo installed on your system. You can do it from [Rust Website](https://www.rust-lang.org/tools/install).

1. Git Clone the repository to your workspace, and navgigate into the directory.
   `git clone https://github.com/amar-nath-bhat/git-acp.git`
   `cd git-acp`
2. Now build it with Cargo.
   `cargo build --release`
3. Now install the binary.
   `cargo install --path .`
4. Finally run the `git-acp` command with your commit message.
   `git-acp "Your Commit Message"`
