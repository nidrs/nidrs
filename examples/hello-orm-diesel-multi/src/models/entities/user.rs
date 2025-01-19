use crate::models::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use nidrs::externs::serde;
use nidrs::{injectable, AppResult, Inject};
use nidrs_diesel::AsyncQuery;
use serde::Serialize;

use nidrs_diesel::mysql::MysqlPoolManager as Mysql;
use nidrs_diesel::sqlite::SqlitePoolManager as Sqlite;

#[derive(Selectable, Queryable, Debug, Serialize)]
#[diesel(table_name = users)]
// #[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: u32,
    pub name: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
}

#[injectable()]
pub struct UserEntity {
    mysql_pool: Inject<Mysql>,
    sqlite_pool: Inject<Sqlite>,
}

impl UserEntity {
    pub async fn all(&self) -> AppResult<Vec<User>> {
        let mut conn = self.mysql_pool.get().await?;
        let result = users::table.load::<User>(&mut conn).await.unwrap();
        Ok(result)
    }

    pub async fn create(&self, name: String) -> AppResult<usize> {
        self.create_sqlite(name.clone()).await?;
        let mut conn = self.mysql_pool.get().await?;
        let new_user = NewUser { name };
        let result = diesel::insert_into(users::table).values(&new_user).execute(&mut conn).await.unwrap();
        Ok(result)
    }

    pub async fn create_sqlite(&self, name: String) -> AppResult<usize> {
        let mut conn = self.sqlite_pool.get().await?;
        let new_user = NewUser { name };
        let result = diesel::insert_into(users::table).values(&new_user).execute(&mut conn).await.unwrap();
        Ok(result)
    }

    pub async fn update(&self, id: u32, name: String) -> AppResult<usize> {
        let mut conn = self.mysql_pool.get().await?;
        let result = diesel::update(users::table.find(id)).set(users::name.eq(name)).execute(&mut conn).await.unwrap();
        Ok(result)
    }

    pub async fn find_by_id(&self, id: u32) -> AppResult<User> {
        let mut conn = self.mysql_pool.get().await?;
        let result = users::table.find(id).first::<User>(&mut conn).await.unwrap();
        Ok(result)
    }

    pub async fn remove_by_id(&self, id: u32) -> AppResult<usize> {
        let mut conn = self.mysql_pool.get().await?;
        let result = diesel::delete(users::table.find(id)).execute(&mut conn).await.unwrap();
        Ok(result)
    }
}
