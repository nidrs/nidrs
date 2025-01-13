#[cfg(not(feature = "async"))]
pub mod driver {
    #[derive(Default)]
    pub enum ConnectionDriver {
        #[cfg(feature = "sqlite")]
        Sqlite(sqlite::SqlitePoolManager),

        #[cfg(feature = "mysql")]
        Mysql(mysql::MysqlPoolManager),

        #[cfg(feature = "postgres")]
        Postgres(postgres::PostgresPoolManager),

        #[default]
        None,
    }

    #[cfg(feature = "sqlite")]
    pub mod sqlite {
        use std::{marker::Send, sync::Mutex};

        use diesel::{
            r2d2::{ConnectionManager, Pool},
            QueryResult, SqliteConnection,
        };
        use nidrs::AppResult;
        use nidrs_extern::{
            anyhow,
            axum::{async_trait, http},
            tokio::task,
        };

        use crate::ConnectionDriver;

        use nidrs::injectable;

        type TConnection = SqliteConnection;

        #[injectable()]
        pub struct SqlitePoolManager {
            pub pool: Option<Mutex<Pool<ConnectionManager<TConnection>>>>,
        }

        impl SqlitePoolManager {
            pub fn new<T: Into<String>>(url: T) -> SqlitePoolManager {
                let manager: ConnectionManager<TConnection> = ConnectionManager::<TConnection>::new(url);
                // Refer to the `r2d2` documentation for more methods to use
                // when building a connection pool
                let pool: Pool<ConnectionManager<TConnection>> =
                    Pool::builder().test_on_check_out(true).build(manager).expect("Could not build connection pool");

                SqlitePoolManager { pool: Some(Mutex::new(pool)) }
            }

            pub fn get_pool(&self) -> &Option<Mutex<Pool<ConnectionManager<TConnection>>>> {
                &self.pool
            }

            pub fn get(&self) -> diesel::r2d2::PooledConnection<ConnectionManager<TConnection>> {
                let binding = self.get_pool().as_ref().unwrap();
                let pool = binding.lock().unwrap();
                pool.get().unwrap()
            }

            pub async fn query<F, R>(&self, f: F) -> AppResult<R>
            where
                F: FnOnce(diesel::r2d2::PooledConnection<ConnectionManager<TConnection>>) -> QueryResult<R> + Send + 'static,
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

        impl From<SqlitePoolManager> for ConnectionDriver {
            fn from(val: SqlitePoolManager) -> Self {
                ConnectionDriver::Sqlite(val)
            }
        }
    }

    #[cfg(feature = "mysql")]
    pub mod mysql {
        use std::{marker::Send, sync::Mutex};

        use diesel::{
            r2d2::{ConnectionManager, Pool},
            MysqlConnection, QueryResult,
        };
        use nidrs::AppResult;
        use nidrs_extern::{
            anyhow,
            axum::{async_trait, http},
            tokio::task,
        };

        use crate::ConnectionDriver;

        use nidrs::injectable;

        type TConnection = MysqlConnection;

        #[injectable()]
        pub struct MysqlPoolManager {
            pub pool: Option<Mutex<Pool<ConnectionManager<TConnection>>>>,
        }

        impl MysqlPoolManager {
            pub fn new<T: Into<String>>(url: T) -> MysqlPoolManager {
                let manager: ConnectionManager<TConnection> = ConnectionManager::<TConnection>::new(url);
                let pool: Pool<ConnectionManager<TConnection>> =
                    Pool::builder().test_on_check_out(true).build(manager).expect("Could not build connection pool");

                MysqlPoolManager { pool: Some(Mutex::new(pool)) }
            }

            pub fn get_pool(&self) -> &Option<Mutex<Pool<ConnectionManager<TConnection>>>> {
                &self.pool
            }

            pub fn get(&self) -> diesel::r2d2::PooledConnection<ConnectionManager<TConnection>> {
                let binding = self.get_pool().as_ref().unwrap();
                let pool = binding.lock().unwrap();
                pool.get().unwrap()
            }

            pub async fn query<F, R>(&self, f: F) -> AppResult<R>
            where
                F: FnOnce(diesel::r2d2::PooledConnection<ConnectionManager<TConnection>>) -> QueryResult<R> + Send + 'static,
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

        impl From<MysqlPoolManager> for ConnectionDriver {
            fn from(val: MysqlPoolManager) -> Self {
                ConnectionDriver::Mysql(val)
            }
        }
    }

    #[cfg(feature = "postgres")]
    pub mod postgres {
        use std::{marker::Send, sync::Mutex};

        use diesel::{
            r2d2::{ConnectionManager, Pool},
            PgConnection, QueryResult,
        };
        use nidrs::AppResult;
        use nidrs_extern::{
            anyhow,
            axum::{async_trait, http},
            tokio::task,
        };

