use std::env;
use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, Extensions},
};
use tokio::sync::RwLock;

use crate::{diagnostics, util::config::TomlConfig};

#[cfg(feature = "enable_websocket_pubsub_sample")]
use crate::ws::pubsub::PubSubState;

use sqlx::Pool;

#[cfg(feature = "use_sqlite")]
mod db_impl {
    use sqlx::{sqlite::SqlitePoolOptions, Sqlite};
    pub(crate) type DataBase = Sqlite;
    pub(crate) type PoolOptions = SqlitePoolOptions;
}

#[cfg(feature = "use_postgres")]
mod db_impl {
    use sqlx::{postgres::PgPoolOptions, Postgres};
    pub(crate) type DataBase = Postgres;
    pub(crate) type PoolOptions = PgPoolOptions;
}

pub(crate) type DataBase = db_impl::DataBase;
pub(crate) type DataBasePoolOptions = db_impl::PoolOptions;

use bb8;
use bb8_redis;

pub(crate) type RedisPool = bb8::Pool<bb8_redis::RedisConnectionManager>;
pub(crate) struct RedisConnection(
    pub bb8::PooledConnection<'static, bb8_redis::RedisConnectionManager>,
);

mod session_impl{
    use async_session::MemoryStore;

    pub(crate) type SessionStoreImpl = MemoryStore;
}

pub(crate) type SessionStoreImpl = session_impl::SessionStoreImpl;

// https://docs.rs/axum/latest/axum/extract/struct.State.html
#[derive(Clone, Debug)]
pub(crate) struct AppState {
    pub db_pool: Pool<DataBase>,
    pub redis_pool: RedisPool,
    pub session_store: SessionStoreImpl,
    pub extentions: Arc<RwLock<Extensions>>,
    #[cfg(feature = "enable_websocket_pubsub_sample")]
    pub pubsub: PubSubState,
}

impl AppState {
    pub async fn new(config: &TomlConfig) -> Self {
        #[cfg(feature = "use_sqlite")]
        Self::sqlite_create_database(&config).await;

        // for sqlx::query!
        env::set_var("DATABASE_URL", config.database.url.as_str());
        AppState {
            db_pool: DataBasePoolOptions::new()
                .max_connections(config.database.max_connection)
                .connect(config.database.url.as_str())
                .await
                .expect("Unabled to Connect to Database"),

            redis_pool: bb8::Pool::builder()
                .build(bb8_redis::RedisConnectionManager::new(config.redis.url.as_str()).unwrap())
                .await
                .unwrap(),

            session_store: SessionStoreImpl::new(),

            extentions: Arc::new(RwLock::new(Extensions::default())),

            #[cfg(feature = "enable_websocket_pubsub_sample")]
            pubsub: PubSubState::new(),
        }
    }

    pub async fn migrate_database(&self) -> diagnostics::Result<()> {
        sqlx::migrate!("./migrations").run(&self.db_pool).await?;
        Ok(())
    }

    #[cfg(feature = "use_sqlite")]
    pub async fn sqlite_create_database(config: &TomlConfig) {
        use sqlx::migrate::MigrateDatabase;
        use sqlx::Sqlite;

        assert!(
            config.database.url.starts_with("sqlite://"),
            "invalid database url scheme"
        );
        let filename = config.database.url.replace("sqlite://", "");
        if !Sqlite::database_exists(&filename).await.unwrap_or(false) {
            match Sqlite::create_database(&filename).await {
                Ok(_) => tracing::debug!("Create db success"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            tracing::debug!("Database already exists");
        }
    }
}

// substate
// async fn handler(State(db_pool): State<Pool<DataBase>>) -> ...{}
impl FromRef<AppState> for Pool<DataBase> {
    fn from_ref(input: &AppState) -> Self {
        input.db_pool.clone()
    }
}

impl FromRef<AppState> for RedisPool {
    fn from_ref(input: &AppState) -> Self {
        input.redis_pool.clone()
    }
}

impl FromRef<AppState> for SessionStoreImpl {
    fn from_ref(input: &AppState) -> Self {
        input.session_store.clone()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for RedisConnection
where
    S: Send + Sync,
    RedisPool: FromRef<S>,
{
    type Rejection = diagnostics::Error;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> diagnostics::Result<Self> {
        let pool = RedisPool::from_ref(state);
        let conn = pool
            .get_owned()
            .await
            .map_err(|e| diagnostics::Error::BB8Error(e.to_string()))?;
        Ok(Self(conn))
    }
}
