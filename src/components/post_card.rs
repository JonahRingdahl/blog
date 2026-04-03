use leptos::prelude::*;

use crate::types::MetaPost;

#[component]
pub fn PostCard(meta: MetaPost) -> impl IntoView {
    let date = meta.date.clone();
    let summary = meta.summary.clone();

    view! {
        <article class="post-card">
            <span class="post-date">{date}</span>
            <h2 class="post-title">{meta.title}</h2>
            <p class="post-summary">{summary}</p>
        </article>
    }
}
