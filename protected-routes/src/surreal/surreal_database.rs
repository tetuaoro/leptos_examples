use crate::errors::*;
use crate::utils::config::*;
use axum::{async_trait, extract::FromRequestParts, Extension};
use http::{request::Parts, StatusCode};
use leptos::logging;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock as Lazy;
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::{Jwt, Root},
    Connection, Surreal,
};
use tower::{
    layer::util::{Identity, Stack},
    ServiceBuilder,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Token(String);

impl Token {
    pub const KEY: &'static str = "token";
}

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
static ROOT_TOKEN: Lazy<Mutex<Jwt>> = Lazy::new(|| Mutex::new(Jwt::from(String::new())));

/// Define an asynchronous function to initiate the database instance
pub async fn initiate() -> AppResult<()> {
    let endpoint = SURREAL_BIND()?;
    let namespace = SURREAL_NS()?;
    let database = SURREAL_DB()?;
    let username = SURREAL_USER()?;
    let password = SURREAL_PASS()?;

    DB.connect::<Ws>(endpoint).await?;
    let token = DB
        .signin(Root {
            username: &username,
            password: &password,
        })
        .await?;
    DB.use_ns(&namespace).use_db(&database).await?;

    let mut root_token = ROOT_TOKEN.lock()?;
    *root_token = token;
    logging::debug_warn!("ROOT TOKEN SAVED DONE");

    logging::debug_warn!("DB INITIALIZE DONE");
    Ok(())
}

#[async_trait]
pub trait DatabaseProvider: Send + Sync {
    async fn login(&self) -> AppResult<Token>;
    async fn logout(&self) -> AppResult<()>;
    async fn as_root_server(&self) -> AppResult<()>;
}

#[derive(Clone)]
pub struct DatabaseState {
    db: Arc<dyn DatabaseProvider>,
}

impl DatabaseState {
    pub fn new(db: Arc<dyn DatabaseProvider>) -> Self {
        DatabaseState { db }
    }

    pub fn get_database(&self) -> Arc<dyn DatabaseProvider> {
        self.db.clone()
    }
}

impl Deref for DatabaseState {
    type Target = dyn DatabaseProvider;
    fn deref(&self) -> &Self::Target {
        &*self.db
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseState
where
    S: Sync + Send,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<DatabaseState>().cloned().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Can't extract database. Is `DatabaseStateLayer` enabled?",
        ))
    }
}

#[async_trait]
impl<C: Connection> DatabaseProvider for Surreal<C> {
    async fn login(&self) -> AppResult<Token> {
        let username = SURREAL_USER()?;
        let password = SURREAL_PASS()?;

        let token = self
            .signin(Root {
                username: &username,
                password: &password,
            })
            .await?;

        Ok(Token(token.into_insecure_token()))
    }

    async fn logout(&self) -> AppResult<()> {
        self.invalidate().await?;
        Ok(())
    }

    async fn as_root_server(&self) -> AppResult<()> {
        let token = ROOT_TOKEN.lock()?.clone();
        self.authenticate(token).await?;
        Ok(())
    }
}

pub type DatabaseService = ServiceBuilder<Stack<Extension<DatabaseState>, Identity>>;

pub async fn database() -> Result<Extension<DatabaseState>, AppError> {
    let db = DB.clone();
    let state = DatabaseState::new(Arc::new(db));
    let extension = Extension(state);
    Ok(extension)
}
