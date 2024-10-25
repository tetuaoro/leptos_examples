use crate::utils::config::SESSION_TOKEN_KEY;
use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use leptos::logging as console;
use std::sync::LazyLock as Lazy;
use surrealdb::opt::auth::Jwt;
use tower_sessions::Session;

static PROTECTED_ROUTES: Lazy<Vec<&str>> = Lazy::new(|| vec!["/dashboard"]);

pub async fn handler(session: Session, req: Request<Body>, next: Next) -> Response {
    console::log!("hit handler proutes");

    let uri = req.uri().path();
    let is_auth = matches!(session.get::<Jwt>(SESSION_TOKEN_KEY).await, Ok(Some(_)));

    let is_protected = PROTECTED_ROUTES.iter().any(|r| uri.starts_with(r));
    if is_protected && !is_auth {
        // the `/login` path does not exist, `NonePage` will trigger
        return Redirect::temporary("/login").into_response();
    }

    next.run(req).await
}
