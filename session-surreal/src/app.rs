use crate::{
    api::{exist_session, NewSession},
    errors::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/session-surreal.css"/>

        // sets the document title
        <Title text="Welcome to Session Surreal Example"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/dashboard" view=DashboardPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let new_session = create_server_action::<NewSession>();

    view! {
        <h1>"Hi, hit button to create a new session !"</h1>
        <ActionForm id="new-session-form" action=new_session>
            <button form="new-session-form" type="submit">
                "New Session"
            </button>
        </ActionForm>
    }
}

#[component]
fn DashboardPage() -> impl IntoView {
    let resource = create_resource(|| (), move |_| async { exist_session().await.ok() });

    let exist_session = move || {
        if let Some(Some(exist)) = resource.get() {
            return exist;
        }
        false
    };

    view! {
        <Suspense>
            <Show when=exist_session fallback=RedirectToHomePage>
                <p>"A session exist !"</p>
            </Show>
        </Suspense>
    }
}

#[component]
fn RedirectToHomePage() -> impl IntoView {
    view! {
        <p>"Please log in !"</p>
        <a href="/">"Log in"</a>
    }
}
