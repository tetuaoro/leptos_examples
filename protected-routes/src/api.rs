use leptos::prelude::*;

type Result<T> = std::result::Result<T, ServerFnError>;

#[server(endpoint = "exist")]
pub async fn exist_session() -> Result<bool> {
    use crate::surreal::Token;
    use crate::utils::config::SESSION_TOKEN_KEY;
    use leptos_axum::extract;
    use tower_sessions::Session;

    leptos::logging::log!("hit exist_session server fn");

    let session = extract::<Session>().await?;

    let token = session.get::<Token>(SESSION_TOKEN_KEY).await?;

    Ok(Option::is_some(&token)) // also check if token is valide
}

#[server(endpoint = "login")]
pub async fn new_session() -> Result<()> {
    use crate::surreal::DatabaseState;
    use crate::utils::config::SESSION_TOKEN_KEY;
    use leptos_axum::extract;
    use tower_sessions::Session;

    let db = extract::<DatabaseState>().await?;
    let session = extract::<Session>().await?;

    let token = db.login().await?;
    session.insert(SESSION_TOKEN_KEY, token).await?;
    db.as_root_server().await?;

    leptos_axum::redirect("/dashboard");
    Ok(())
}

#[server(endpoint = "logout")]
pub async fn delete_session() -> Result<()> {
    use crate::surreal::DatabaseState;
    use leptos_axum::extract;
    use tower_sessions::Session;

    let db = extract::<DatabaseState>().await?;
    let session = extract::<Session>().await?;

    db.logout().await?;
    session.clear().await; // or session.delete().await?; to also delete the token in database
    db.as_root_server().await?;

    leptos_axum::redirect("/");
    Ok(())
}
