use axum::{
    body::{to_bytes, Body, Bytes},
    extract::Request,
    http::{header, response::Parts, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
    Error as AxumError,
};
use cached::proc_macro::cached;
use cached::{Cached, SizedCache};
use std::sync::LazyLock as Lazy;

static ASSETS_TYPE: Lazy<Vec<&str>> =
    Lazy::new(|| vec!["css", "javascript", "wasm", "webp", "ttf", "x-icon"]);

#[cached(
    result = true,
    ty = "SizedCache<String, (Parts, Bytes)>",
    create = "{ SizedCache::with_size(200) }"
)]
async fn cached_response(_path: String) -> Result<(Parts, Bytes), AxumError> {
    todo!()
}

const CACHE_X_K: &str = "x-cache-status";
const CACHE_X_V: &str = "HIT";
const CACHE_C_K: &str = "cache-control";
const CACHE_C_V: &str = "public, max-age=31536000";

/// Middleware function for handling requests and caching responses for static assets.
///
/// 1. **Cache Lookup** : The function first checks if the requested URI is already cached
///    in the `CACHED_RESPONSE`. If found, the cached response is returned with a `PARTIAL_CONTENT` status.
///
/// 2. **Forward Request** : If the request URI isn't cached, it is forwarded to the next handler using
///    `next.run(req).await`.
///
/// 3. **Content-Type Check** : The `Content-Type` header of the response is inspected to determine if the
///    response corresponds to an asset type (e.g., CSS, JS, WebP). These types are listed in the `ASSETS_TYPE` static variable.
///
/// 4. **Cache Response** : If the response is of an asset type :
///    - Adds `x-cache-status: HIT` and `cache-control: public, max-age=31536000` headers.
///    - Stores the response in the cache for future requests, ensuring faster access.
///
/// 5. **Return Response** : If the response is not cacheable, the function returns the response as-is.

pub async fn handler(req: Request, next: Next) -> Response {
    let uri = req.uri().to_owned();
    let key = uri.to_string();

    // if match uri_key, return `cached`

    if let Some((parts, bytes)) = CACHED_RESPONSE.lock().await.cache_get(&key) {
        let body = Body::from(bytes.to_owned());
        let mut response = Response::from_parts(parts.to_owned(), body);
        *response.status_mut() = StatusCode::PARTIAL_CONTENT;
        return response;
    }

    // if match content type, `cached`

    let response = next.run(req).await;
    let (mut parts, body) = response.into_parts();

    let content_type = parts
        .headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("not_an_asset_type_condition");

    let is_contained = ASSETS_TYPE.iter().any(|at| content_type.contains(at));
    if is_contained {
        let c_x_v = HeaderValue::from_static(CACHE_X_V);
        let c_c_v = HeaderValue::from_static(CACHE_C_V);

        _ = parts.headers.insert(CACHE_X_K, c_x_v);
        _ = parts.headers.insert(CACHE_C_K, c_c_v);

        let bytes = to_bytes(body, usize::MAX)
            .await
            .expect(StatusCode::INSUFFICIENT_STORAGE.as_str());

        _ = CACHED_RESPONSE
            .lock()
            .await
            .cache_set(key, (parts.clone(), bytes.clone()));

        return Response::from_parts(parts, Body::from(bytes));
    }

    // next

    Response::from_parts(parts, body)
}
