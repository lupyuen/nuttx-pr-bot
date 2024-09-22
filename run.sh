#!/usr/bin/env bash
## Handle PRs every minute

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

## Handle PRs every 10 minutes
for (( ; ; ))
do
    ## For NuttX Repo
    cargo run -- --owner apache --repo nuttx

    ## For NuttX Apps Repo
    # cargo run -- --owner apache --repo nuttx-apps

    ## Wait a while
    sleep 600
done
