# Axum Session with Database

This example demonstrates how to create and manage user sessions using **Axum**, with **SurrealDB** as the session store, leveraging the **`tower_sessions`** and **`tower_sessions_surrealdb_store`** crates.

## Features

- **Session Management** : Sessions are stored in SurrealDB to persist data across user requests.
- **Secure Cookie Storage** : Sessions are securely stored in cookies and verified with a secret key.
- **Async Database Support** : Uses the async capabilities of both SurrealDB and `tower_sessions` for efficient handling of user data.
- **Session Expiration** : Supports session expiration to manage session lifecycles.

## How It Works

In this example, the session handler initializes a session store using SurrealDB, where session data is saved and cleaned up periodically. The session management is handled by a session layer, which includes options like session expiry on inactivity and whether to secure session cookies. Additionally, the application sets up a database layer that provides access to the SurrealDB state across routes. The `main.rs` file combines both the session and database services, integrating them into Axum’s routing system with support for handling Leptos routes and error handling.

## How to Launch

1. **Fill the `.env` File** : Start by creating a `.env` file based on the provided [TEMPLATE.env](./TEMPLATE.env). Customize it with your own settings for the database connection, session secrets, and other configurations.
2. **Start SurrealDB** : Launch your SurrealDB instance using the desired configuration, ensuring it's accessible based on your .env settings.
3. **Run the Application** : Once the environment and database are ready, simply use the following command to start the application :

```bash
cargo-leptos serve
```
