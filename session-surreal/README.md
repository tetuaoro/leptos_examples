# Axum Session with Database

This example demonstrates how to create and manage user sessions using **Axum**, with **SurrealDB** as the session store, leveraging the **`tower_sessions`** and **`tower_sessions_surrealdb_store`** crates.

## Features

- **Session Management** : Sessions are stored in SurrealDB to persist data across user requests.
- **Secure Cookie Storage** : Sessions are securely stored in cookies and verified with a secret key.
- **Async Database Support** : Uses the async capabilities of both SurrealDB and `tower_sessions` for efficient handling of user data.
- **Session Expiration** : Supports session expiration to manage session lifecycles.

## Installation

To include the required dependencies, add the following to your Cargo.toml :

```toml
[dependencies]
# ...
serde = { version = "1", optional = true }
surrealdb = { version = "1.5.4", optional = true }
once_cell = { version = "1.19.0", optional = true }
tower-sessions = { version = "0.12.3", optional = true }
tower-sessions-surrealdb-store = { version = "0.4.0", optional = true }
```

## How It Works

In this example, the session handler initializes a session store using SurrealDB, where session data is saved and cleaned up periodically. The session management is handled by a session layer, which includes options like session expiry on inactivity and whether to secure session cookies. Additionally, the application sets up a database layer that provides access to the SurrealDB state across routes. The `main.rs` file combines both the session and database services, integrating them into Axum’s routing system with support for handling Leptos routes and error handling. Also, edit the [template](./template.env) environment for the database.
