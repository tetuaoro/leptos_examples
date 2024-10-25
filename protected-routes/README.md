# Protected Routes

This example demonstrates how to protect routes with `<ProtectedRoute/>` component, `#[middleware]` macro and `.layer(...)` function provided by `Axum`.

## Features

- **Protected Routes** : Ensure that certain routes can only be accessed by authenticated users.

## How It Works

When a request is made, the server first runs the middleware layer defined in `main` function. This middleware is responsible for checking if the user has an active session. If the session token is valid, the middleware allows the request to proceed; otherwise, it redirects the user to a login page or an error page.

## How to Launch

1. **Fill the `.env` File** : Start by creating a `.env` file based on the provided [TEMPLATE.env](./TEMPLATE.env). Customize it with your own settings for the database connection, session secrets, and other configurations.
2. **Start SurrealDB** : Launch your SurrealDB instance using the desired configuration, ensuring it's accessible based on your .env settings.
3. **Run the Application** : Once the environment and database are ready, simply use the following command to start the application :

```bash
cargo-leptos serve
```
