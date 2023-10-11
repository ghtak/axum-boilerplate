use crate::{app_state::RedisPool, diagnostics};
use async_session::{MemoryStore, Session, SessionStore};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct RedisStore {
    redis_pool: RedisPool,
    prefix: String,
}

impl RedisStore {
    pub fn new(redis_pool: RedisPool) -> Self {
        Self {
            redis_pool,
            prefix: "session".to_owned(),
        }
    }

    pub async fn get_connection(
        &self,
    ) -> diagnostics::Result<bb8::PooledConnection<'_, bb8_redis::RedisConnectionManager>> {
        Ok(self.redis_pool.get().await?)
    }

    pub fn key(&self, key: impl AsRef<str>) -> String {
        format!("{}:{}", self.prefix, key.as_ref())
    }
}

#[async_trait]
impl SessionStore for RedisStore {
    async fn load_session(&self, cookie_value: String) -> Result<Option<Session>, anyhow::Error> {
        let id = Session::id_from_cookie_value(&cookie_value)?;
        let mut conn = self.get_connection().await?;
        let value: Option<String> = bb8_redis::redis::cmd("GET")
            .arg(self.key(id))
            .query_async(&mut *conn)
            .await?;
        match value {
            Some(v) => Ok(serde_json::from_str(&v)?),
            _ => Ok(None),
        }
    }

    async fn store_session(&self, session: Session) -> Result<Option<String>, anyhow::Error> {
        tracing::trace!("storing session by id `{}`", session.id());
        let id = session.id();
        let value = serde_json::to_string(&session)?;
        let mut conn = self.get_connection().await?;
        bb8_redis::redis::cmd("SET")
            .arg(self.key(id))
            .arg(value)
            .query_async(&mut *conn)
            .await?;

        session.reset_data_changed();
        Ok(session.into_cookie_value())
    }

    async fn destroy_session(&self, session: Session) -> Result<(), anyhow::Error> {
        tracing::trace!("destroying session by id `{}`", session.id());
        let id = session.id();
        let mut conn = self.get_connection().await?;
        bb8_redis::redis::cmd("DEL")
            .arg(self.key(id))
            .query_async(&mut *conn)
            .await?;
        Ok(())
    }

    async fn clear_store(&self) -> Result<(), anyhow::Error> {
        log::trace!("clearing memory store");
        let mut conn = self.get_connection().await?;
        let ids: Vec<String> = bb8_redis::redis::cmd("KEYS")
            .arg(format!("{}:*", self.prefix))
            .query_async(&mut *conn)
            .await?;
        bb8_redis::redis::cmd("DEL")
            .arg(ids)
            .query_async(&mut *conn)
            .await?;
        Ok(())
    }
}

pub(crate) type _SessionStoreImpl = MemoryStore;
pub(crate) type SessionStoreImpl = RedisStore;
