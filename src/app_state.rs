use std::env;
use std::sync::Arc;

use axum::{extract::FromRef, http::Extensions};
use tokio::sync::RwLock;

use crate::{diagnostics, util::config::TomlConfig};

#[cfg(feature = "enable_websocket_pubsub_sample")]
use crate::ws::pubsub::PubSubState;

#[cfg(feature = "dbtype_sqlite")]
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
#[cfg(feature = "dbtype_sqlite")]
pub type DataBase = Sqlite;

#[cfg(feature = "dbtype_postgres")]
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
#[cfg(feature = "dbtype_postgres")]
pub type DataBase = Postgres;

// https://docs.rs/axum/latest/axum/extract/struct.State.html

#[derive(Clone, Debug)]
pub(crate) struct AppState {
    pub db_pool: Pool<DataBase>,
    pub extentions: Arc<RwLock<Extensions>>,

    #[cfg(feature = "enable_websocket_pubsub_sample")]
    pub pubsub: PubSubState,
}

impl AppState {
    #[cfg(feature = "dbtype_sqlite")]
    pub async fn new(config: &TomlConfig) -> Self {
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
        env::set_var("DATABASE_URL", config.database.url.as_str());
        AppState {
            db_pool: SqlitePoolOptions::new()
                .max_connections(config.database.max_connection)
                .connect(config.database.url.as_str())
                .await
                .expect("Unabled to Connect to Database"),

            extentions: Arc::new(RwLock::new(Extensions::default())),

            #[cfg(feature = "enable_websocket_pubsub_sample")]
            pubsub: PubSubState::new(),
        }
    }

    #[cfg(feature = "dbtype_postgres")]
    pub async fn new(config: &TomlConfig) -> Self {
        assert!(
            config.database.url.starts_with("postgres://"),
            "invalid database url scheme"
        );
        env::set_var("DATABASE_URL", config.database.url.as_str());
        AppState {
            db_pool: PgPoolOptions::new()
                .max_connections(config.database.max_connection)
                .connect(config.database.url.as_str())
                .await
                .expect("Unabled to Connect to Database"),

            extentions: Arc::new(RwLock::new(Extensions::default())),

            #[cfg(feature = "enable_websocket_pubsub_sample")]
            pubsub: PubSubState::new(),
        }
    }

    pub async fn migrate_database(&self) -> diagnostics::Result<()> {
        sqlx::migrate!("./migrations").run(&self.db_pool).await?;
        Ok(())
    }
}

// substate
// async fn handler(State(db_pool): State<Pool<DataBase>>) -> ...{}
impl FromRef<AppState> for Pool<DataBase> {
    fn from_ref(input: &AppState) -> Self {
        input.db_pool.clone()
    }
}
