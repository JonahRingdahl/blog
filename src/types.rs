use gloo_net::Error;
use gloo_net::http::{Request, Response};
use log::info;
use serde::{Deserialize, Serialize};

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
    pub async fn get_articles() -> Vec<Post> {
        let res = Request::get("posts.json").send().await.expect("test");

        res.json().await.unwrap()
    }
}
