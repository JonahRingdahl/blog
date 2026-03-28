use serde::{Deserialize, Serialize};
use std::fs;
use log::{info, warn};
use gloo_net::http::Request;
use serde_json::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MetaPost {
    pub title: String,
    pub date: String,
    pub summary: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Post {
    pub post_meta: MetaPost,
    pub html: String,
}

impl Post {
    pub async fn get_articles() -> Vec<Post> {
    }
}
