use leptos::component;
use leptos::prelude::*;

use crate::components::post_card::PostCard;
use crate::types::Post;

#[component]
pub fn Articles() -> impl IntoView {
    let (posts, set_posts) = signal(Vec::<Post>::new());
    let (selected_post, set_selected_post) = signal(Option::<Post>::None);

    wasm_bindgen_futures::spawn_local(async move {
        let articles = Post::get_articles().await;
        set_posts.set(articles);
    });

    view! {
        <div class="articles-layout">
            <div class="posts-list">
                {move || posts.get().iter().cloned().map(|post| {
                    let post_clone = post.clone();
                    view! {
                        <div class="post-card-wrapper" on:click=move |_| set_selected_post.set(Some(post_clone.clone()))>
                            <PostCard post_meta=post.post_meta />
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
            <div class="article-content">
                {move || selected_post.get().map(|post| {
                    view! { <div inner_html=post.html /> }
                })}
            </div>
        </div>
    }
}
