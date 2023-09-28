use axum::{extract::State, routing::get, Json, Router};
use axum_extra::extract::WithRejection;

use crate::{
    app_state::AppState,
    diagnostics, dto,
    entity::Sample,
    repository::{Repository, SampleRepositoryDB, CrudRepository},
    usecase::{SampleUsecase, Usecase},
};

async fn get_samples(State(app_state): State<AppState>) -> diagnostics::Result<Json<Vec<Sample>>> {
    let repo = SampleRepositoryDB::new(app_state.db_pool.clone());
    let samples = repo.find_all().await?;
    Ok(Json(samples))
}

async fn create_sample(
    State(app_state): State<AppState>,
    WithRejection(Json(v), _): WithRejection<Json<dto::SampleCreate>, diagnostics::Error>,
) -> diagnostics::Result<Json<Sample>> {
    let repo = SampleRepositoryDB::new(app_state.db_pool.clone());
    let samples = repo.save(Sample::from_name(v.name)).await?;
    Ok(Json(samples))
}

async fn get_samples_v2(
    Repository(sample_repo): Repository<SampleRepositoryDB>,
) -> diagnostics::Result<Json<Vec<Sample>>> {
    let samples = sample_repo.find_all().await?;
    Ok(Json(samples))
}

async fn create_sample_v2(
    Repository(sample_repo): Repository<SampleRepositoryDB>,
    WithRejection(Json(v), _): WithRejection<Json<dto::SampleCreate>, diagnostics::Error>,
) -> diagnostics::Result<Json<Sample>> {
    let samples = sample_repo.save(Sample::from_name(v.name)).await?;
    Ok(Json(samples))
}

async fn get_samples_v3(
    Usecase(sample_usecase): Usecase<SampleUsecase>,
) -> diagnostics::Result<Json<Vec<Sample>>> {
    let samples = sample_usecase.find_all().await?;
    Ok(Json(samples))
}

async fn create_sample_v3(
    Usecase(sample_usecase): Usecase<SampleUsecase>,
    WithRejection(Json(v), _): WithRejection<Json<dto::SampleCreate>, diagnostics::Error>,
) -> diagnostics::Result<Json<Sample>> {
    let sample = sample_usecase.save(Sample::from_name(v.name)).await?;
    Ok(Json(sample))
}

pub(crate) fn router_(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_samples).post(create_sample))
        .with_state(app_state)
}

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/", get(get_samples_v3).post(create_sample_v3))
}
