# LLM Bot that does PR Review for Apache NuttX RTOS

See https://github.com/apache/nuttx/pull/13494#issuecomment-2357300091

```bash
## See `run.sh`
export GITHUB_TOKEN=...
export GEMINI_API_KEY=...
export RUST_LOG=info 
export RUST_BACKTRACE=1
cargo run

## Every Minute: Fetch the Latest 20 PRs
##   If PR Status = Open
##   And PR Comments don't exist
##     Then Call Gemini API to Validate the PR
##     And Post Gemini Response as PR Comment
```

# Run Log

```text
$ cargo run
   Compiling nuttx-pr-bot v0.1.0 (/home/luppy/nuttx-pr-bot)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.96s
     Running `target/debug/nuttx-pr-bot`
[2024-09-18T08:50:25Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13521"
[2024-09-18T08:50:25Z INFO  nuttx_pr_bot] Skipping PR with comments: 13521
[2024-09-18T08:50:31Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13520"
[2024-09-18T08:50:31Z INFO  nuttx_pr_bot] Skipping PR with comments: 13520
[2024-09-18T08:50:36Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13519"
[2024-09-18T08:50:36Z INFO  nuttx_pr_bot] Skipping PR with comments: 13519
[2024-09-18T08:50:42Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13518"
[2024-09-18T08:50:42Z INFO  nuttx_pr_bot] Skipping PR with comments: 13518
[2024-09-18T08:50:47Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13513"
[2024-09-18T08:50:47Z INFO  nuttx_pr_bot] Skipping PR with comments: 13513
[2024-09-18T08:50:52Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13511"
[2024-09-18T08:50:52Z INFO  nuttx_pr_bot] Skipping PR with comments: 13511
[2024-09-18T08:50:58Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13509"
[2024-09-18T08:50:58Z INFO  nuttx_pr_bot] Skipping PR with comments: 13509
```
