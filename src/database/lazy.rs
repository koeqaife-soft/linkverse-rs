use std::sync::Arc;

use deadpool_postgres::{Object, Pool, PoolError, Transaction};
use tokio::sync::{Mutex, MutexGuard};

#[derive(Debug)]
pub enum ResultError {
    PoolError(deadpool_postgres::PoolError),
    QueryError(tokio_postgres::Error),
    AnyhowError(anyhow::Error),
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
