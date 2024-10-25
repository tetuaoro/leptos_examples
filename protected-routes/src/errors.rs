use http::status::StatusCode;
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_navigate};
use std::env::VarError;
#[cfg(feature = "ssr")]
use std::sync::PoisonError;
#[cfg(feature = "ssr")]
use surrealdb::Error as SurrealError;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
    #[error("Env missing")]
    EnvMissing(String),
    #[error("Database failure")]
    Database(String),
    #[error("Something wrong")]
    Poison(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::BAD_REQUEST,
        }
    }
}

pub type AppResult<T> = std::result::Result<T, AppError>;

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
#[component]
pub fn ErrorTemplate(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => RwSignal::new(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };
    // Get Errors from Signal
    let errors = errors.get_untracked();

    // Downcast lets us take a type that implements `std::error::Error`
    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
        .collect();
    println!("Errors: {errors:#?}");

    // Only the response code for the first error is actually sent from the server
    // this may be customized by the specific application
    #[cfg(feature = "ssr")]
    {
        use leptos_axum::ResponseOptions;
        let response = use_context::<ResponseOptions>();
        if let Some(response) = response {
            response.set_status(errors[0].status_code());
        }
    }

    let navigate = use_navigate();
    let back_to_home = move || navigate("/", Default::default());
    let cb_timeout_effect =
        move || set_timeout(back_to_home.clone(), std::time::Duration::from_millis(7259));

    Effect::new(cb_timeout_effect);

    view! {
        <h1>{if errors.len() > 1 { "Errors" } else { "Error" }}</h1>
        <p>"You will automatically redirect to home page (7 secs) !"</p>
        <A href="/">"Goto home page"</A>
        <For
            // a function that returns the items we're iterating over; a signal is fine
            each=move || { errors.clone().into_iter().enumerate() }
            // a unique key for each item as a reference
            key=|(index, _error)| *index
            // renders each item to a view
            children=move |error| {
                let error_string = error.1.to_string();
                let error_code = error.1.status_code();
                view! {
                    <h2>{error_code.to_string()}</h2>
                    <p>"Error: " {error_string}</p>
                }
            }
        />
    }
}

impl From<VarError> for AppError {
    fn from(error: VarError) -> Self {
        Self::EnvMissing(error.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<SurrealError> for AppError {
    fn from(error: SurrealError) -> Self {
        Self::Database(error.to_string())
    }
}

#[cfg(feature = "ssr")]
impl<T> From<PoisonError<T>> for AppError {
    fn from(error: PoisonError<T>) -> Self {
        Self::Poison(error.to_string())
    }
}
