use axum::extract::FromRef;

use crate::{app_state::AppState, diagnostics, entity::Sample, repository::SampleRepository};

pub(crate) struct BasicSampleUsecase<SampleRepositoryT> {
    pub sample_repository: SampleRepositoryT,
}

impl<SampleRepositoryT> BasicSampleUsecase<SampleRepositoryT>
where
    SampleRepositoryT: SampleRepository,
{
    pub fn new(sample_repository: SampleRepositoryT) -> Self {
        BasicSampleUsecase { sample_repository }
    }

    pub async fn find_all(&self) -> diagnostics::Result<Vec<Sample>> {
        let samples = self.sample_repository.find_all().await?;
        Ok(samples)
    }

    pub async fn create(&self, sample: Sample) -> diagnostics::Result<Sample> {
        let sample = self.sample_repository.create(sample).await?;
        Ok(sample)
    }
}

impl<SampleRepositoryT> FromRef<AppState> for BasicSampleUsecase<SampleRepositoryT> 
    where SampleRepositoryT: FromRef<AppState> + SampleRepository
{
    fn from_ref(state: &AppState) -> Self {
        BasicSampleUsecase::new(SampleRepositoryT::from_ref(state))
    }
}
