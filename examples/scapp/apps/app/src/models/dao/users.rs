use crate::models::schema::users;
use chrono::NaiveDateTime;
use diesel::{connection::LoadConnection, prelude::*};
use nidrs::{injectable, openapi::schema, AppResult, Inject};
use nidrs_diesel::{PoolManager, PostgresPoolManager};
use serde::{Deserialize, Serialize};

#[schema]
#[derive(Selectable, Queryable, Debug, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub unionid: String,
    pub openid: String,
    pub derive: String,
    pub blank: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct CreateUser {
    pub name: String,
    pub unionid: String,
    pub openid: String,
    pub derive: String,
}

#[injectable()]
pub struct UserEntity {
    pool: Inject<PostgresPoolManager>,
}

impl UserEntity {
    pub async fn all(&self) -> AppResult<Vec<User>> {
        self.pool
            .query(|mut conn| {
                users::table
                    .select(User::as_select())
                    .load::<User>(&mut conn)
            })
            .await
    }

    pub async fn create(&self, openid: String) -> AppResult<usize> {
        self.pool
            .query(|mut conn| {
                let new_user = CreateUser {
                    name: "".to_string(),
                    unionid: "default".to_string(),
                    openid,
                    derive: "".to_string(),
                };

                diesel::insert_into(users::table)
                    .values(&new_user)
                    .execute(&mut conn)
            })
            .await
    }

    pub async fn update(&self, id: i32, name: String) -> AppResult<usize> {
        self.pool
            .query(move |mut conn| {
                diesel::update(users::table.find(id))
                    .set(users::name.eq(name))
                    .execute(&mut conn)
            })
            .await
    }

    pub async fn find_by_id(&self, id: i32) -> AppResult<User> {
        self.pool
            .query(move |mut conn| {
                users::table
                    .find(id)
                    .select(User::as_select())
                    .first::<User>(&mut conn)
            })
            .await
    }

    pub async fn find_by_openid(&self, openid: String) -> AppResult<User> {
        self.pool
            .query(move |mut conn| {
                users::table
                    .filter(users::openid.eq(openid))
                    .select(User::as_select())
                    .first::<User>(&mut conn)
            })
            .await
    }

    pub async fn remove_by_id(&self, id: i32) -> AppResult<usize> {
        self.pool
            .query(move |mut conn| diesel::delete(users::table.find(id)).execute(&mut conn))
            .await
    }
}
