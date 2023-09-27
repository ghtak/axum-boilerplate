use crate::{app_state::DBPool, diagnostics, entity::Sample};

#[derive(Debug)]
pub(crate) struct SampleRepository {
    pub pool: DBPool,
}

impl SampleRepository {
    pub fn new(pool: DBPool) -> Self {
        SampleRepository { pool }
    }

    pub async fn save(&self, sample: Sample) -> diagnostics::Result<Sample> {
        Ok(sqlx::query_as::<_, Sample>(
            r#"
            insert into sample(name)
            values ($1)
            returning id, name
            "#,
        )
        .bind(sample.name.as_str())
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn find_all(&self) -> diagnostics::Result<Vec<Sample>> {
        Ok(sqlx::query_as::<_, Sample>("select * from sample")
            .fetch_all(&self.pool)
            .await?)
    }
}


impl From<DBPool> for SampleRepository {
    fn from(pool: DBPool) -> Self {
        SampleRepository { pool }
    }
}