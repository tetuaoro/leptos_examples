#[allow(unused_imports)]
use api::*;

use axum::{http::Method, routing::*, Router};
use leptos_axum::handle_server_fns_with_context;
use leptos_config::*;
use reactive_graph::owner::provide_context;

#[tokio::main]
async fn main() {
    // init logger
    simple_logger::SimpleLogger::new()
        .env()
        .init()
        .expect("couldn't initialize the logger");

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let state = leptos_options.clone();
    let cx_with_state = move || {
        provide_context(state.clone());
    };

    let mut app = Router::new();

    for (path, method) in ::server_fn::axum::server_fn_paths() {
        log::info!("{path} with {method} method");

        let cx_with_state = cx_with_state.clone();
        let handler =
            move |req| async move { handle_server_fns_with_context(cx_with_state, req).await };

        app = app.route(
            path,
            match method {
                Method::GET => get(handler),
                Method::POST => post(handler),
                Method::PUT => put(handler),
                Method::DELETE => delete(handler),
                Method::PATCH => patch(handler),
                _ => {
                    panic!(
                        "Unsupported server function HTTP method: \
                             {method:?}"
                    );
                }
            },
        );
    }

    let app = app.with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