        use crate::ConnectionDriver;

        use nidrs::injectable;

        type TConnection = PgConnection;

        #[injectable()]
        #[derive(Default)]
        pub struct PostgresPoolManager {
            pub pool: Option<Mutex<Pool<ConnectionManager<TConnection>>>>,
        }

        impl PostgresPoolManager {
            pub fn new<T: Into<String>>(url: T) -> PostgresPoolManager {
                let manager: ConnectionManager<TConnection> = ConnectionManager::<TConnection>::new(url);
                let pool: Pool<ConnectionManager<TConnection>> =
                    Pool::builder().test_on_check_out(true).build(manager).expect("Could not build connection pool");

                PostgresPoolManager { pool: Some(Mutex::new(pool)) }
            }

            pub fn get_pool(&self) -> &Option<Mutex<Pool<ConnectionManager<TConnection>>>> {
                &self.pool
            }

            pub fn get(&self) -> diesel::r2d2::PooledConnection<ConnectionManager<TConnection>> {
                let binding = self.get_pool().as_ref().unwrap();
                let pool = binding.lock().unwrap();
                pool.get().unwrap()
            }

            pub async fn query<F, R>(&self, f: F) -> AppResult<R>
            where
                F: FnOnce(diesel::r2d2::PooledConnection<ConnectionManager<TConnection>>) -> QueryResult<R> + Send + 'static,
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

        impl From<PostgresPoolManager> for ConnectionDriver {
            fn from(val: PostgresPoolManager) -> Self {
                ConnectionDriver::Postgres(val)
            }
        }
    }
}

#[cfg(feature = "async")]
pub mod driver {

    #[derive(Default)]
    pub enum ConnectionDriver {
        #[cfg(feature = "sqlite_async")]
        Sqlite(sqlite::SqlitePoolManager),

        #[cfg(feature = "mysql_async")]
        Mysql(mysql::MysqlPoolManager),

        // #[cfg(feature = "postgres_async")]
        // Postgres(postgres::PostgresPoolManager),
        #[default]
        None,
    }

    #[cfg(feature = "sqlite_async")]
    pub mod sqlite {
        use crate::ConnectionDriver;

        use diesel::{Connection, SqliteConnection};
        use diesel_async::{sync_connection_wrapper::SyncConnectionWrapper, AsyncConnection};
        use nidrs::injectable;

        type TConnection = SyncConnectionWrapper<SqliteConnection>;

        #[injectable()]
        pub struct SqlitePoolManager {
            pub url: String,
        }

        impl SqlitePoolManager {
            pub fn new<T: Into<String>>(url: T) -> SqlitePoolManager {
                SqlitePoolManager { url: url.into() }
            }

            pub async fn get(&self) -> TConnection {
                SyncConnectionWrapper::<SqliteConnection>::establish(&self.url).await.unwrap()
            }
        }

        impl From<SqlitePoolManager> for ConnectionDriver {
            fn from(val: SqlitePoolManager) -> Self {
                ConnectionDriver::Sqlite(val)
            }
        }
    }

    #[cfg(feature = "mysql_async")]
    pub mod mysql {
        use crate::ConnectionDriver;

        use diesel::QueryResult;
        use nidrs::{injectable, AppResult};

        use diesel_async::pooled_connection::AsyncDieselConnectionManager;
        use diesel_async::{pooled_connection::mobc, AsyncMysqlConnection};
        use nidrs_extern::tokio::sync::Mutex;

        type TConnection = AsyncMysqlConnection;

        #[injectable()]
        pub struct MysqlPoolManager {
            pub pool: Option<Mutex<mobc::Pool<TConnection>>>,
        }

        impl MysqlPoolManager {
            pub fn new<T: Into<String>>(url: T) -> MysqlPoolManager {
                let config = AsyncDieselConnectionManager::<TConnection>::new(url);
                let pool = mobc::Pool::new(config);
                MysqlPoolManager { pool: Some(Mutex::new(pool)) }
            }

            pub async fn get(&self) -> mobc::PooledConnection<TConnection> {
                let binding = self.pool.as_ref().unwrap();
                let pool = binding.lock().await;

                pool.get().await.unwrap()
            }

            pub async fn query<F, R>(&self, f: F) -> AppResult<R>
            where
                F: FnOnce(mobc::PooledConnection<TConnection>) -> QueryResult<R> + Send + 'static,
                R: Send + 'static,
            {
                let conn = self.get().await;
                let result = f(conn).unwrap();
                Ok(result)
            }
        }

        impl From<MysqlPoolManager> for ConnectionDriver {
            fn from(val: MysqlPoolManager) -> Self {
                ConnectionDriver::Mysql(val)
            }
        }
    }
}
