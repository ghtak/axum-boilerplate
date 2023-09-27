use axum::extract::FromRef;

use crate::{app_state::AppState, diagnostics, entity::Sample, repository::SampleRepository};

pub(crate) struct SampleUsecase {
    pub sample_repository: SampleRepository,
}

impl SampleUsecase {
    pub fn new(sample_repository: SampleRepository) -> Self {
        SampleUsecase { sample_repository }
    }

    pub async fn find_all(&self) -> diagnostics::Result<Vec<Sample>> {
        let samples = self.sample_repository.find_all().await?;
        Ok(samples)
    }

    pub async fn save(&self, sample: Sample) -> diagnostics::Result<Sample> {
        let sample = self.sample_repository.save(sample).await?;
        Ok(sample)
    }
}

impl FromRef<AppState> for SampleUsecase {
    fn from_ref(state: &AppState) -> Self {
        SampleUsecase::new(SampleRepository::from_ref(state))
    }
}
