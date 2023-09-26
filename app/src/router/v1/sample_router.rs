use axum::{extract::State, routing::get, Json, Router};
use axum_extra::extract::WithRejection;

use crate::{app_state::AppState, diagnostics, entity::Sample, repository::SampleRepository, dto};

async fn get_samples(State(app_state): State<AppState>) -> diagnostics::Result<Json<Vec<Sample>>> {
    let repo = SampleRepository::new(&app_state.db_pool);
    let samples = repo.find_all().await?;
    Ok(Json(samples))
}

async fn create_sample(
    State(app_state): State<AppState>,
    WithRejection(Json(v), _): WithRejection<Json<dto::CreateSample>, diagnostics::Error>,
) -> diagnostics::Result<Json<Sample>> {
    let repo = SampleRepository::new(&app_state.db_pool);
    let samples = repo.save(Sample { id: -1, name: v.name }).await?;
    Ok(Json(samples))
}

pub(crate) fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_samples).post(create_sample))
        .with_state(app_state)
}
