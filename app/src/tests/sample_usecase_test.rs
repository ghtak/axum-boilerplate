use std::{borrow::BorrowMut, collections::HashMap};

use axum::{async_trait, extract::FromRef};
use tokio::sync::RwLock;

use crate::{
    app_state::AppState,
    diagnostics,
    entity::Sample,
    repository::{BasicRepository, SampleRepository, SampleRepositoryDB},
    usecase::BasicSampleUsecase,
    utils,
};

struct SampleRepositoryMap {
    map: RwLock<HashMap<i64, String>>,
}

impl SampleRepositoryMap {
    pub fn new() -> Self {
        SampleRepositoryMap {
            map: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl BasicRepository<Sample, i64> for SampleRepositoryMap {
    async fn create(&self, entity: Sample) -> diagnostics::Result<Sample> {
        let mut g = self.map.write().await;
        let map = g.borrow_mut();
        let id = if map.len() == 0 {
            0
        } else {
            map.keys().max().unwrap() + 1
        };
        map.insert(id, entity.name.clone());
        Ok(Sample::new(id, entity.name))
    }

    async fn find_all(&self) -> diagnostics::Result<Vec<Sample>> {
        let map = self.map.read().await;
        Ok(map
            .iter()
            .map(|item| Sample::new(*item.0, item.1.clone()))
            .collect::<Vec<Sample>>())
    }

    async fn find_by_id(&self, id: &'_ i64) -> diagnostics::Result<Sample> {
        let map = self.map.read().await;
        if let Some(name) = map.get(id) {
            Ok(Sample::new(id.clone(), name.clone()))
        } else {
            Err(diagnostics::Error::RowNotFound)
        }
    }

    async fn update(&self, entity: Sample) -> diagnostics::Result<Sample> {
        let mut g = self.map.write().await;
        let map = g.borrow_mut();
        *map.entry(entity.id).or_insert("".to_owned()) = entity.name.clone();
        Ok(Sample::new(entity.id, entity.name))
    }

    async fn delete(&self, entity: Sample) -> diagnostics::Result<()> {
        Ok(self.delete_by_id(&entity.id).await?)
    }

    async fn delete_all(&self) -> diagnostics::Result<()> {
        let mut g = self.map.write().await;
        let map = g.borrow_mut();
        map.clear();
        Ok(())
    }

    async fn delete_by_id(&self, id: &'_ i64) -> diagnostics::Result<()> {
        let mut g = self.map.write().await;
        let map = g.borrow_mut();
        map.remove(id);
        Ok(())
    }

    async fn find_all_by_id(&self, _ids: Vec<i64>) -> diagnostics::Result<Vec<Sample>>{
        Err(diagnostics::Error::RowNotFound)
    }

    async fn delete_all_by_id(&self, _ids: Vec<i64>) -> diagnostics::Result<()>{
        Ok(())
    }
}

#[async_trait]
impl SampleRepository for SampleRepositoryMap {}

impl FromRef<AppState> for SampleRepositoryMap {
    fn from_ref(_: &AppState) -> Self {
        SampleRepositoryMap::new()
    }
}

#[tokio::test]
async fn sample_usecase() {
    let config = utils::config::TomlConfig::from_file("config.toml").unwrap();
    let _guard = utils::tracing::init(&config.tracing).unwrap();
    let app_state = AppState::new(&config).await;
    let _ = app_state.create_tables().await;
    let _ = BasicSampleUsecase::<SampleRepositoryDB>::new(SampleRepositoryDB::new(
        app_state.db_pool.clone(),
    ));
    let sample_usecase_impl =
        BasicSampleUsecase::<SampleRepositoryMap>::new(SampleRepositoryMap::new());
    let s = sample_usecase_impl
        .create(Sample::with_name("s".into()))
        .await;
    let _s1 = sample_usecase_impl
        .create(Sample::with_name("s1".into()))
        .await;
    
    println!("{:?}", s);
    let samples = sample_usecase_impl.find_all().await;
    println!("samples {:?}", samples);
}
