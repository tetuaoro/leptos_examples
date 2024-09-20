# Axum Static Assets Caching

This example demonstrates how to implement caching for static assets (e.g., CSS, JavaScript, WASM) using **Axum** with the help of the **`cached`** and **`once_cell`** crates.

## Features

- **Asset Caching** : Responses for asset types like CSS, JS, WebP, etc., are cached in memory to speed up subsequent requests.
- **Cache-Control Headers** : Adds custom cache headers (`x-cache-status`, `cache-control`) to the responses to optimize browser-side caching.
- **Async Caching** : Uses the `cached` crate with async support to handle caching efficiently.
- **Lazy Initialization** : The `once_cell` crate is used to lazily initialize static asset types for content-type checking.

## Installation

To include the required dependencies, add the following to your `Cargo.toml` :

```toml
[dependencies]
# ...
cached = { version = "0.53.1", features = ["async"], optional = true }
once_cell = { version = "1.19.0", optional = true }
```

### How It Works

When a request is made, the URI is checked against the in-memory cache. If found, the cached response is returned. If the request isn't cached, the `Content-Type` header is checked. If it matches a predefined asset type, the response is cached for future requests.
Responses for cached assets are tagged with `x-cache-status` to `HIT` and `cache-control` to `public, max-age=31536000` headers to optimize client-side caching.
