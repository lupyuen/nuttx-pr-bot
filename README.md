# LLM Bot that does PR Review for Apache NuttX RTOS

See https://github.com/apache/nuttx/pull/13494#issuecomment-2357300091

```bash
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
