use serde_json::json;
use std::{env, fs, process};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let slack_webhook = env::var("INPUT_SLACK_WEBHOOK").expect("SLACK_WEBHOOK env var is required");
    let slack_heading = env::var("INPUT_HEADING").expect("HEADING env var is required");
    let slack_message = env::var("INPUT_MESSAGE").expect("MESSAGE env var is required");
    let slack_template = env::var("INPUT_TEMPLATE_TYPE"); // detailed, file, custom
    let slack_template = slack_template
        .as_ref()
        .map(String::as_str)
        .unwrap_or("simple");
    let slack_template_custom: String = env::var("INPUT_TEMPLATE_CUSTOM").unwrap(); // if template type file or custom
    let github_repository = env::var("GITHUB_REPOSITORY").unwrap();
    let github_event_name = env::var("GITHUB_EVENT_NAME").unwrap();
    let github_ref = env::var("GITHUB_REF").unwrap();
    let github_sha = env::var("GITHUB_SHA").unwrap();

    println!("::debug ::Sending a request to slack");
    // ${{ github.event.head_commit.message }}

    let obj = match slack_template {
        "detailed" => json!({
            "blocks": [
                {
                    "type": "header",
                    "text": {
                        "type": "plain_text",
                        "text": format!("*{}*",slack_heading),
                        "emoji": true
                    }
                },
                {
                    "type": "section",
                    "text": {
                        "type": "mrkdwn",
                        "text": format!("*{}*",slack_message),
                    }
                },
                {
                    "type": "section",
                    "fields": [
                        {
                            "type": "mrkdwn",
                            "text": format!("*Repository:*\n{}", github_repository),
                        },
                        {
                            "type": "mrkdwn",
                            "text": format!("*Event:*\n{}", github_event_name),
                        },
                        {
                            "type": "mrkdwn",
                            "text": format!("*Ref:*\n{}", github_ref),
                        },
                        {
                            "type": "mrkdwn",
                            "text": format!("*SHA:*\n{}", github_sha),
                        },
                    ]
                }
            ]
        }),
        "custom" => json!(slack_template_custom),
        "file" => json!(fs::read_to_string(slack_template_custom).ok()),
        _ => json!({
            "blocks": [
                {
                    "type": "header",
                    "text": {
                        "type": "plain_text",
                        "text": format!("*{}*",slack_heading),
                        "emoji": true
                    }
                },
                {
                    "type": "section",
                    "text": {
                        "type": "mrkdwn",
                        "text": format!("*{}*",slack_message),
                    }
                },
            ]
        }),
    };

    let resp: String = reqwest::Client::new()
        .post(slack_webhook)
        .json(&obj)
        .send()
        .await?
        .text()
        .await?;

    if resp != "ok" {
        println!("::error ::{}", resp);
        process::exit(1);
    }
    Ok(())
}
