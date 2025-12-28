use std::sync::Arc;

use deadpool_postgres::{Object, Pool, PoolError, Transaction};

pub struct LazyConn {
    pool: Arc<Pool>,
    client: Option<Object>,
}

impl LazyConn {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool, client: None }
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

#[macro_export]
macro_rules! get_conn {
    ($state:expr) => {
        LazyConn::new($state.db_pool.clone())
    };
}
