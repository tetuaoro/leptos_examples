use crate::api::*;
use crate::errors::{AppError, ErrorTemplate};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path, MatchNestedRoutes};

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
        <Stylesheet id="leptos" href="/pkg/protected-routes.css"/>

        // sets the document title
        <Title text="Welcome to Protected Routes Example"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=error_fallback>
                    <Route path=path!("") view=HomePage/>
                    <Route path=path!("dashboard") view=DashboardPage/>
                    <SecretProtectedRoute/>
                    <Route path=path!("*any") view=NonePage/>
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
        <h1>"Hi, hit button to create a new session !"</h1>
        <a href="/dashboard">"Goto dashboard page"</a>
        <ActionForm action=new_session>
            <button type="submit">
                "Log in"
            </button>
        </ActionForm>
    }
}

#[component]
fn DashboardPage() -> impl IntoView {
    view! {
        <h1>"You match the dashboard page using layer !"</h1>
        <span>"Goto secret page"</span>
    }
}

#[component(transparent)]
fn SecretProtectedRoute() -> impl MatchNestedRoutes + Clone {
    let resource = Resource::new(|| (), move |_| async { exist_session().await.ok() });
    let cond = move || resource.get().flatten();

    view! {
        <ProtectedRoute path=path!("secret") condition=cond redirect_path=|| "/wrong/session" view=Outlet/>
    }
    .into_inner()
}

#[component]
fn SecretPage() -> impl IntoView {
    view! {
        <h1>"You match the secret page using <ProtectedRoute/> component !"</h1>
    }
}

#[component]
fn NonePage() -> impl IntoView {
    view! {
        <h2>"You are on the wrong path !"</h2>
        <a href="/">"Goto home page"</a>
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
