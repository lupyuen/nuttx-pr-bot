#!/usr/bin/env bash
## Handle PRs for NuttX Kernel and Apps every 10 minutes

## Update the repo
git pull

## Set the GitHub and Gemini Tokens
## export GITHUB_TOKEN=...
## export GEMINI_API_KEY=...
. $HOME/github-token.sh
. $HOME/gemini-token.sh

## Echo commands
set -x

## Enable Rust Logging
export RUST_LOG=info 
export RUST_BACKTRACE=1

## Handle PRs for NuttX Kernel and Apps every 10 minutes
for (( ; ; ))
do
    ## For NuttX Kernel Repo
    cargo run -- --owner apache --repo nuttx
    sleep 300

    ## For NuttX Apps Repo
    cargo run -- --owner apache --repo nuttx-apps
    sleep 300
done
