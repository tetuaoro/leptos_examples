mod api;
pub mod app;
pub mod errors;
#[cfg(feature = "ssr")]
pub mod middleware;
#[cfg(feature = "ssr")]
pub mod proutes;
#[cfg(feature = "ssr")]
pub mod surreal;
#[cfg(feature = "ssr")]
pub mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App)
}
