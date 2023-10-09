use async_session::SessionStore;
use async_trait::async_trait;
use axum::{
    extract::{rejection::TypedHeaderRejectionReason, FromRef, FromRequestParts},
    headers,
    http::request::Parts,
    RequestPartsExt, TypedHeader,
};
use hyper::header;
use serde::{Deserialize, Serialize};

use crate::{app_state::SessionStoreImpl, define::SESSION_COOKIE, diagnostics};

use super::Entity;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn with_name(name: String) -> Self {
        User {
            id: i64::default(),
            name,
            email: "abc@d.e".to_owned(),
        }
    }

    pub fn new(id: i64, name: String) -> Self {
        User {
            id,
            name,
            email: "abc@d.e".to_owned(),
        }
    }
}

impl Entity for User {
    type ID = i64;

    fn get_id(&self) -> &Self::ID {
        &self.id
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
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
                    TypedHeaderRejectionReason::Missing => diagnostics::Error::AuthRedirect,
                    _ => panic!("unexpected error getting Cookie header(s): {e}"),
                },
                _ => panic!("unexpected error getting cookies: {e}"),
            })?;
        let session_cookie = cookies
            .get(SESSION_COOKIE)
            .ok_or(diagnostics::Error::AuthRedirect)?;
        
        let session_cookie = urlencoding::decode(session_cookie).unwrap().to_string();
        tracing::debug!("{}", session_cookie);
        let session = store
            .load_session(session_cookie)
            .await?
            .ok_or(diagnostics::Error::AuthRedirect)?;

        let user = session
            .get::<User>("user")
            .ok_or(diagnostics::Error::AuthRedirect)?;

        Ok(user)
    }
}
