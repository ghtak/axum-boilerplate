use axum::extract::FromRef;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite, Pool};

use crate::{diagnostics, util::config::TomlConfig};

pub type DataBase = Sqlite;

// https://docs.rs/axum/latest/axum/extract/struct.State.html

#[derive(Clone, Debug)]
pub(crate) struct AppState {
    pub db_pool: Pool<DataBase>, //pub db_pool: PgPool,
}

impl AppState {
    pub async fn new(config: &TomlConfig) -> Self {
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
            /*
            db_pool : PgPoolOptions::new()
                .max_connections(config.database.max_connection)
                .connect(config.database.url().as_str())
                .await
                .expect("Unabled to Connect to Database")
            */
        }
    }

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
}

// substate
// async fn handler(State(db_pool): State<Pool<DataBase>>) -> ...{}
impl FromRef<AppState> for Pool<DataBase>{
    fn from_ref(input: &AppState) -> Self {
        input.db_pool.clone()
    }
}