use crate::models::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use nidrs::{injectable, AppResult, Inject};
use nidrs_diesel::sqlite::SqlitePoolManager;
use serde::Serialize;

#[derive(Selectable, Queryable, Debug, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
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
    pool: Inject<SqlitePoolManager>,
}

impl UserEntity {
    pub async fn all(&self) -> AppResult<Vec<User>> {
        let mut conn = self.pool.get().await?;
        Ok(users::table.load::<User>(&mut conn).await.unwrap())
    }

    pub async fn all2(&self) -> AppResult<Vec<User>> {
        self.pool
            .query(|mut conn| async move {
                let users = users::table.load::<User>(&mut conn).await.unwrap();
                Ok(users)
            })
            .await
    }

    pub async fn create(&self, name: String) -> AppResult<usize> {
        let mut conn = self.pool.get().await?;

        let new_user = NewUser { name };

        Ok(diesel::insert_into(users::table).values(&new_user).execute(&mut conn).await.unwrap())
    }

    pub async fn update(&self, id: i32, name: String) -> AppResult<usize> {
        let mut conn = self.pool.get().await?;
        Ok(diesel::update(users::table.find(id)).set(users::name.eq(name)).execute(&mut conn).await.unwrap())
    }

    pub async fn find_by_id(&self, id: i32) -> AppResult<User> {
        let mut conn = self.pool.get().await?;
        Ok(users::table.find(id).first::<User>(&mut conn).await.unwrap())
    }

    pub async fn find_by_id2(&self, id: i32) -> AppResult<User> {
        self.pool
            .query(move |mut conn| async move {
                let user = users::table.find(id).first::<User>(&mut conn).await.unwrap();
                Ok(user)
            })
            .await
    }

    pub async fn remove_by_id(&self, id: i32) -> AppResult<usize> {
        let mut conn = self.pool.get().await?;
        Ok(diesel::delete(users::table.find(id)).execute(&mut conn).await.unwrap())
    }
}
