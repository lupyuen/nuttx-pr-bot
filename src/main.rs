// Every Minute: Fetch New PRs
//   If PR Status = Open
//   And PR Comments don't include Bot
//     Then Call Gemini API
//     And Post Gemini Response as PR Comment

use std::env;
use log::info;
use octocrab::{models::IssueState, Octocrab};
use octocrab::params;

use google_generative_ai_rs::v1::{
    api::Client,
    gemini::{request::Request, Content, Model, Part, Role},
};

// For Production
const OWNER: &str = "apache";
const REPO: &str = "nuttx";

// For Testing
// const OWNER: &str = "lupyuen2";
// const REPO: &str = "wip-nuttx";

/// Simple text request using the public API and an API key for authn
/// To run:
/// ```
/// GEMINI_API_KEY=[YOUR_API_KEY] RUST_LOG=info cargo run
/// ``
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init the Logger
    env_logger::init();

    // Init the GitHub Client
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = octocrab::Octocrab::builder().personal_token(token).build()?;

    // Fetch the 10 Newest Pull Requests that are Open
    let pr_list = octocrab.pulls(OWNER, REPO).list()
        .state(params::State::Open)
        .sort(params::pulls::Sort::Created)
        .direction(params::Direction::Descending)
        .per_page(10)
        .send()
        .await?;
    // info!("{:#?}", pr_list);

    // Process every PR
    for pr in pr_list {
        info!("{:#?}", pr);
        let pr_id = pr.number;
        info!("{:#?}", pr_id);
        process_pr(&octocrab, pr_id).await?;

        // Wait 5 seconds
        std::thread::sleep(
            std::time::Duration::from_secs(5)
        );
    }

    // Return OK
    Ok(())
}

async fn process_pr(octocrab: &Octocrab, pr_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch the PR
    let pr = octocrab.pulls(OWNER, REPO)
        .get(pr_id).await?;
    info!("{:#?}", pr);
    info!("{:#?}", pr.url);

    // Skip if PR State is Not Open
    if pr.state.unwrap() != IssueState::Open {
        info!("Skipping Closed PR");
        return Ok(());
    }

    // Skip if PR contains Comments
    if pr.comments.unwrap() > 0 {
        info!("Skipping PR with comments");
        return Ok(());
    }
    
    // Get the PR Body
    let body = pr.body.unwrap();
    info!("{:#?}", body);

    // Init the Gemini Client
    let client = Client::new_from_model(
        Model::Gemini1_5Pro,  // For Production
        // Model::GeminiPro,  // For Testing
        env::var("GEMINI_API_KEY").unwrap().to_string()
    );

    // Input for the request
    // let input = "# Here are the requirements for a NuttX PR\n\n## Summary\n\n* Why change is necessary (fix, update, new feature)?\n* What functional part of the code is being changed?\n* How does the change exactly work (what will change and how)?\n* Related [NuttX Issue](https://github.com/apache/nuttx/issues) reference if applicable.\n* Related NuttX Apps [Issue](https://github.com/apache/nuttx-apps/issues) / [Pull Request](https://github.com/apache/nuttx-apps/pulls) reference if applicable.\n\n## Impact\n\n* Is new feature added? Is existing feature changed?\n* Impact on user (will user need to adapt to change)? NO / YES (please describe if yes).\n* Impact on build (will build process change)? NO / YES (please descibe if yes).\n* Impact on hardware (will arch(s) / board(s) / driver(s) change)? NO / YES (please describe if yes).\n* Impact on documentation (is update required / provided)? NO / YES (please describe if yes).\n* Impact on security (any sort of implications)? NO / YES (please describe if yes).\n* Impact on compatibility (backward/forward/interoperability)? NO / YES (please describe if yes).\n* Anything else to consider?\n\n## Testing\n\nI confirm that changes are verified on local setup and works as intended:\n* Build Host(s): OS (Linux,BSD,macOS,Windows,..), CPU(Intel,AMD,ARM), compiler(GCC,CLANG,version), etc.\n* Target(s): arch(sim,RISC-V,ARM,..), board:config, etc.\n\nTesting logs before change:\n\n```\nyour testing logs here\n```\n\nTesting logs after change:\n```\nyour testing logs here\n```\n\n# Does this PR meet the NuttX Requirements?\n\n## Summary\nBCH: Add readonly configuration for BCH devices\n## Impact\nNONE\n## Testing\n";
    let requirements = "# Here are the requirements for a NuttX PR\n\n## Summary\n\n* Why change is necessary (fix, update, new feature)?\n* What functional part of the code is being changed?\n* How does the change exactly work (what will change and how)?\n* Related [NuttX Issue](https://github.com/apache/nuttx/issues) reference if applicable.\n* Related NuttX Apps [Issue](https://github.com/apache/nuttx-apps/issues) / [Pull Request](https://github.com/apache/nuttx-apps/pulls) reference if applicable.\n\n## Impact\n\n* Is new feature added? Is existing feature changed?\n* Impact on user (will user need to adapt to change)? NO / YES (please describe if yes).\n* Impact on build (will build process change)? NO / YES (please descibe if yes).\n* Impact on hardware (will arch(s) / board(s) / driver(s) change)? NO / YES (please describe if yes).\n* Impact on documentation (is update required / provided)? NO / YES (please describe if yes).\n* Impact on security (any sort of implications)? NO / YES (please describe if yes).\n* Impact on compatibility (backward/forward/interoperability)? NO / YES (please describe if yes).\n* Anything else to consider?\n\n## Testing\n\nI confirm that changes are verified on local setup and works as intended:\n* Build Host(s): OS (Linux,BSD,macOS,Windows,..), CPU(Intel,AMD,ARM), compiler(GCC,CLANG,version), etc.\n* Target(s): arch(sim,RISC-V,ARM,..), board:config, etc.\n\nTesting logs before change:\n\n```\nyour testing logs here\n```\n\nTesting logs after change:\n```\nyour testing logs here\n```";
    let input = 
        requirements.to_string() +
        "\n\n# Does this PR meet the NuttX Requirements?\n\n" +
        &body;

    // Compose the request
    let txt_request = Request {
        contents: vec![Content {
            role: Role::User,
            parts: vec![Part {
                text: Some(input.to_string()),
                inline_data: None,
                file_data: None,
                video_metadata: None,
            }],
        }],
        tools: vec![],
        safety_settings: vec![],
        generation_config: None,
        system_instruction: None,
    };

    // Send the request
    let response = client.post(30, &txt_request).await?;
    info!("{:#?}", response);

    // Get the response
    let response_text = response.rest().unwrap()
        .candidates.first().unwrap()
        .content.parts.first().unwrap()
        .text.clone().unwrap();
    info!("{:#?}", response_text);

    // Header for PR Comment
    let header = "[**\\[Experimental Bot, please feedback here\\]**](https://github.com/search?q=repo%3Aapache%2Fnuttx+13494&type=pullrequests)";

    // Compose the PR Comment
    let comment_text = header.to_string() + "\n\n" + &response_text;

    // Post the PR Comment
    let comment = octocrab
        .issues(OWNER, REPO)
        .create_comment(pr_id, comment_text)
        .await?;
    info!("{:#?}", comment);       
    info!("{:#?}", pr.url);

    // Wait 1 minute
    std::thread::sleep(
        std::time::Duration::from_secs(60)
    );

    // Return OK
    Ok(())
}
