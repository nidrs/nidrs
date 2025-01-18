use crate::models::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use nidrs::externs::serde;
use nidrs::{injectable, AppResult, Inject};
use nidrs_diesel::mysql::MysqlPoolManager;
use serde::Serialize;

#[derive(Selectable, Queryable, Debug, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
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
    pool: Inject<MysqlPoolManager>,
}

impl UserEntity {
    pub async fn all(&self) -> AppResult<Vec<User>> {
        self.pool.query(|mut conn| async move { users::table.load::<User>(&mut conn) }).await
    }

    pub async fn create(&self, name: String) -> AppResult<usize> {
        self.pool
            .query(|mut conn| async move {
                let new_user = NewUser { name };

                diesel::insert_into(users::table).values(&new_user).execute(&mut conn)
            })
            .await
    }

    pub async fn update(&self, id: u32, name: String) -> AppResult<usize> {
        self.pool.query(move |mut conn| async move { diesel::update(users::table.find(id)).set(users::name.eq(name)).execute(&mut conn) }).await
    }

    pub async fn find_by_id(&self, id: u32) -> AppResult<User> {
        self.pool.query(move |mut conn| async move { users::table.find(id).first::<User>(&mut conn) }).await
    }

    pub async fn remove_by_id(&self, id: u32) -> AppResult<usize> {
        self.pool.query(move |mut conn| async move { diesel::delete(users::table.find(id)).execute(&mut conn) }).await
    }
}
