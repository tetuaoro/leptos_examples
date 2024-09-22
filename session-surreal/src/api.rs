use leptos::*;

#[server(endpoint = "exist")]
pub async fn exist_session() -> Result<bool, ServerFnError> {
    use crate::config::SESSION_TOKEN_KEY;
    use crate::surreal_database::Token;
    use leptos_axum::extract;
    use tower_sessions::Session;

    let session = extract::<Session>().await?;

    let token = session.get::<Token>(SESSION_TOKEN_KEY).await?;

    Ok(Option::is_some(&token)) // also check if token is valide
}

#[server(endpoint = "login")]
pub async fn new_session() -> Result<(), ServerFnError> {
    use crate::config::SESSION_TOKEN_KEY;
    use crate::surreal_database::DatabaseState;
    use leptos_axum::extract;
    use tower_sessions::Session;

    let db = extract::<DatabaseState>().await?;
    let session = extract::<Session>().await?;

    let jwt = db.login().await?;
    session.insert(SESSION_TOKEN_KEY, jwt).await?;
    db.as_root_server().await?;

    leptos_axum::redirect("/dashboard");
    Ok(())
}

#[server(endpoint = "logout")]
pub async fn delete_session() -> Result<(), ServerFnError> {
    use crate::surreal_database::DatabaseState;
    use leptos_axum::extract;
    use tower_sessions::Session;

    let db = extract::<DatabaseState>().await?;
    let session = extract::<Session>().await?;

    db.logout().await?;
    session.clear().await;
    // session.delete().await?;
    db.as_root_server().await?;

    leptos_axum::redirect("/");
    Ok(())
}
