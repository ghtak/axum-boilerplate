use async_session::SessionStore;
use async_trait::async_trait;
use axum::{
    extract::{rejection::TypedHeaderRejectionReason, FromRef, FromRequestParts},
    headers,
    http::request::Parts,
    RequestPartsExt, TypedHeader,
};
use hyper::header;

use crate::{app_state::SessionStoreImpl, define::SESSION_COOKIE, diagnostics, entity::User};

use super::Depend;

#[async_trait]
impl<S> FromRequestParts<S> for Depend<User>
where
    SessionStoreImpl: FromRef<S> + SessionStore,
    S: Send + Sync,
{
    type Rejection = diagnostics::Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = SessionStoreImpl::from_ref(state);

        let cookies = parts
            .extract::<TypedHeader<headers::Cookie>>()
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => diagnostics::Error::Unauthorized,
                    _ => panic!("unexpected error getting Cookie header(s): {e}"),
                },
                _ => panic!("unexpected error getting cookies: {e}"),
            })?;
        let session_cookie = cookies
            .get(SESSION_COOKIE)
            .ok_or(diagnostics::Error::Unauthorized)?;

        let session_cookie = urlencoding::decode(session_cookie).unwrap().to_string();
        tracing::debug!("{}", session_cookie);
        let session = store
            .load_session(session_cookie)
            .await?
            .ok_or(diagnostics::Error::Unauthorized)?;

        let user = session
            .get::<User>("user")
            .ok_or(diagnostics::Error::Unauthorized)?;

        Ok(Depend(user))
    }
}
