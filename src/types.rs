use gloo_net::http::Request;
use log::info;
use serde::{Deserialize, Serialize};

use leptos::prelude::*;

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
        info!("Loading Posts");
        let url = format!("{}/posts.json", leptos::config::base_url());
        let response = Request::get(url)
            .send()
            .await
            .expect("Failed to fetch posts");

        response.json().await.expect("Failed to parse JSON")
    }
}
