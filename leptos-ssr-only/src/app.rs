use leptos::prelude::*;

#[server(endpoint = "hey")]
pub async fn hey() -> Result<String, ServerFnError> {
    Ok("hey salut".into())
}
