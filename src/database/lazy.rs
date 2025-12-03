use std::sync::Arc;

use deadpool_postgres::{Object, Pool, PoolError, Transaction};
use tokio::sync::Mutex;
use tokio_postgres::Error;

#[derive(Debug)]
pub enum ResultError {
    PoolError(deadpool_postgres::PoolError),
    QueryError(tokio_postgres::Error),
}

#[derive(Debug)]
pub enum ContextError<E> {
    Start(PoolError),
    User(E),
    Commit(Error),
    Rollback(Error),
}

pub struct LazyConn {
    pool: Pool,
    client: Option<Object>,
    transaction: Option<Transaction<'static>>,
}

pub type ArcLazyConn = Arc<Mutex<LazyConn>>;

impl LazyConn {
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

    pub async fn start_transaction(&mut self) -> Result<(), PoolError> {
        if self.transaction.is_none() {
            let client = self.get_client().await?;
            let tx = client.transaction().await?;
            self.transaction = Some(unsafe { std::mem::transmute(tx) });
        }
        Ok(())
    }

    pub async fn commit(&mut self) -> Result<(), Error> {
        if let Some(tx) = self.transaction.take() {
            tx.commit().await?;
        }
        Ok(())
    }

    pub async fn rollback(&mut self) -> Result<(), Error> {
        if let Some(tx) = self.transaction.take() {
            tx.rollback().await?;
        }
        Ok(())
    }

    pub async fn as_context<F, Fut, T, E>(&mut self, f: F) -> Result<T, ContextError<E>>
    where
        F: FnOnce(&mut Self) -> Fut,
        Fut: Future<Output = Result<T, E>>,
    {
        if let Err(e) = self.start_transaction().await {
            return Err(ContextError::Start(e));
        }

        match f(self).await {
            Ok(val) => {
                if let Err(e) = self.commit().await {
                    return Err(ContextError::Commit(e));
                }
                Ok(val)
            }
            Err(user_err) => {
                if let Err(rb_err) = self.rollback().await {
                    return Err(ContextError::Rollback(rb_err));
                }
                Err(ContextError::User(user_err))
            }
        }
    }
}
