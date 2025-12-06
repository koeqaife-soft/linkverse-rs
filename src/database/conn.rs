use std::sync::Arc;

use deadpool_postgres::{Object, Pool, PoolError, Transaction};
use tokio::sync::Mutex;
use tracing::error;

use crate::utils::response::AppError;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ResultError {
    PoolError(deadpool_postgres::PoolError),
    QueryError(tokio_postgres::Error),
    AnyhowError(anyhow::Error),
}

// App errors (API)
impl From<ResultError> for AppError {
    fn from(err: ResultError) -> Self {
        error!("Database error: {:?}", err);
        AppError::Internal("INTERNAL_SERVER_ERROR".to_string())
    }
}

impl From<deadpool_postgres::PoolError> for AppError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        error!("Pool error: {:?}", err);
        AppError::Internal("INTERNAL_SERVER_ERROR".to_string())
    }
}

impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        error!("Tokio postgres error: {:?}", err);
        AppError::Internal("INTERNAL_SERVER_ERROR".to_string())
    }
}

// Result errors
impl From<anyhow::Error> for ResultError {
    fn from(err: anyhow::Error) -> Self {
        Self::AnyhowError(err)
    }
}

impl From<deadpool_postgres::PoolError> for ResultError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        Self::PoolError(err)
    }
}

impl From<tokio_postgres::Error> for ResultError {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::QueryError(err)
    }
}

pub struct LazyConn {
    pool: Arc<Pool>,
    client: Option<Object>,
}

pub type ArcLazyConn = Arc<Mutex<LazyConn>>;

impl LazyConn {
    pub fn new(pool: Arc<Pool>) -> ArcLazyConn {
        Arc::new(Mutex::new(Self { pool, client: None }))
    }

    pub async fn get_client(&mut self) -> Result<&mut Object, PoolError> {
        if self.client.is_none() {
            let conn = self.pool.get().await?;
            self.client = Some(conn);
        }
        Ok(self.client.as_mut().unwrap())
    }

    pub async fn transaction(&mut self) -> Result<Transaction<'_>, PoolError> {
        let client = self.get_client().await?;
        Ok(client.transaction().await?)
    }
}
