use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::service::slack;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResponse {
    messages: Vec<SlackMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SlackMessage {
    user_name: String,
    channel_name: String,
    text: String,
}

impl From<&slack::message::Message> for SlackMessage {
    fn from(m: &slack::message::Message) -> Self {
        SlackMessage {
            user_name: m.username.clone(),
            channel_name: m.channel.name.clone(),
            text: m.text.clone(),
        }
    }
}

pub async fn exec(query: String) -> Result<CommandResponse> {
    // let token = std::env::var("SLACK_USER_TOKEN").expect("SLACK_USER_TOKEN is not set.");

    let token: &str = "xoxp-8734147719-171770099568-3936479728738-8946eb1d69b2f496dba0bafa09d56d70";

    let client = slack::SlackClient::new(token);

    let res = client.search_message(query.as_str(), "timestamp").await?;

    let messages = res.messages.matches;

    let messages = messages.iter().map(|m| m.into()).collect::<Vec<_>>();

    println!("{:?}", messages);

    Ok(CommandResponse { messages })
}
