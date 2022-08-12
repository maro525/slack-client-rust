use anyhow::Result;
use serde::{Deserialize, Serialize};

use bson::DateTime;

use crate::service::slack;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    id: String,
    user_id: String,
    user_name: String,
    channel_name: String,
    text: String,
    permalink: String,
    created_at: bson::DateTime,
}

impl From<&slack::search_messages::Message> for Message {
    fn from(m: &slack::search_messages::Message) -> Self {
        let sec = m.ts.split('.').collect::<Vec<_>>()[0];
        let sec = sec.parse::<i64>().unwrap();

        Message {
            id: m.iid.clone(),
            user_id: m.user.clone(),
            user_name: m.username.clone(),
            channel_name: m.channel.name.clone(),
            text: m.text.clone(),
            permalink: m.permalink.clone(),
            created_at: DateTime::now(),
        }
    }
}

pub async fn exec(query: String) -> Result<Response> {
    // let token = std::env::var("SLACK_USER_TOKEN").expect("SLACK_USER_TOKEN is not set.");

    let token: &str = "xoxp-8734147719-171770099568-3873404412051-87d3225ac7b4ddf98165c96027dad5ad";

    println!("query {}", query);

    let res = slack::SlackClient::new(token)
        .search_message(query.as_str(), "timestamp")
        .await?;

    println!("res {:?}", res);

    let messages = res
        .messages
        .matches
        .iter()
        .map(|m| m.into())
        .collect::<Vec<_>>();

    println!("messages {:?}", messages);
    Ok(Response { messages })
}
