use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use server_fn::ServerFn;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/postman.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[server(endpoint = "one")]
pub async fn ep_one(body: String) -> Result<String, ServerFnError> {
    let response = format!("Response from endpoint one : {}", body);
    Ok(response)
}

#[server(endpoint = "two")]
pub async fn ep_two(body: String) -> Result<String, ServerFnError> {
    let response = format!("Response from endpoint two : {}", body);
    Ok(response)
}

#[component]
fn HomePage() -> impl IntoView {
    let endpoints = move || vec![EpOne::PATH, EpTwo::PATH];

    let (endpoint, set_endpoint) = create_signal(EpOne::PATH.to_string());
    let (body, set_body) = create_signal(String::new());
    let (response, set_response) = create_signal(String::new());

    let update_endpoint = move |evt: ev::Event| {
        let value = event_target_value(&evt);
        set_endpoint.set(value);
    };

    let update_body = move |evt: ev::Event| {
        let value = event_target_value(&evt);
        set_body.set(value);
    };

    let send_request = move |_| {
        let endpoint = endpoint.get();
        let body = body.get();
        spawn_local(async move {
            let res = if endpoint.eq(EpOne::PATH) {
                ep_one(body).await
            } else if endpoint.eq(EpTwo::PATH) {
                ep_two(body).await
            } else {
                Err(ServerFnError::MissingArg("Invalid endpoint".to_string()))
            };

            match res {
                Ok(data) => set_response.set(data),
                Err(e) => set_response.set(format!("Error: {:?}", e)),
            }
        });
    };

    view! {
        <div>
            <form id="postman"></form>
            <select id="endpoint" form="postman" name="endpoint" on:input=update_endpoint>
                <For each=endpoints key=|ep| ep.to_string() let:ep>
                    <option value=ep>{ep}</option>
                </For>
            </select>
            <textarea id="body" form="postman" name="body" on:input=update_body></textarea>
            <button on:click=send_request>"Send Request"</button>
            <div id="response" inner_html=response></div>
        </div>
    }
}
