use diesel::r2d2::ConnectionManager;

use diesel::r2d2::Pool;
use nidrs::injectable;

use std::sync::Mutex;

// type TConnection = SqliteConnection;

#[injectable()]
pub struct DbPoolManager<TConnection>
where
    TConnection: diesel::r2d2::R2D2Connection + 'static,
{
    pub pool: Option<Mutex<Pool<ConnectionManager<TConnection>>>>,
}

impl<TConnection> DbPoolManager<TConnection>
where
    TConnection: diesel::r2d2::R2D2Connection + 'static,
{
    pub fn new<T: Into<String>>(url: T) -> DbPoolManager<TConnection> {
        let manager: ConnectionManager<TConnection> = ConnectionManager::<TConnection>::new(url);
        // Refer to the `r2d2` documentation for more methods to use
        // when building a connection pool
        let pool: Pool<ConnectionManager<TConnection>> =
            Pool::builder().test_on_check_out(true).build(manager).expect("Could not build connection pool");

        DbPoolManager { pool: Some(Mutex::new(pool)) }
    }
}

impl<TConnection> DbPoolManager<TConnection>
where
    TConnection: diesel::r2d2::R2D2Connection + 'static,
{
    pub fn get_pool(&self) -> &Option<Mutex<Pool<ConnectionManager<TConnection>>>> {
        &self.pool
    }
}

// impl From<DbPoolManager> for ConnectionDriver {
//     fn from(val: DbPoolManager) -> Self {
//         ConnectionDriver::Sqlite(val)
//     }
// }
