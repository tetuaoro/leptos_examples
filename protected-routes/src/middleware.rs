use crate::surreal::Token;
use axum::body::Body;
use http::{Request, Response, StatusCode};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tower_sessions::Session;

pub struct AuthLayer;

impl<S> Layer<S> for AuthLayer {
    type Service = AuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthService { inner }
    }
}

pub struct AuthService<T> {
    inner: T,
}

impl<T> AuthService<T> {
    #[allow(dead_code)]
    fn handler() -> Result<(), ()> {
        Ok(())
    }
}

impl<T> Service<Request<Body>> for AuthService<T>
where
    T: Service<Request<Body>, Response = Response<Body>> + Send + 'static,
    T::Future: Send + 'static,
{
    type Response = Response<Body>;
    type Error = T::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let extensions = req.extensions().clone();
        let next_req = self.inner.call(req);

        let session_fut = async move {
            if let Some(session) = extensions.get::<Session>() {
                if let Ok(Some(_token)) = session.get::<Token>(Token::KEY).await {
                    // run next request
                    let response = next_req.await;
                    return response;
                }
            }

            let response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("Session does not exist"))
                .expect("couldn't build a response");

            Ok(response)
        };

        Box::pin(session_fut)
    }
}
