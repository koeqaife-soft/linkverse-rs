use std::sync::Arc;

use deadpool_postgres::{Object, Pool, PoolError, Transaction};
use tokio::sync::Mutex;
use tokio_postgres::Error;

#[derive(Debug)]
pub enum ResultError {
    PoolError(deadpool_postgres::PoolError),
    QueryError(tokio_postgres::Error),
}

pub struct LazyConn<'a> {
    pool: Pool,
    client: Option<Object>,
    transaction: Option<Transaction<'a>>,
}

pub type ArcLazyConn<'a> = Arc<Mutex<LazyConn<'a>>>;

impl LazyConn<'_> {
    pub fn new(pool: Pool) -> Arc<Self> {
        Arc::new(Self {
            pool,
            client: None,
            transaction: None,
        })
    }

    pub async fn get_client(&mut self) -> Result<&mut Object, PoolError> {
        if self.client.is_none() {
            let conn = self.pool.get().await?;
            self.client = Some(conn);
        }
        Ok(self.client.as_mut().unwrap())
    }

    pub async fn with_transaction<F, Fut, T>(&mut self, f: F) -> Result<T, ResultError>
    where
        F: FnOnce(&mut Transaction<'_>) -> Fut,
        Fut: Future<Output = Result<T, Error>>,
    {
        let client = self.get_client().await.map_err(ResultError::PoolError)?;
        let mut tx = client
            .transaction()
            .await
            .map_err(ResultError::QueryError)?;
        let res = f(&mut tx).await;
        if res.is_ok() {
            tx.commit().await.map_err(ResultError::QueryError)?;
        } else {
            tx.rollback().await.map_err(ResultError::QueryError)?;
        }
        Ok(res.map_err(ResultError::QueryError)?)
    }
}
