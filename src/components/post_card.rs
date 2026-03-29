use leptos::prelude::*;

use crate::types::MetaPost;

#[component]
pub fn PostCard(post_meta: MetaPost) -> impl IntoView {
    let date = post_meta.date.clone();
    let summary = post_meta.summary.clone();

    view! {
        <article class="post-card">
            <span class="post-date">{date}</span>
            <h2 class="post-title">{post_meta.title}</h2>
            <p class="post-summary">{summary}</p>
        </article>
    }
}
