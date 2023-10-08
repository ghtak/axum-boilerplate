use std::{borrow::BorrowMut, collections::HashMap};

use axum::{async_trait, extract::FromRef};
use tokio::sync::RwLock;

use crate::{
    app_state::AppState,
    diagnostics,
    entity::{Sample, Entity},
    repository::{BasicRepository, SampleRepository, SampleRepositoryDB},
    usecase::BasicSampleUsecase,
    util,
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
impl BasicRepository<Sample> for SampleRepositoryMap {
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

    async fn find_all_by_id<I>(&self, ids: I) -> diagnostics::Result<Vec<Sample>>
    where
        I: Iterator<Item = &'async_trait <Sample as Entity>::ID> + Send,
        <Sample as Entity>::ID: 'async_trait,
    {
        let map = self.map.read().await;
        let samples = ids
            .filter_map(|id| {
                if let Some(name) = map.get(&id) {
                    Some(Sample::new(id.clone(), name.clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<Sample>>();
        Ok(samples)
    }

    async fn delete_all_by_id<I>(&self, ids: I) -> diagnostics::Result<()>
    where
        I: Iterator<Item = &'async_trait <Sample as Entity>::ID> + Send,
        <Sample as Entity>::ID: 'async_trait,
    {
        let mut g = self.map.write().await;
        let map = g.borrow_mut();
        ids.for_each(|x| {
            map.remove(x);
        });
        Ok(())
    }

    // async fn find_all_by_id<I>(&self, ids: I) -> diagnostics::Result<Vec<Sample>>
    // where
    //     I: IntoIterator<Item = i64> + Send + 'async_trait,
    // {
    //     let map = self.map.read().await;
    //     let samples = ids
    //         .into_iter()
    //         .filter_map(|id| {
    //             if let Some(name) = map.get(&id) {
    //                 Some(Sample::new(id.clone(), name.clone()))
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect::<Vec<Sample>>();
    //     Ok(samples)
    // }

    // async fn delete_all_by_id<I>(&self, _ids: I) -> diagnostics::Result<()>
    // where
    //     I: IntoIterator<Item = i64> + Send + 'async_trait,
    // {
    //     Err(diagnostics::Error::NotImplemented)
    // }
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
    let config = util::config::TomlConfig::from_file("config.toml").unwrap();
    let _guard = util::tracing::init(&config.tracing).unwrap();
    let app_state = AppState::new(&config).await;
    if config.database.with_migrations {
        let _ = app_state.migrate_database().await;
    }

    let _ = BasicSampleUsecase::<SampleRepositoryDB>::from_ref(&app_state);
    let sample_usecase_impl = BasicSampleUsecase::<SampleRepositoryMap>::from_ref(&app_state);
    let s = sample_usecase_impl
        .create(Sample::with_name("s".into()))
        .await;
    let s1 = sample_usecase_impl
        .create(Sample::with_name("s1".into()))
        .await;
    let v = vec![0, 1,2,3];
    let samples = sample_usecase_impl.sample_repository.find_all_by_id(v.iter()).await;

    println!("{:?} {:?} {:?}", s, s1, samples);
    let _ = sample_usecase_impl.sample_repository.delete_all_by_id(vec![0].iter()).await;
    let samples = sample_usecase_impl.find_all().await;
    println!("samples {:?}", samples);
}
