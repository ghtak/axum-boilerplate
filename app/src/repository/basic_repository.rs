use axum::async_trait;

use crate::{diagnostics, entity::Entity};

#[async_trait]
pub(crate) trait BasicRepository<EntityT>
where
    EntityT: Entity,
{
    async fn create(&self, entity: EntityT) -> diagnostics::Result<EntityT>;

    async fn find_all(&self) -> diagnostics::Result<Vec<EntityT>>;
    async fn find_by_id(&self, id: &'_ EntityT::ID) -> diagnostics::Result<EntityT>;

    async fn find_all_by_id<I>(&self, ids: I) -> diagnostics::Result<Vec<EntityT>>
    where
        I: Iterator<Item = &'async_trait EntityT::ID> + Send,
        EntityT::ID: 'async_trait;

    async fn update(&self, entity: EntityT) -> diagnostics::Result<EntityT>;

    async fn delete(&self, entity: EntityT) -> diagnostics::Result<()>
    where
        EntityT: 'async_trait,
    {
        Ok(self.delete_by_id(entity.get_id()).await?)
    }

    async fn delete_all(&self) -> diagnostics::Result<()>;
    async fn delete_by_id(&self, id: &'_ EntityT::ID) -> diagnostics::Result<()>;
    async fn delete_all_by_id<I>(&self, ids: I) -> diagnostics::Result<()>
    where
        I: Iterator<Item = &'async_trait EntityT::ID> + Send,
        EntityT::ID: 'async_trait;

    // async fn find_all_by_id(&self, ids: Vec<EntityT::ID>) -> diagnostics::Result<Vec<EntityT>>;
    // async fn find_all_by_id<I>(&self, ids: I) -> diagnostics::Result<Vec<EntityT>>
    // where
    //     I: IntoIterator<Item = EntityT::ID> + Send + 'async_trait;

    //async fn delete_all_by_id(&self, ids: Vec<EntityT::ID>) -> diagnostics::Result<()>;
    // async fn delete_all_by_id<I>(&self, ids: I) -> diagnostics::Result<()>
    // where
    //     I: IntoIterator<Item = EntityT::ID> + Send + 'async_trait;
}
