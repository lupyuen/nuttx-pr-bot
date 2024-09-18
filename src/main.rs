// Every Minute: Fetch PR Number x to x+10
//   If PR Status = Open
//   And PR Comments don't include Bot
//   Then call Gemini API
//   Post Gemini Response as PR Comment

// Header for PR Comment: [**\[Experimental Bot, please feedback here\]**](https://github.com/apache/nuttx/pull/13494#issuecomment-2357300091)

use log::info;
use std::env;

use google_generative_ai_rs::v1::{
    api::Client,
    gemini::{request::Request, Content, Model, Part, Role},
};

/// Simple text request using the public API and an API key for authn
/// To run:
/// ```
/// GEMINI_API_KEY=[YOUR_API_KEY] RUST_LOG=info cargo run
/// ``
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init the Logger
    env_logger::init();

    // Fetch the Pull Request
    let owner = "lupyuen2";
    let repo = "wip-nuttx";
    let pr_id = 76;
    let pr = octocrab::instance().pulls(owner, repo).get(pr_id).await?;
    info!("{:#?}", pr);

    // Get the PR Body
    let body = pr.body.unwrap();
    info!("{:#?}", body);

    // // Init the Gemini Client
    // let client = Client::new_from_model(
    //     // Model::Gemini1_5Pro,  // For Production
    //     Model::GeminiPro,  // For Testing
    //     env::var("GEMINI_API_KEY").unwrap().to_string()
    // );

    // // Input for the request
    // let input = "# Here are the requirements for a NuttX PR\n\n## Summary\n\n* Why change is necessary (fix, update, new feature)?\n* What functional part of the code is being changed?\n* How does the change exactly work (what will change and how)?\n* Related [NuttX Issue](https://github.com/apache/nuttx/issues) reference if applicable.\n* Related NuttX Apps [Issue](https://github.com/apache/nuttx-apps/issues) / [Pull Request](https://github.com/apache/nuttx-apps/pulls) reference if applicable.\n\n## Impact\n\n* Is new feature added? Is existing feature changed?\n* Impact on user (will user need to adapt to change)? NO / YES (please describe if yes).\n* Impact on build (will build process change)? NO / YES (please descibe if yes).\n* Impact on hardware (will arch(s) / board(s) / driver(s) change)? NO / YES (please describe if yes).\n* Impact on documentation (is update required / provided)? NO / YES (please describe if yes).\n* Impact on security (any sort of implications)? NO / YES (please describe if yes).\n* Impact on compatibility (backward/forward/interoperability)? NO / YES (please describe if yes).\n* Anything else to consider?\n\n## Testing\n\nI confirm that changes are verified on local setup and works as intended:\n* Build Host(s): OS (Linux,BSD,macOS,Windows,..), CPU(Intel,AMD,ARM), compiler(GCC,CLANG,version), etc.\n* Target(s): arch(sim,RISC-V,ARM,..), board:config, etc.\n\nTesting logs before change:\n\n```\nyour testing logs here\n```\n\nTesting logs after change:\n```\nyour testing logs here\n```\n\n# Does this PR meet the NuttX Requirements?\n\n## Summary\nBCH: Add readonly configuration for BCH devices\n## Impact\nNONE\n## Testing\n";

    // // Compose the request
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

    // // Send the request
    // let response = client.post(30, &txt_request).await?;
    // info!("{:#?}", response);

    // // Get the response
    // let text = response.rest().unwrap()
    //     .candidates.first().unwrap()
    //     .content.parts.first().unwrap()
    //     .text.clone().unwrap();
    // info!("{:#?}", text);

    // Return OK
    Ok(())
}
