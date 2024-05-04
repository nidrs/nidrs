use std::{marker::Send, sync::Mutex};

use diesel::{
    r2d2::{ConnectionManager, Pool},
    QueryResult, SqliteConnection,
};
use nidrs::{injectable, AppResult};
use nidrs_extern::{
    anyhow,
    axum::{async_trait, http},
    tokio::task,
};

#[injectable()]
#[derive(Default)]
pub struct SqlitePoolManager {
    pub pool: Option<Mutex<Pool<ConnectionManager<SqliteConnection>>>>,
}

impl SqlitePoolManager {
    pub fn new(url: String) -> SqlitePoolManager {
        let manager: ConnectionManager<SqliteConnection> = ConnectionManager::<SqliteConnection>::new(url);
        // Refer to the `r2d2` documentation for more methods to use
        // when building a connection pool
        let pool: Pool<ConnectionManager<SqliteConnection>> =
            Pool::builder().test_on_check_out(true).build(manager).expect("Could not build connection pool");

        SqlitePoolManager { pool: Some(Mutex::new(pool)) }
    }
}

impl PoolManager for SqlitePoolManager {
    type Connection = SqliteConnection;
    fn get_pool(&self) -> &Option<Mutex<Pool<ConnectionManager<SqliteConnection>>>> {
        &self.pool
    }
}

#[async_trait]
pub trait PoolManager {
    type Connection: diesel::r2d2::R2D2Connection + 'static;
    fn get_pool(&self) -> &Option<Mutex<Pool<ConnectionManager<Self::Connection>>>>;

    fn get(&self) -> diesel::r2d2::PooledConnection<ConnectionManager<Self::Connection>> {
        let binding = self.get_pool().as_ref().unwrap();
        let pool = binding.lock().unwrap();
        pool.get().unwrap()
    }

    async fn query<F, R>(&self, f: F) -> AppResult<R>
    where
        F: FnOnce(diesel::r2d2::PooledConnection<ConnectionManager<Self::Connection>>) -> QueryResult<R> + Send + 'static,
        R: Send + 'static,
    {
        let conn = self.get();

        let result = task::spawn_blocking(move || f(conn)).await?;

        if let Err(e) = result {
            return Err(nidrs::AppError::Exception(nidrs::Exception::new(http::StatusCode::INTERNAL_SERVER_ERROR, anyhow::Error::new(e))));
        }

        Ok(result.unwrap())
    }
}
