use axum::{extract::FromRef, async_trait};

use crate::{
    app_state::{AppState, DBPool},
    diagnostics,
    entity::{Entity, Sample},
};

use super::SampleRepository;

#[derive(Debug)]
pub(crate) struct SampleRepositoryImpl {
    pub pool: DBPool,
}

impl SampleRepositoryImpl {
    pub fn new(pool: DBPool) -> Self {
        SampleRepositoryImpl { pool }
    }
}

#[async_trait]
impl SampleRepository for SampleRepositoryImpl{
    async fn save(&self, sample: Sample) -> diagnostics::Result<Sample> {
        if Entity::is_new(&sample) {
            Ok(
                sqlx::query_as::<_, Sample>(
                    r#" insert into sample(name) values ($1) returning * "#,
                )
                .bind(sample.name.as_str())
                .fetch_one(&self.pool)
                .await?,
            )
        } else {
            Ok(sqlx::query_as::<_, Sample>(
                r#" insert or replace into sample(id,name) values ($1,$2) returning * "#,
            )
            .bind(sample.id)
            .bind(sample.name.as_str())
            .fetch_one(&self.pool)
            .await?)
        }
    }

    async fn find_all(&self) -> diagnostics::Result<Vec<Sample>> {
        Ok(sqlx::query_as::<_, Sample>("select * from sample")
            .fetch_all(&self.pool)
            .await?)
    }
}

impl FromRef<AppState> for SampleRepositoryImpl {
    fn from_ref(state: &AppState) -> Self {
        SampleRepositoryImpl::new(state.db_pool.clone())
    }
}
