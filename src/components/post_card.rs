use leptos::prelude::*;

use crate::types::MetaPost;

#[component]
pub fn PostCard(meta: MetaPost) -> impl IntoView {
    let href = format!("/post/{}", meta.slug);
    let date = meta.date;
    let title = meta.title;
    let summary = meta.summary;

    view! {
        <article>
            <span>{date}</span>
            <h2>
                <a href=href>{title}</a>
            </h2>
            <p>{summary}</p>
        </article>
    }
}
