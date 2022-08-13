pub mod message;

use crate::service::slack::message::Response;

use anyhow::{Error, Result};

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
        let client = reqwest::Client::new();

        let response = client
            .get("https://slack.com/api/search.messages")
            .query(&[("query", query), ("sort", sort), ("pretty", "1")])
            .bearer_auth(&self.token)
            .send()
            .await?;

        let json = response.json::<Response>().await.unwrap();

        Ok(json)
    }
}
