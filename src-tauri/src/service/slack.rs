pub mod search_messages;

use crate::service::slack::search_messages::Response;

use anyhow::Error;

pub struct SlackClient {
    token: String,
}

impl SlackClient {
    pub fn new(token: &str) -> SlackClient {
        SlackClient {
            token: token.into(),
        }
    }

    pub async fn search_message(&self, query: &str, sort: &str) -> Result<Response, Error> {
        println!("sort {}", sort);
        let client = reqwest::Client::new();

        Ok(client
            .get("https://slack.com/api/search.messages")
            .query(&[("query", query), ("sort", sort), ("pretty", "1")])
            .bearer_auth(&self.token)
            .send()
            .await?
            .json::<Response>()
            .await?)
    }
}
