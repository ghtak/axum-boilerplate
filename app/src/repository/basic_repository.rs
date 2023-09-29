
use axum::async_trait;

use crate::diagnostics;

#[async_trait]
pub(crate) trait BasicRepository<EntityT, IdT> {
    async fn create(&self, entity: EntityT) -> diagnostics::Result<EntityT>;

    async fn find_all(&self) -> diagnostics::Result<Vec<EntityT>>;
    async fn find_by_id(&self, id: &'_ IdT) -> diagnostics::Result<EntityT>;
    async fn find_all_by_id(&self, ids: Vec<IdT>) -> diagnostics::Result<Vec<EntityT>>;

    async fn update(&self, entity: EntityT) -> diagnostics::Result<EntityT>;

    async fn delete(&self, entity: EntityT) -> diagnostics::Result<()>;
    async fn delete_all(&self) -> diagnostics::Result<()>;
    async fn delete_by_id(&self, id: &'_ IdT) -> diagnostics::Result<()>;
    async fn delete_all_by_id(&self, ids: Vec<IdT>) -> diagnostics::Result<()>;
}