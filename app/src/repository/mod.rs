pub(crate) mod sample_repository;
pub(crate) mod sample_repository_impl;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};

use crate::diagnostics;

#[async_trait]
pub(crate) trait CrudRepository<EntityT, IdT> {
    async fn add(&self, entity: EntityT) -> diagnostics::Result<EntityT>;

    async fn find_all(&self) -> diagnostics::Result<Vec<EntityT>>;
    async fn find_by_id(&self, id: &'_ IdT) -> diagnostics::Result<EntityT>;

    async fn save(&self, entity: EntityT) -> diagnostics::Result<EntityT>;

    async fn delete(&self, entity: EntityT) -> diagnostics::Result<()>;
    async fn delete_all(&self) -> diagnostics::Result<()>;
    async fn delete_by_id(&self, id: &'_ IdT) -> diagnostics::Result<()>;

    // async fn delete_all_by_id<'a, I>(&self, ids: IntoIterator)
    // where
    //     I: Iterator<Item = &IdT>;
    // async fn find_all_by_id(
    //     &self,
    //     ids: &'_ impl Iterator<Item = &'_ IdT>,
    // ) -> diagnostics::Result<Vec<EntityT>>;
}

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

pub(crate) use self::sample_repository::{SampleRepository, SampleRepositoryDB};
