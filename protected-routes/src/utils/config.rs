#![allow(dead_code)]

use crate::errors::{AppError, AppResult};
use std::env::var;

type EnvResult = AppResult<String>;

pub(crate) const SURREAL_NS: fn() -> EnvResult = || var("SURREAL_NS").map_err(AppError::from);
pub(crate) const SURREAL_DB: fn() -> EnvResult = || var("SURREAL_DB").map_err(AppError::from);
pub(crate) const SURREAL_BIND: fn() -> EnvResult = || var("SURREAL_BIND").map_err(AppError::from);
pub(crate) const SURREAL_USER: fn() -> EnvResult = || var("SURREAL_USER").map_err(AppError::from);
pub(crate) const SURREAL_PASS: fn() -> EnvResult = || var("SURREAL_PASS").map_err(AppError::from);
pub(crate) const SESSION_TOKEN_KEY: &str = "token";
