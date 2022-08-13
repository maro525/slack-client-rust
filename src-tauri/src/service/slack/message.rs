use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub ok: bool,
    pub messages: Messages,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Messages {
    pub total: i32,
    pagination: Pagination,
    pub matches: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pagination {
    total_count: i32,
    page: i32,
    per_page: i32,
    page_count: i32,
    first: i32,
    last: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub iid: String,
    pub score: f64,
    pub user: Option<String>,
    pub username: String,
    pub ts: String,
    pub channel: Channel,
    pub text: String,
    pub permalink: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: String,
}
