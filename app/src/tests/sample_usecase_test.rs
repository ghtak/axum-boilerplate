
use std::{collections::HashMap, cell::RefCell, sync::RwLock, borrow::BorrowMut};

use axum::async_trait;

use crate::{utils, app_state::{AppState, DBPool}, usecase::BasicSampleUsecase, repository::{SampleRepositoryImpl, SampleRepository}, entity::{Entity, Sample}, diagnostics};


struct SampleRepositoryMap{
    cell_map: RwLock<HashMap<i64, String>>
}

impl SampleRepositoryMap{
    pub fn new(_: DBPool) -> Self {
        SampleRepositoryMap{ cell_map: RwLock::new(HashMap::new()) }
    }
}


#[async_trait]
impl SampleRepository for SampleRepositoryMap {
    async fn save(&self, sample: Sample) -> diagnostics::Result<Sample> {
        let mut map = self.cell_map.write().map_err(|e|diagnostics::Error::Message(format!("{:?}", e)))?;
        if Entity::is_new(&sample) {
            let id = if map.len() == 0 {
                0
            } else {
                map.keys().max().unwrap() + 1
            };
            map.insert(id, sample.name.clone());
            Ok(Sample::new(id, sample.name))
        } else {
            *map.entry(sample.id).or_insert("".to_owned()) = sample.name.clone();
            Ok(Sample::new(sample.id, sample.name))
        }
    }

    async fn find_all(&self) -> diagnostics::Result<Vec<Sample>> {
        let map = self.cell_map.read().map_err(|e|
            diagnostics::Error::Message(format!("{:?}", e))
        )?;
        Ok(map
            .iter()
            .map(|item| Sample::new(*item.0, item.1.clone()))
            .collect::<Vec<Sample>>())
    }
}


#[tokio::test]
async fn sample_usecase() {
    let config = utils::config::TomlConfig::from_file("config.toml").unwrap();
    let _guard = utils::tracing::init(&config.tracing).unwrap();
    let app_state = AppState::new(&config).await;
    let _ = app_state.create_tables().await;
    // let sample_usecase_impl = BasicSampleUsecase::<SampleRepositoryImpl>::new(
    //     SampleRepositoryImpl::new(app_state.db_pool.clone()));
    let sample_usecase_impl = BasicSampleUsecase::<SampleRepositoryMap>::new(
        SampleRepositoryMap::new(app_state.db_pool.clone()));
    let s = sample_usecase_impl.save(Sample::from_name("xyz".into())).await;
    println!("{:?}", s);
    let samples = sample_usecase_impl.find_all().await;
    println!("samples {:?}", samples);
}