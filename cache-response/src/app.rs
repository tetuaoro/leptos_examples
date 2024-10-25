use crate::error_template::{AppError, ErrorTemplate};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let error_fallback = || {
        let mut outside_errors = Errors::default();
        outside_errors.insert_with_default_key(AppError::NotFound);
        view! {
            <ErrorTemplate outside_errors/>
        }
        .into_view()
    };

    view! {
        <Title text="Welcome to Cache Response Example"/>
        <Stylesheet id="leptos" href="/pkg/cache-response.css"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=error_fallback>
                    <Route path=path!("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| count.update(|count| *count += 1);

    view! {
        <h1>"Cache response !"</h1>
        <p>"Reload the page and open your browser console to check the code status for js, css and wasm files !"</p>
        <p>"Expected to equal 206 and Cached-Control to equal 1 year."</p>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
