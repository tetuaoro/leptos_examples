mod api;
pub mod app;
mod config;
pub mod errors;
#[cfg(feature = "ssr")]
pub mod fileserv;
#[cfg(feature = "ssr")]
pub mod surreal_database;
#[cfg(feature = "ssr")]
pub mod surreal_session;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
