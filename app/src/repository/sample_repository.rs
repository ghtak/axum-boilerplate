use axum::async_trait;

use crate::{
    diagnostics,
    entity::Sample,
};

#[async_trait]
pub(crate) trait SampleRepository {
    async fn save(&self, sample: Sample) -> diagnostics::Result<Sample>;
    async fn find_all(&self) -> diagnostics::Result<Vec<Sample>>;
}

