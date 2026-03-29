use leptos::prelude::*;
use log::info;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    info!("loading Home");
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">

                <h1>"Welcome to my website"</h1>

                <a href="/articles"> Go to articles </a>

            </div>
        </ErrorBoundary>
    }
}
