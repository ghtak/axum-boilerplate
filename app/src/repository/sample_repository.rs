use axum::extract::FromRef;

use crate::{
    app_state::{AppState, DBPool},
    diagnostics,
    entity::{Entity, Sample},
};

#[derive(Debug)]
pub(crate) struct SampleRepository {
    pub pool: DBPool,
}

impl SampleRepository {
    pub fn new(pool: DBPool) -> Self {
        SampleRepository { pool }
    }

    pub async fn save(&self, sample: Sample) -> diagnostics::Result<Sample> {
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

    pub async fn find_all(&self) -> diagnostics::Result<Vec<Sample>> {
        Ok(sqlx::query_as::<_, Sample>("select * from sample")
            .fetch_all(&self.pool)
            .await?)
    }
}

impl FromRef<AppState> for SampleRepository {
    fn from_ref(state: &AppState) -> Self {
        SampleRepository::new(state.db_pool.clone())
    }
}
