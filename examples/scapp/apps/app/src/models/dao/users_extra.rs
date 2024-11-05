use crate::models::schema::users_extra;
use chrono::NaiveDateTime;
use nidrs::{injectable, openapi::schema, AppResult, Inject};
use nidrs_diesel::{PoolManager, PostgresPoolManager};
use serde::{Deserialize, Serialize};

use diesel::{connection::LoadConnection, prelude::*};

#[schema]
#[derive(Selectable, Queryable, Debug, Serialize)]
#[diesel(table_name = users_extra)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserExtra {
    pub id: i32,
    pub user_id: i32,
    pub first_launch_path: String,
    pub first_launch_scene: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users_extra)]
pub struct CreateUserExtra {
    pub user_id: i32,
    pub first_launch_path: String,
    pub first_launch_scene: String,
}

#[injectable()]
pub struct UserExtraEntity {
    pool: Inject<PostgresPoolManager>,
}

impl UserExtraEntity {
    pub async fn all(&self) -> AppResult<Vec<UserExtra>> {
        self.pool
            .query(|mut conn| users_extra::table.load::<UserExtra>(&mut conn))
            .await
    }

    pub async fn create(
        &self,
        user_id: i32,
        first_launch_path: String,
        first_launch_scene: String,
    ) -> AppResult<usize> {
        self.pool
            .query(move |mut conn| {
                let new_user_extra = CreateUserExtra {
                    user_id,
                    first_launch_path,
                    first_launch_scene,
                };

                diesel::insert_into(users_extra::table)
                    .values(&new_user_extra)
                    .execute(&mut conn)
            })
            .await
    }

    pub async fn update(
        &self,
        id: i32,
        first_launch_path: String,
        first_launch_scene: String,
    ) -> AppResult<usize> {
        self.pool
            .query(move |mut conn| {
                diesel::update(users_extra::table.find(id))
                    .set((
                        users_extra::first_launch_path.eq(first_launch_path),
                        users_extra::first_launch_scene.eq(first_launch_scene),
                    ))
                    .execute(&mut conn)
            })
            .await
    }

    pub async fn find_by_id(&self, id: i32) -> AppResult<UserExtra> {
        self.pool
            .query(move |mut conn| users_extra::table.find(id).first::<UserExtra>(&mut conn))
            .await
    }

    pub async fn remove_by_id(&self, id: i32) -> AppResult<usize> {
        self.pool
            .query(move |mut conn| diesel::delete(users_extra::table.find(id)).execute(&mut conn))
            .await
    }
}
