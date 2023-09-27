mod sample_repository;

use axum::{extract::{FromRef, FromRequestParts}, async_trait, http::request::Parts};

use crate::{app_state::*, diagnostics};

pub(crate) use self::sample_repository::SampleRepository;


pub(crate) struct Repository<T>(pub T);



#[async_trait]
impl<T,S> FromRequestParts<S> for Repository<T>
where
    S : Send + Sync,
    T:  From<DBPool>,
    DBPool: FromRef<S>,
{
    type Rejection = diagnostics::Error;

    async fn from_request_parts(
        _parts: &mut Parts, state: &S
    ) -> diagnostics::Result<Self> {
        Ok(Repository::<T>(DBPool::from_ref(state).into()))
    }
}

