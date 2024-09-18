#!/usr/bin/env bash
## Handle PRs every minute

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
    cargo run
    sleep 600
done
