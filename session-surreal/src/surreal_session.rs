use crate::errors::AppError;
use crate::surreal_database as surreal;
use axum::Extension;
use std::sync::Arc;
use surrealdb::engine::remote::ws::Client;
use tower::{
    layer::util::{Identity, Stack},
    ServiceBuilder,
};
use tower_sessions::{cookie::time::Duration, ExpiredDeletion, Expiry, SessionManagerLayer};
use tower_sessions_surrealdb_store::SurrealSessionStore;

pub type SessionService =
    ServiceBuilder<Stack<SessionManagerLayer<SurrealSessionStore<Client>>, Identity>>;

pub async fn handler() -> Result<SessionService, AppError> {
    let session_store = SurrealSessionStore::new(surreal::DB.clone(), "sessions".to_string());
    let expired_session_cleanup_interval = 3600;
    tokio::task::spawn(session_store.clone().continuously_delete_expired(
        tokio::time::Duration::from_secs(expired_session_cleanup_interval),
    ));
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::minutes(10)));
    let session_service = ServiceBuilder::new().layer(session_layer);
    Ok(session_service)
}

pub type DatabaseService = ServiceBuilder<Stack<Extension<surreal::DatabaseState>, Identity>>;

pub async fn database() -> Result<DatabaseService, AppError> {
    let db = surreal::DB.clone();
    let state = surreal::DatabaseState::new(Arc::new(db));
    let extension = Extension(state);
    let database_service = ServiceBuilder::new().layer(extension);
    Ok(database_service)
}
