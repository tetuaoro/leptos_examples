# Leptos serve only API functions

This project demonstrates how to use **Leptos** alongside **Axum** as a server-side API only, leveraging the `server` macro from Leptos for lightweight and efficient server-side rendering (SSR) and API building. This based runs `leptos` and `axum` as server only. The leptos `#[server]` function is great to make a project like this.

## Features

- **Server-Side API with Leptos**: Use the powerful `#[server]` macro from Leptos to easily create SSR routes.
- **Lightweight**: Reduced dependency on extra crates, ensuring a streamlined and efficient setup.
- **Axum Integration**: Combine Leptos with Axum for flexible routing and server capabilities.

## How It Works

The Leptos `#[server]` macro provides a simple way to define server-side logic that integrates seamlessly with your Axum app. This project uses Axum as the main server framework and handles all HTTP requests through it, while the server-side logic is powered by Leptos. This combination allows for highly performant server-side responses and efficient API endpoints.

### Key Benefits

- **Minimal Setup**: With Leptos and Axum, you can build SSR APIs without needing extra server frameworks.
- **Unified Handling**: Define your server-side logic with Leptos macros and route requests via Axum for consistency.
- **Ease of Use**: The use of fewer crates simplifies dependency management and project maintenance.

## How to Launch

1. **Run the Application** : Simply use the following command to start the application :

```bash
cargo build --package=server --bin=server --no-default-features
LEPTOS_OUTPUT_NAME=raodeploy cargo run --package=server --bin=server --no-default-features
```