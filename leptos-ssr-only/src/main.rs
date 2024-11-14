#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos::*;
    use leptos_axum::generate_route_list;
    #[allow(unused_imports)]
    use leptos_ssr_only::app::*;

    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    _ = generate_route_list(|| ());
    dbg!(server_fn::axum::server_fn_paths().collect::<Vec<_>>());

    let app = Router::new().route(
        "/api/*fn_name",
        axum::routing::any(leptos_axum::handle_server_fns),
    );

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
