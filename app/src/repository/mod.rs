pub(crate) mod sample_repository;
pub(crate) mod sample_repository_impl;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};

use crate::diagnostics;

pub(crate) use self::sample_repository::SampleRepository;
pub(crate) use self::sample_repository_impl::SampleRepositoryImpl;

pub(crate) struct Repository<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for Repository<T>
where
    S: Send + Sync,
    T: FromRef<S>,
{
    type Rejection = diagnostics::Error;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> diagnostics::Result<Self> {
        Ok(Repository::<T>(T::from_ref(state)))
    }
}
