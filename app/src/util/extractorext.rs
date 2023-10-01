use axum::{
    async_trait,
    extract::{
        rejection::{JsonRejection, PathRejection},
        FromRequest, FromRequestParts, MatchedPath,
    },
    http::request::Parts, RequestPartsExt,
};
use hyper::Request;
use serde::de::DeserializeOwned;

use crate::diagnostics::Error;

pub struct Json<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for Json<T>
where
    axum::Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    S: Send + Sync,
    B: Send + 'static,
{
    type Rejection = Error;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();
        let path = parts
            .extract::<MatchedPath>()
            .await
            .map(|path| path.as_str().to_owned())
            .ok();
        let req = Request::from_parts(parts, body);
        match axum::Json::<T>::from_request(req, state).await {
            Ok(json) => Ok(Self(json.0)),
            Err(rejection) => {
                let err = match rejection {
                    JsonRejection::JsonDataError(_) => {
                        Error::Message(format!("{:?} {}", path, rejection.to_string()))
                    }
                    JsonRejection::JsonSyntaxError(_) => {
                        Error::Message(format!("{:?} {}", path, rejection.to_string()))
                    }
                    JsonRejection::MissingJsonContentType(_) => {
                        Error::Message(format!("{:?} {}", path, rejection.to_string()))
                    }
                    _ => Error::Message(rejection.to_string()),
                };
                Err(err)
            }
        }
    }
}

pub struct Path<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for Path<T>
where
    // these trait bounds are copied from `impl FromRequest for axum::extract::path::Path`
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let err = match rejection {
                    PathRejection::FailedToDeserializePathParams(inner) => {
                        let kind = inner.into_kind();
                        let inner_err = match &kind {
                            _ => Error::Message(format!("FailedToDeserializePathParams: {}", kind)),
                        };
                        inner_err
                    }
                    PathRejection::MissingPathParams(error) => Error::Message(error.to_string()),
                    _ => Error::Message(format!("Unhandled path rejection: {}", rejection)),
                };
                Err(err)
            }
        }
    }
}
