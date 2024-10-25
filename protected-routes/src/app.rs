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
        <Title text="Welcome to Protected Routes Example"/>
        <Stylesheet id="leptos" href="/pkg/protected-routes.css"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=error_fallback>
                    <Route path=path!("") view=HomePage/>
                    <Route path=path!("dashboard") view=DashboardPage/>
                    <SecretProtectedRoute/>
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
        <p>"Because the example is "<i>"hydrated"</i>", the link below is an "<code>"<form/>"</code>" element instead of "<code>"<A/>"</code>" component to match the server layer."</p>
        <form action="/dashboard"><button type="submit" name="uselayer" value="true">"Goto dashboard page"</button></form>
        <ActionForm action=new_session>
            <button type="submit">"Log in"</button>
        </ActionForm>
    }
}

#[component]
fn LogoutBtn() -> impl IntoView {
    let delete_session = ServerAction::<DeleteSession>::new();

    view! {
        <ActionForm action=delete_session>
            <button type="submit">"Log out"</button>
        </ActionForm>
    }
}

#[component]
fn DashboardPage() -> impl IntoView {
    view! {
        <h1>"You match the dashboard page using layer !"</h1>
        <form action="/secret"><button type="submit" name="usecomponent" value="true">"Show me the secret now (form)"</button></form>
        <A href="/secret">"Show me the secret (A)"</A>
        <LogoutBtn/>
    }
}

#[component(transparent)]
fn SecretProtectedRoute() -> impl MatchNestedRoutes + Clone {
    let resource = Resource::new(|| (), move |_| async { exist_session().await.ok() });
    let is_auth = move || resource.get().flatten();

    view! {
        <ProtectedRoute path=path!("/secret") view=SecretPage condition=is_auth redirect_path=|| "/no/exist/path" ssr=leptos_router::SsrMode::Async/>
    }
    .into_inner()
}

#[component]
fn SecretPage() -> impl IntoView {
    view! {
        <h1>"You match the secret page using <ProtectedRoute/> component !"</h1>
        <LogoutBtn/>
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
