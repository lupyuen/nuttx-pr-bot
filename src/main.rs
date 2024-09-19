// Every Minute: Fetch the Latest 20 PRs
//   If PR Status = Open
//   And PR Comments don't exist
//     Then Call Gemini API to Validate the PR
//     And Post Gemini Response as PR Comment

use std::env;
use log::info;
use octocrab::{models::{reactions, reactions::ReactionContent, IssueState}, params, Octocrab};
use google_generative_ai_rs::v1::{
    api::Client,
    gemini::{request::Request, Content, Model, Part, Role},
};

// Production Repo
const OWNER: &str = "apache";
const REPO: &str = "nuttx";

// Testing Repo
// const OWNER: &str = "lupyuen2";
// const REPO: &str = "wip-nuttx";

// Requirements for PR Review
const REQUIREMENTS: &str =
r#####"
# Here are the requirements for a NuttX PR

## Summary

* Why change is necessary (fix, update, new feature)?
* What functional part of the code is being changed?
* How does the change exactly work (what will change and how)?
* Related [NuttX Issue](https://github.com/apache/nuttx/issues) reference if applicable.
* Related NuttX Apps [Issue](https://github.com/apache/nuttx-apps/issues) / [Pull Request](https://github.com/apache/nuttx-apps/pulls) reference if applicable.

## Impact

* Is new feature added? Is existing feature changed?
* Impact on user (will user need to adapt to change)? NO / YES (please describe if yes).
* Impact on build (will build process change)? NO / YES (please descibe if yes).
* Impact on hardware (will arch(s) / board(s) / driver(s) change)? NO / YES (please describe if yes).
* Impact on documentation (is update required / provided)? NO / YES (please describe if yes).
* Impact on security (any sort of implications)? NO / YES (please describe if yes).
* Impact on compatibility (backward/forward/interoperability)? NO / YES (please describe if yes).
* Anything else to consider?

## Testing

I confirm that changes are verified on local setup and works as intended:
* Build Host(s): OS (Linux,BSD,macOS,Windows,..), CPU(Intel,AMD,ARM), compiler(GCC,CLANG,version), etc.
* Target(s): arch(sim,RISC-V,ARM,..), board:config, etc.

Testing logs before change:

```
your testing logs here
```

Testing logs after change:
```
your testing logs here
```
"#####;

/// Validate the Latest PRs and post the PR Reviews as PR Comments
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init the Logger
    env_logger::init();

    // Init the GitHub Client
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = octocrab::Octocrab::builder().personal_token(token).build()?;

    // Fetch the 20 Newest Pull Requests that are Open
    let pr_list = octocrab.pulls(OWNER, REPO).list()
        .state(params::State::Open)
        .sort(params::pulls::Sort::Created)
        .direction(params::Direction::Descending)
        .per_page(20)
        .send()
        .await?;

    // Process every PR
    for pr in pr_list {
        // info!("{:#?}", pr);
        let pr_id = pr.number;
        // info!("{:#?}", pr_id);
        process_pr(&octocrab, pr_id).await?;

        // Wait 5 seconds
        std::thread::sleep(
            std::time::Duration::from_secs(5)
        );
    }

    // Return OK
    Ok(())
}

/// Validate the PR and post the PR Review as a PR Comment
async fn process_pr(octocrab: &Octocrab, pr_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Skip these PRs. Why do they fail?
    if pr_id == 13456 || pr_id == 13453 {
        info!("Skipping Problematic PR: {}", pr_id);
        return Ok(());
    }

    // Fetch the PR
    let pr = octocrab
        .pulls(OWNER, REPO)
        .get(pr_id).await?;
    // info!("{:#?}", pr);
    info!("{:#?}", pr.url);

    // Skip if PR State is Not Open
    if pr.state.unwrap() != IssueState::Open {
        info!("Skipping Closed PR: {}", pr_id);
        return Ok(());
    }

    // Skip if PR contains Comments
    if pr.comments.unwrap() > 0 {
        info!("Skipping PR with comments: {}", pr_id);
        return Ok(());
    }

    // Check for Multiple Commits
    let mut precheck = String::new();
    let commits = octocrab
        .pulls(OWNER, REPO)
        .pull_number(pr_id)
        .commits()
        .await;
    let commits = commits.unwrap().items;
    if commits.len() > 1 {
        precheck.push_str(
            &format!("__Squash The Commits:__ This PR contains {} Commits. Please Squash the Multiple Commits into a Single Commit.\n\n", commits.len())
        );
    }

    // Check for Empty Commit Message
    let mut empty_message = false;
    for commit in commits.iter() {
        // Message should be "title\n\nbody"
        let message = &commit.commit.message;
        if message.find("\n").is_some() {
        } else {
            info!("Missing Commit Message: {:#?}", message);
            empty_message = true;
            break;
        }
    }
    if empty_message {
        precheck.push_str(
            "__Fill In The Commit Message:__ This PR contains a Commit with an Empty Commit Message. Please fill in the Commit Message with the PR Summary.\n\n"
        );
    }

    // Get the PR Body
    let body = pr.body.unwrap_or("".to_string());
    info!("{:#?}", body);

    // TODO: Retry Gemini API up to 3 times

    // Fetch the PR Reactions
    let reactions = get_reactions(octocrab, pr_id).await?;

    // // Init the Gemini Client
    // let client = Client::new_from_model(
    //     Model::Gemini1_5Pro,  // For Production
    //     // Model::GeminiPro,  // For Testing
    //     env::var("GEMINI_API_KEY").unwrap().to_string()
    // );

    // // Compose the Prompt for Gemini Request: PR Requirements + PR Body
    // let input = 
    //     REQUIREMENTS.to_string() +
    //     "\n\n# Does this PR meet the NuttX Requirements? Please be concise\n\n" +
    //     &body;

    // // For Testing:
    // // let input = "# Here are the requirements for a NuttX PR\n\n## Summary\n\n* Why change is necessary (fix, update, new feature)?\n* What functional part of the code is being changed?\n* How does the change exactly work (what will change and how)?\n* Related [NuttX Issue](https://github.com/apache/nuttx/issues) reference if applicable.\n* Related NuttX Apps [Issue](https://github.com/apache/nuttx-apps/issues) / [Pull Request](https://github.com/apache/nuttx-apps/pulls) reference if applicable.\n\n## Impact\n\n* Is new feature added? Is existing feature changed?\n* Impact on user (will user need to adapt to change)? NO / YES (please describe if yes).\n* Impact on build (will build process change)? NO / YES (please descibe if yes).\n* Impact on hardware (will arch(s) / board(s) / driver(s) change)? NO / YES (please describe if yes).\n* Impact on documentation (is update required / provided)? NO / YES (please describe if yes).\n* Impact on security (any sort of implications)? NO / YES (please describe if yes).\n* Impact on compatibility (backward/forward/interoperability)? NO / YES (please describe if yes).\n* Anything else to consider?\n\n## Testing\n\nI confirm that changes are verified on local setup and works as intended:\n* Build Host(s): OS (Linux,BSD,macOS,Windows,..), CPU(Intel,AMD,ARM), compiler(GCC,CLANG,version), etc.\n* Target(s): arch(sim,RISC-V,ARM,..), board:config, etc.\n\nTesting logs before change:\n\n```\nyour testing logs here\n```\n\nTesting logs after change:\n```\nyour testing logs here\n```\n\n# Does this PR meet the NuttX Requirements?\n\n## Summary\nBCH: Add readonly configuration for BCH devices\n## Impact\nNONE\n## Testing\n";

    // // Compose the Gemini Request
    // let txt_request = Request {
    //     contents: vec![Content {
    //         role: Role::User,
    //         parts: vec![Part {
    //             text: Some(input.to_string()),
    //             inline_data: None,
    //             file_data: None,
    //             video_metadata: None,
    //         }],
    //     }],
    //     tools: vec![],
    //     safety_settings: vec![],
    //     generation_config: None,
    //     system_instruction: None,
    // };

    // // Send the Gemini Request
    // let response = client.post(30, &txt_request).await?;
    // info!("Gemini Response: {:#?}", response);

    // // Get the Gemini Response
    // let response_text = response.rest().unwrap()
    //     .candidates.first().unwrap()
    //     .content.parts.first().unwrap()
    //     .text.clone().unwrap();
    // info!("Response TextL {:#?}", response_text);

    // // Header for PR Comment
    // let header = "[**\\[Experimental Bot, please feedback here\\]**](https://github.com/search?q=repo%3Aapache%2Fnuttx+13494&type=pullrequests)";

    // // Compose the PR Comment
    // let comment_text =
    //     header.to_string() + "\n\n" +
    //     &precheck + "\n\n" +
    //     &response_text;

    // // Post the PR Comment
    // let comment = octocrab
    //     .issues(OWNER, REPO)
    //     .create_comment(pr_id, comment_text)
    //     .await?;
    // info!("PR Comment: {:#?}", comment);       
    // info!("{:#?}", pr.url);

    // // Wait 1 minute
    // std::thread::sleep(
    //     std::time::Duration::from_secs(60)
    // );

    // Return OK
    Ok(())
}

/// Return the Reaction IDs for Rocket and Eyes Reactions, created by the Bot
async fn get_reactions(octocrab: &Octocrab, pr_id: u64) -> 
    Result<(Option<u64>, Option<u64>), Box<dyn std::error::Error>> {
    // Fetch the PR Reactions
    let reactions = octocrab
        .issues(OWNER, REPO)
        .list_reactions(pr_id)
        .per_page(100)
        .page(0u32)
        .send()
        .await?;
    let reactions = reactions.items;

    // Watch for Rocket and Eyes Reactions created by the Bot
    let mut result: (Option<u64>, Option<u64>) = (None, None);
    for reaction in reactions.iter() {
        let content = &reaction.content;
        let user = &reaction.user.login;
        let reaction_id = &reaction.id.0;
        if user == "nuttxpr" {
            // info!("user: {:#?}", user);
            // info!("content: {:#?}", content);
            // info!("reaction_id: {:#?}", reaction_id);    
            match content {
                ReactionContent::Rocket => { result.0 = Some(*reaction_id) }
                ReactionContent::Eyes   => { result.1 = Some(*reaction_id) }
                _ => {}
            }
        }
    }
    info!("result: {:#?}", result);    
    Ok(result)
}

/// Bump up the 2 PR Reactions: 00 > 01 > 10 > 11
async fn bump_reactions(octocrab: &Octocrab, pr_id: u64, reactions: (Option<u64>, Option<u64>)) -> 
    Result<(), Box<dyn std::error::Error>> {
    // TODO
    Ok(())
}

/// Delete the PR Reactions
async fn delete_reactions(octocrab: &Octocrab, pr_id: u64, reactions: (Option<u64>, Option<u64>)) -> 
    Result<(), Box<dyn std::error::Error>> {
    if let Some(reaction_id) = reactions.0 {
        delete_reaction(octocrab, pr_id, reaction_id).await?;
    }
    if let Some(reaction_id) = reactions.1 {
        delete_reaction(octocrab, pr_id, reaction_id).await?;
    }
    Ok(())
}

/// Create the PR Reaction
async fn create_reaction(octocrab: &Octocrab, pr_id: u64, content: ReactionContent) -> 
    Result<(), Box<dyn std::error::Error>> {
    octocrab
        .issues(OWNER, REPO)
        .create_reaction(pr_id, content)
        .await?;
    Ok(())
}

/// Delete the PR Reaction
async fn delete_reaction(octocrab: &Octocrab, pr_id: u64, reaction_id: u64) -> 
    Result<(), Box<dyn std::error::Error>> {
    octocrab
        .issues(OWNER, REPO)
        .delete_reaction(pr_id, reaction_id)
        .await?;
    Ok(())
}
