use gloo_net::http::Request;
use log::info;
use serde::{Deserialize, Serialize};
use std::io::Result;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MetaPost {
    pub title: String,
    pub date: String,
    pub summary: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Post {
    pub meta: MetaPost,
    pub html: String,
}

impl Post {
    pub async fn get_articles() -> Result<Vec<Post>> {
        Request::get("posts.json")
                    .send()
                    .await
                    .json()
                    .await
    }
}
