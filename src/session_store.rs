use async_trait::async_trait;
use sqlx::MySqlPool;
use time::OffsetDateTime;
use tower_sessions_core::{
    session::{Id, Record},
    session_store, ExpiredDeletion, SessionStore,
};

/// A MySQL session store.
#[derive(Clone, Debug)]
pub struct MySqlStore {
    pool: MySqlPool,
}

impl MySqlStore {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ExpiredDeletion for MySqlStore {
    async fn delete_expired(
        &self,
    ) -> session_store::Result<()> {
        sqlx::query("DELETE FROM session WHERE expiry_date < utc_timestamp()")
            .execute(&self.pool)
            .await
            .map_err(SqlxStoreError::Sqlx)?;
        Ok(())
    }
}

#[async_trait]
impl SessionStore for MySqlStore {
    async fn save(
        &self,
        record: &Record,
    ) -> session_store::Result<()> {
        let query = r#"
            INSERT INTO session
              (id, data, expiry_date) VALUES (?, ?, ?)
            ON DUPLICATE KEY UPDATE
              data = VALUES(data),
              expiry_date = VALUES(expiry_date)
            "#;
        sqlx::query(query)
            .bind(&record.id.to_string())
            .bind(
                rmp_serde::to_vec(&record)
                    .map_err(SqlxStoreError::Encode)?,
            )
            .bind(record.expiry_date)
            .execute(&self.pool)
            .await
            .map_err(SqlxStoreError::Sqlx)?;

        Ok(())
    }

    async fn load(
        &self,
        session_id: &Id,
    ) -> session_store::Result<Option<Record>> {
        let data: Option<(Vec<u8>,)> = sqlx::query_as(
            r#"SELECT data FROM session WHERE id = ? AND expiry_date > ?"#
        )
            .bind(session_id.to_string())
            .bind(OffsetDateTime::now_utc())
            .fetch_optional(&self.pool)
            .await
            .map_err(SqlxStoreError::Sqlx)?;

        if let Some((data,)) = data {
            Ok(Some(
                rmp_serde::from_slice(&data)
                    .map_err(SqlxStoreError::Decode)?,
            ))
        } else {
            Ok(None)
        }
    }

    async fn delete(
        &self,
        session_id: &Id,
    ) -> session_store::Result<()> {
        sqlx::query(r#"delete from session where id = ?"#)
            .bind(&session_id.to_string())
            .execute(&self.pool)
            .await
            .map_err(SqlxStoreError::Sqlx)?;

        Ok(())
    }
}

// sqlx store error

/// An error type for SQLx stores.
#[derive(thiserror::Error, Debug)]
pub enum SqlxStoreError {
    /// A variant to map `sqlx` errors.
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    /// A variant to map `rmp_serde` encode
    /// errors.
    #[error(transparent)]
    Encode(#[from] rmp_serde::encode::Error),

    /// A variant to map `rmp_serde` decode
    /// errors.
    #[error(transparent)]
    Decode(#[from] rmp_serde::decode::Error),
}

impl From<SqlxStoreError> for session_store::Error {
    fn from(err: SqlxStoreError) -> Self {
        match err {
            SqlxStoreError::Sqlx(inner) => {
                session_store::Error::Backend(
                    inner.to_string(),
                )
            }
            SqlxStoreError::Decode(inner) => {
                session_store::Error::Decode(
                    inner.to_string(),
                )
            }
            SqlxStoreError::Encode(inner) => {
                session_store::Error::Encode(
                    inner.to_string(),
                )
            }
        }
    }
}
