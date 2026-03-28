use leptos::prelude::*;

use crate::types::MetaPost;

#[component]
pub fn PostCard(post_meta: MetaPost) -> impl IntoView {
    let href = format!("/post/{}", post_meta.title);
    let date = post_meta.date;
    let summary = post_meta.summary;

    view! {
        <article>
            <span>{date}</span>
            <h2>
                <a href=href>{post_meta.title}</a>
            </h2>
            <p>{summary}</p>
        </article>
    }
}
