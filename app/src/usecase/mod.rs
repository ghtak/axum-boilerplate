mod basic_sample_usecase;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};

use crate::{diagnostics, repository::SampleRepositoryDB};

pub(crate) struct Usecase<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for Usecase<T>
where
    S: Send + Sync,
    T: FromRef<S>,
{
    type Rejection = diagnostics::Error;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> diagnostics::Result<Self> {
        Ok(Usecase::<T>(T::from_ref(state).into()))
    }
}

pub(crate) use self::basic_sample_usecase::BasicSampleUsecase;
// user custom exports
pub(crate) type SampleUsecase = BasicSampleUsecase<SampleRepositoryDB>;
