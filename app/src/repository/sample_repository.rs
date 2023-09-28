use axum::{async_trait, extract::FromRef};

use crate::{
    app_state::{AppState, DBPool},
    diagnostics,
    entity::Sample,
};

use super::CrudRepository;

#[async_trait]
pub(crate) trait SampleRepository: CrudRepository<Sample, i64> + FromRef<AppState> {}

pub(crate) struct SampleRepositoryDB {
    pub pool: DBPool,
}

impl SampleRepositoryDB {
    pub fn new(pool: DBPool) -> Self {
        SampleRepositoryDB { pool }
    }
}

#[async_trait]
impl CrudRepository<Sample, i64> for SampleRepositoryDB {
    async fn add(&self, entity: Sample) -> diagnostics::Result<Sample> {
        Ok(
            sqlx::query_as::<_, Sample>(r#" insert into sample(name) values ($1) returning * "#)
                .bind(entity.name.as_str())
                .fetch_one(&self.pool)
                .await?,
        )
    }

    async fn find_all(&self) -> diagnostics::Result<Vec<Sample>> {
        Ok(sqlx::query_as::<_, Sample>("select * from sample")
            .fetch_all(&self.pool)
            .await?)
    }

    async fn find_by_id(&self, id: &'_ i64) -> diagnostics::Result<Sample> {
        Ok(
            sqlx::query_as::<_, Sample>("select * from sample where id = ($1)")
                .bind(id)
                .fetch_one(&self.pool)
                .await?,
        )
    }

    async fn save(&self, entity: Sample) -> diagnostics::Result<Sample> {
        Ok(sqlx::query_as::<_, Sample>(
            r#" insert or replace into sample(id,name) values ($1,$2) returning * "#,
        )
        .bind(entity.id)
        .bind(entity.name.as_str())
        .fetch_one(&self.pool)
        .await?)
    }

    async fn delete(&self, entity: Sample) -> diagnostics::Result<()> {
        Ok(self.delete_by_id(&entity.id).await?)
    }

    async fn delete_all(&self) -> diagnostics::Result<()> {
        sqlx::query(r#"delete from sample"#)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete_by_id(&self, id: &'_ i64) -> diagnostics::Result<()> {
        sqlx::query(r#"delete from sample where id = ($1)"#)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

impl FromRef<AppState> for SampleRepositoryDB {
    fn from_ref(state: &AppState) -> Self {
        SampleRepositoryDB::new(state.db_pool.clone())
    }
}

#[async_trait]
impl SampleRepository for SampleRepositoryDB {}
