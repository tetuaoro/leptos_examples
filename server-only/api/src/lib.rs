use leptos::{prelude::*, server_fn::codec::GetUrl};

#[server(endpoint = "name")]
pub async fn get_name() -> Result<String, ServerFnError> {
    use_context::<LeptosOptions>()
        .ok_or_else(|| {
            use axum::http::StatusCode;
            use leptos_axum::ResponseOptions;

            if let Some(response) = use_context::<ResponseOptions>() {
                response.set_status(StatusCode::SERVICE_UNAVAILABLE);
            }

            ServerFnError::new("could't get core options")
        })
        .map(|opts| {
            let name = opts.output_name.to_string();
            format!("{}51615101_", name)
        })
}

#[server(input = GetUrl, prefix = "/mirage/v1", endpoint = "number")]
pub async fn get_index() -> Result<i64, ServerFnError> {
    Ok(29)
}
