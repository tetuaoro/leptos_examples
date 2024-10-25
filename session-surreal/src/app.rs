use crate::api::*;
use crate::errors::{AppError, ErrorTemplate};
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
        view! { <ErrorTemplate outside_errors/> }.into_view()
    };

    view! {
        <Title text="Welcome to Session Surreal Example"/>
        <Stylesheet id="leptos" href="/pkg/session-surreal.css"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=error_fallback>
                    <Route path=path!("") view=HomePage/>
                    <Route path=path!("/dashboard") view=DashboardPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let new_session = ServerAction::<NewSession>::new();

    view! {
        <h1>"Hi, hit the button « Log in » to create a new session !"</h1>
        <A href="/dashboard">"Goto dashboard page"</A>
        <ActionForm action=new_session>
            <button type="submit">
                "Log in"
            </button>
        </ActionForm>
    }
}

#[component]
fn DashboardPage() -> impl IntoView {
    let delete_session = ServerAction::<DeleteSession>::new();
    let resource = Resource::new(|| (), move |_| async { exist_session().await.ok() });

    let exist_session = move || {
        if let Some(Some(exist)) = resource.get() {
            return exist;
        }
        false
    };

    view! {
        <Suspense>
            <Show when=exist_session fallback=RedirectToHomePage>
                <h1>"A session exist !"</h1>
                <p>"Open your browser console to check the cookie token."</p>
                <ActionForm action=delete_session>
                    <button type="submit">
                        "Log out"
                    </button>
                </ActionForm>
            </Show>
        </Suspense>
    }
}

#[component]
fn RedirectToHomePage() -> impl IntoView {
    view! {
        <h2>"Please first log in !"</h2>
        <A href="/">"Goto home page"</A>
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
