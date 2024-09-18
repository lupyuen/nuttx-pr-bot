# LLM Bot that does PR Review for Apache NuttX RTOS

See https://github.com/apache/nuttx/pull/13494#issuecomment-2357300091

```bash
## See `run.sh` for the Complete Script

## Browse to Google AI Studio > Get API Key > Create API Key > Create API Key In New Project
## https://aistudio.google.com/app/apikey
export GEMINI_API_KEY=...

## GitHub Settings > Developer Settings > Tokens (Classic) > Generate New Token (Classic)
## Check the following:
## repo (Full control of private repositories)
## repo:status (Access commit status)
## repo_deployment (Access deployment status)
## public_repo (Access public repositories)
## repo:invite (Access repository invitations)
## security_events (Read and write security events)
export GITHUB_TOKEN=...

## Enable Rust Logging
export RUST_LOG=info 
export RUST_BACKTRACE=1

## Run the NuttX PR Bot once
cargo run

## Which will: Fetch the Latest 20 PRs
##   If PR Status = Open
##   And PR Comments don't exist
##     Then Call Gemini API to Validate the PR
##     And Post Gemini Response as PR Comment
```

# Run Log

```text
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/nuttx-pr-bot`
[2024-09-18T11:32:59Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13523"
[2024-09-18T11:32:59Z INFO  nuttx_pr_bot] "## Summary\r\n\r\ndetail: Add g_ prefix to can_dlc_to_len and len_to_can_dlc.\r\n\r\n## Impact\r\n\r\n## Testing\r\n\r\n"
[2024-09-18T11:33:14Z INFO  nuttx_pr_bot] Gemini Response: Rest(
        GeminiResponse {
            candidates: [
                Candidate {
                    content: Content {
                        role: Model,
                        parts: [
                            Part {
                                text: Some(
                                    "This PR seems **incomplete** to meet NuttX requirements. Here's why and how to improve it:\n\n**Missing Information:**\n\n* **Summary:**\n    *  **Why?** What's the reasoning behind adding the `g_` prefix? Is it for coding style consistency, namespace collision avoidance, or another reason?\n    * **What?**  Be specific. Which files and functions are affected? \n    * **How?** Briefly explain the technical details of adding the prefix. \n\n* **Impact:**\n    * You need to address **all** impact categories.  Even if the answer is \"NO\", state it explicitly (e.g., \"Impact on build: NO\"). \n    * For this change, pay close attention to:\n        * **Impact on user:** Will existing user code break? Will they need to update their code to use the new prefixed functions?\n        * **Impact on compatibility:** Does this affect backward compatibility?\n        * **Impact on documentation:** Does the documentation need to be updated to reflect the name change? \n\n* **Testing:**\n    * **Insufficient Detail:** \"Build Host(s)\" and \"Target(s)\" sections need concrete examples. List the specific operating systems, architectures, boards, and configurations you used.\n    * **Missing Logs:** You must provide actual testing logs from before and after the change.  This demonstrates that the code works as intended and helps reviewers understand the context.\n\n**Example Improvements:**\n\n**Summary:**\n\n* **Why?** Add `g_` prefix to `can_dlc_to_len` and `len_to_can_dlc` to follow NuttX coding style conventions for global symbols, improving code readability and maintainability.\n* **What?**  This change modifies the declarations and definitions of `can_dlc_to_len` and `len_to_can_dlc` within the CAN driver source files to use the `g_` prefix.\n* **How?**  The prefix is simply added to the function names wherever they are used (declarations, definitions, calls).\n\n**Impact:**\n\n* Is new feature added? Is existing feature changed? NO \n* Impact on user: YES. Existing user code calling these functions will need to be updated to use the prefixed names (e.g., `g_can_dlc_to_len`).\n* Impact on build: NO\n* Impact on hardware: NO\n* Impact on documentation: YES. The documentation for the CAN driver will be updated to reflect the new function names.\n* Impact on security: NO\n* Impact on compatibility: Backward compatibility is broken. A migration guide will be provided in the documentation.\n* Anything else to consider: N/A\n\n**Testing:**\n\n* Build Host(s): \n    * Linux (Ubuntu 20.04), x86_64, GCC 9.4.0\n    * macOS 11.6.1, Apple M1, Clang 13.0.0 \n* Target(s):\n    * Simulator (qemu-system-arm), ARMv7-M, STM32F4Discovery board config\n    * Actual Hardware:  [Specify your board and configuration here]\n\n**Testing logs before change:** [Provide relevant log snippets]\n\n**Testing logs after change:** [Provide relevant log snippets showing the prefixed functions are used correctly] \n\nBy providing this level of detail, you make it easier for maintainers to review and merge your PR quickly. \n",
                                ),
                                inline_data: None,
                                file_data: None,
                                video_metadata: None,
                            },
                        ],
                    },
                    finish_reason: Some(
                        "STOP",
                    ),
                    index: Some(
                        0,
                    ),
                    safety_ratings: [
                        SafetyRating {
                            category: HarmCategorySexuallyExplicit,
                            probability: Negligible,
                            blocked: false,
                        },
                        SafetyRating {
                            category: HarmCategoryHateSpeech,
                            probability: Negligible,
                            blocked: false,
                        },
                        SafetyRating {
                            category: HarmCategoryHarassment,
                            probability: Negligible,
                            blocked: false,
                        },
                        SafetyRating {
                            category: HarmCategoryDangerousContent,
                            probability: Negligible,
                            blocked: false,
                        },
                    ],
                },
            ],
            prompt_feedback: None,
            usage_metadata: Some(
                UsageMetadata {
                    prompt_token_count: 460,
                    candidates_token_count: 743,
                },
            ),
        },
    )
[2024-09-18T11:33:14Z INFO  nuttx_pr_bot] Response TextL "This PR seems **incomplete** to meet NuttX requirements. Here's why and how to improve it:\n\n**Missing Information:**\n\n* **Summary:**\n    *  **Why?** What's the reasoning behind adding the `g_` prefix? Is it for coding style consistency, namespace collision avoidance, or another reason?\n    * **What?**  Be specific. Which files and functions are affected? \n    * **How?** Briefly explain the technical details of adding the prefix. \n\n* **Impact:**\n    * You need to address **all** impact categories.  Even if the answer is \"NO\", state it explicitly (e.g., \"Impact on build: NO\"). \n    * For this change, pay close attention to:\n        * **Impact on user:** Will existing user code break? Will they need to update their code to use the new prefixed functions?\n        * **Impact on compatibility:** Does this affect backward compatibility?\n        * **Impact on documentation:** Does the documentation need to be updated to reflect the name change? \n\n* **Testing:**\n    * **Insufficient Detail:** \"Build Host(s)\" and \"Target(s)\" sections need concrete examples. List the specific operating systems, architectures, boards, and configurations you used.\n    * **Missing Logs:** You must provide actual testing logs from before and after the change.  This demonstrates that the code works as intended and helps reviewers understand the context.\n\n**Example Improvements:**\n\n**Summary:**\n\n* **Why?** Add `g_` prefix to `can_dlc_to_len` and `len_to_can_dlc` to follow NuttX coding style conventions for global symbols, improving code readability and maintainability.\n* **What?**  This change modifies the declarations and definitions of `can_dlc_to_len` and `len_to_can_dlc` within the CAN driver source files to use the `g_` prefix.\n* **How?**  The prefix is simply added to the function names wherever they are used (declarations, definitions, calls).\n\n**Impact:**\n\n* Is new feature added? Is existing feature changed? NO \n* Impact on user: YES. Existing user code calling these functions will need to be updated to use the prefixed names (e.g., `g_can_dlc_to_len`).\n* Impact on build: NO\n* Impact on hardware: NO\n* Impact on documentation: YES. The documentation for the CAN driver will be updated to reflect the new function names.\n* Impact on security: NO\n* Impact on compatibility: Backward compatibility is broken. A migration guide will be provided in the documentation.\n* Anything else to consider: N/A\n\n**Testing:**\n\n* Build Host(s): \n    * Linux (Ubuntu 20.04), x86_64, GCC 9.4.0\n    * macOS 11.6.1, Apple M1, Clang 13.0.0 \n* Target(s):\n    * Simulator (qemu-system-arm), ARMv7-M, STM32F4Discovery board config\n    * Actual Hardware:  [Specify your board and configuration here]\n\n**Testing logs before change:** [Provide relevant log snippets]\n\n**Testing logs after change:** [Provide relevant log snippets showing the prefixed functions are used correctly] \n\nBy providing this level of detail, you make it easier for maintainers to review and merge your PR quickly. \n"
[2024-09-18T11:33:14Z INFO  nuttx_pr_bot] PR Comment: Comment {
        id: CommentId(
            2358224717,
        ),
        node_id: "IC_kwDODZiUac6Mj6dN",
        url: Url {
            scheme: "https",
            cannot_be_a_base: false,
            username: "",
            password: None,
            host: Some(
                Domain(
                    "api.github.com",
                ),
            ),
            port: None,
            path: "/repos/apache/nuttx/issues/comments/2358224717",
            query: None,
            fragment: None,
        },
        html_url: Url {
            scheme: "https",
            cannot_be_a_base: false,
            username: "",
            password: None,
            host: Some(
                Domain(
                    "github.com",
                ),
            ),
            port: None,
            path: "/apache/nuttx/pull/13523",
            query: None,
            fragment: Some(
                "issuecomment-2358224717",
            ),
        },
        issue_url: Some(
            Url {
                scheme: "https",
                cannot_be_a_base: false,
                username: "",
                password: None,
                host: Some(
                    Domain(
                        "api.github.com",
                    ),
                ),
                port: None,
                path: "/repos/apache/nuttx/issues/13523",
                query: None,
                fragment: None,
            },
        ),
        body: Some(
            "[**\\[Experimental Bot, please feedback here\\]**](https://github.com/search?q=repo%3Aapache%2Fnuttx+13494&type=pullrequests)\n\nThis PR seems **incomplete** to meet NuttX requirements. Here's why and how to improve it:\n\n**Missing Information:**\n\n* **Summary:**\n    *  **Why?** What's the reasoning behind adding the `g_` prefix? Is it for coding style consistency, namespace collision avoidance, or another reason?\n    * **What?**  Be specific. Which files and functions are affected? \n    * **How?** Briefly explain the technical details of adding the prefix. \n\n* **Impact:**\n    * You need to address **all** impact categories.  Even if the answer is \"NO\", state it explicitly (e.g., \"Impact on build: NO\"). \n    * For this change, pay close attention to:\n        * **Impact on user:** Will existing user code break? Will they need to update their code to use the new prefixed functions?\n        * **Impact on compatibility:** Does this affect backward compatibility?\n        * **Impact on documentation:** Does the documentation need to be updated to reflect the name change? \n\n* **Testing:**\n    * **Insufficient Detail:** \"Build Host(s)\" and \"Target(s)\" sections need concrete examples. List the specific operating systems, architectures, boards, and configurations you used.\n    * **Missing Logs:** You must provide actual testing logs from before and after the change.  This demonstrates that the code works as intended and helps reviewers understand the context.\n\n**Example Improvements:**\n\n**Summary:**\n\n* **Why?** Add `g_` prefix to `can_dlc_to_len` and `len_to_can_dlc` to follow NuttX coding style conventions for global symbols, improving code readability and maintainability.\n* **What?**  This change modifies the declarations and definitions of `can_dlc_to_len` and `len_to_can_dlc` within the CAN driver source files to use the `g_` prefix.\n* **How?**  The prefix is simply added to the function names wherever they are used (declarations, definitions, calls).\n\n**Impact:**\n\n* Is new feature added? Is existing feature changed? NO \n* Impact on user: YES. Existing user code calling these functions will need to be updated to use the prefixed names (e.g., `g_can_dlc_to_len`).\n* Impact on build: NO\n* Impact on hardware: NO\n* Impact on documentation: YES. The documentation for the CAN driver will be updated to reflect the new function names.\n* Impact on security: NO\n* Impact on compatibility: Backward compatibility is broken. A migration guide will be provided in the documentation.\n* Anything else to consider: N/A\n\n**Testing:**\n\n* Build Host(s): \n    * Linux (Ubuntu 20.04), x86_64, GCC 9.4.0\n    * macOS 11.6.1, Apple M1, Clang 13.0.0 \n* Target(s):\n    * Simulator (qemu-system-arm), ARMv7-M, STM32F4Discovery board config\n    * Actual Hardware:  [Specify your board and configuration here]\n\n**Testing logs before change:** [Provide relevant log snippets]\n\n**Testing logs after change:** [Provide relevant log snippets showing the prefixed functions are used correctly] \n\nBy providing this level of detail, you make it easier for maintainers to review and merge your PR quickly. \n",
        ),
        body_text: None,
        body_html: None,
        author_association: Member,
        user: Author {
            login: "lupyuen",
            id: UserId(
                9960133,
            ),
            node_id: "MDQ6VXNlcjk5NjAxMzM=",
            avatar_url: Url {
                scheme: "https",
                cannot_be_a_base: false,
                username: "",
                password: None,
                host: Some(
                    Domain(
                        "avatars.githubusercontent.com",
                    ),
                ),
                port: None,
                path: "/u/9960133",
                query: Some(
                    "u=d9f94db174776621c4fadd0faad20ab12d0b3e11&v=4",
                ),
                fragment: None,
            },
            gravatar_id: "",
            url: Url {
                scheme: "https",
                cannot_be_a_base: false,
                username: "",
                password: None,
                host: Some(
                    Domain(
                        "api.github.com",
                    ),
                ),
                port: None,
                path: "/users/lupyuen",
                query: None,
                fragment: None,
            },
            html_url: Url {
                scheme: "https",
                cannot_be_a_base: false,
                username: "",
                password: None,
                host: Some(
                    Domain(
                        "github.com",
                    ),
                ),
                port: None,
                path: "/lupyuen",
                query: None,
                fragment: None,
            },
            followers_url: Url {
                scheme: "https",
                cannot_be_a_base: false,
                username: "",
                password: None,
                host: Some(
                    Domain(
                        "api.github.com",
                    ),
                ),
                port: None,
                path: "/users/lupyuen/followers",
                query: None,
                fragment: None,
            },
            following_url: Url {
                scheme: "https",
                cannot_be_a_base: false,
                username: "",
                password: None,
                host: Some(
                    Domain(
                        "api.github.com",
                    ),
                ),
                port: None,
                path: "/users/lupyuen/following%7B/other_user%7D",
                query: None,
                fragment: None,
            },
            gists_url: Url {
                scheme: "https",
                cannot_be_a_base: false,
                username: "",
                password: None,
                host: Some(
                    Domain(
                        "api.github.com",
                    ),
                ),
                port: None,
                path: "/users/lupyuen/gists%7B/gist_id%7D",
                query: None,
                fragment: None,
            },
            starred_url: Url {
                scheme: "https",
                cannot_be_a_base: false,
                username: "",
                password: None,
                host: Some(
                    Domain(
                        "api.github.com",
                    ),
                ),
                port: None,
                path: "/users/lupyuen/starred%7B/owner%7D%7B/repo%7D",
                query: None,
                fragment: None,
            },
            subscriptions_url: Url {
                scheme: "https",
                cannot_be_a_base: false,
                username: "",
                password: None,
                host: Some(
                    Domain(
                        "api.github.com",
                    ),
                ),
                port: None,
                path: "/users/lupyuen/subscriptions",
                query: None,
                fragment: None,
            },
            organizations_url: Url {
                scheme: "https",
                cannot_be_a_base: false,
                username: "",
                password: None,
                host: Some(
                    Domain(
                        "api.github.com",
                    ),
                ),
                port: None,
                path: "/users/lupyuen/orgs",
                query: None,
                fragment: None,
            },
            repos_url: Url {
                scheme: "https",
                cannot_be_a_base: false,
                username: "",
                password: None,
                host: Some(
                    Domain(
                        "api.github.com",
                    ),
                ),
                port: None,
                path: "/users/lupyuen/repos",
                query: None,
                fragment: None,
            },
            events_url: Url {
                scheme: "https",
                cannot_be_a_base: false,
                username: "",
                password: None,
                host: Some(
                    Domain(
                        "api.github.com",
                    ),
                ),
                port: None,
                path: "/users/lupyuen/events%7B/privacy%7D",
                query: None,
                fragment: None,
            },
            received_events_url: Url {
                scheme: "https",
                cannot_be_a_base: false,
                username: "",
                password: None,
                host: Some(
                    Domain(
                        "api.github.com",
                    ),
                ),
                port: None,
                path: "/users/lupyuen/received_events",
                query: None,
                fragment: None,
            },
            type: "User",
            site_admin: false,
            patch_url: None,
            email: None,
        },
        created_at: 2024-09-18T11:33:14Z,
        updated_at: Some(
            2024-09-18T11:33:14Z,
        ),
    }
[2024-09-18T11:33:14Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13523"
[2024-09-18T11:34:20Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13521"
[2024-09-18T11:34:20Z INFO  nuttx_pr_bot] Skipping PR with comments: 13521
[2024-09-18T11:34:25Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13520"
[2024-09-18T11:34:25Z INFO  nuttx_pr_bot] Skipping PR with comments: 13520
[2024-09-18T11:34:31Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13519"
[2024-09-18T11:34:31Z INFO  nuttx_pr_bot] Skipping PR with comments: 13519
[2024-09-18T11:34:36Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13518"
[2024-09-18T11:34:36Z INFO  nuttx_pr_bot] Skipping PR with comments: 13518
[2024-09-18T11:34:42Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13513"
[2024-09-18T11:34:42Z INFO  nuttx_pr_bot] Skipping PR with comments: 13513
[2024-09-18T11:34:47Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13511"
[2024-09-18T11:34:47Z INFO  nuttx_pr_bot] Skipping PR with comments: 13511
[2024-09-18T11:34:53Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13509"
[2024-09-18T11:34:53Z INFO  nuttx_pr_bot] Skipping PR with comments: 13509
[2024-09-18T11:34:58Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13507"
[2024-09-18T11:34:58Z INFO  nuttx_pr_bot] Skipping PR with comments: 13507
[2024-09-18T11:35:04Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13506"
[2024-09-18T11:35:04Z INFO  nuttx_pr_bot] Skipping PR with comments: 13506
[2024-09-18T11:35:09Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13502"
[2024-09-18T11:35:09Z INFO  nuttx_pr_bot] Skipping PR with comments: 13502
[2024-09-18T11:35:15Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13499"
[2024-09-18T11:35:15Z INFO  nuttx_pr_bot] Skipping PR with comments: 13499
[2024-09-18T11:35:20Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13498"
[2024-09-18T11:35:20Z INFO  nuttx_pr_bot] Skipping PR with comments: 13498
[2024-09-18T11:35:26Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13494"
[2024-09-18T11:35:26Z INFO  nuttx_pr_bot] Skipping PR with comments: 13494
[2024-09-18T11:35:31Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13487"
[2024-09-18T11:35:31Z INFO  nuttx_pr_bot] Skipping PR with comments: 13487
[2024-09-18T11:35:36Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13486"
[2024-09-18T11:35:36Z INFO  nuttx_pr_bot] Skipping PR with comments: 13486
[2024-09-18T11:35:42Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13459"
[2024-09-18T11:35:42Z INFO  nuttx_pr_bot] Skipping PR with comments: 13459
[2024-09-18T11:35:47Z INFO  nuttx_pr_bot] "https://api.github.com/repos/apache/nuttx/pulls/13458"
[2024-09-18T11:35:47Z INFO  nuttx_pr_bot] Skipping PR with comments: 13458
[2024-09-18T11:35:52Z INFO  nuttx_pr_bot] Skipping Problematic PR: 13456
[2024-09-18T11:35:57Z INFO  nuttx_pr_bot] Skipping Problematic PR: 13453
```
