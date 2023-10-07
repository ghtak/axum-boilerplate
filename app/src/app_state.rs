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
    pub async fn create_tables(&self) -> diagnostics::Result<()> {
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS sample(
                id INTEGER PRIMARY KEY AUTOINCREMENT ,
                name text)"#,
        )
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }

    pub async fn new(config: &TomlConfig) -> Self {
        #[cfg(feature = "dbtype_sqlite")]
        if !Sqlite::database_exists("./sqlite.db")
            .await
            .unwrap_or(false)
        {
            println!("Creating database {}", "./sqlite.db");
            match Sqlite::create_database("./sqlite.db").await {
                Ok(_) => println!("Create db success"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("Database already exists");
        }

        AppState {
            db_pool: SqlitePoolOptions::new()
                .max_connections(config.database.max_connection)
                .connect("./sqlite.db")
                .await
                .expect("Unabled to Connect to Database"),
            extentions: Arc::new(RwLock::new(Extensions::default())),
            #[cfg(feature = "enable_websocket_pubsub_sample")]
            pubsub: PubSubState::new(),
        }
    }

    #[cfg(feature = "dbtype_postgres")]
    pub async fn new(config: &TomlConfig) -> Self {
        AppState {
            db_pool: PgPoolOptions::new()
                .max_connections(config.database.max_connection)
                .connect(config.database.url().as_str())
                .await
                .expect("Unabled to Connect to Database"),

            extentions: Arc::new(RwLock::new(Extensions::default())),

            #[cfg(feature = "enable_websocket_pubsub_sample")]
            pubsub: PubSubState::new(),
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
