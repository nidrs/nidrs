use crate::models::schema::downlogs;
use chrono::NaiveDateTime;
use nidrs::{injectable, openapi::schema, AppResult, Inject};
use nidrs_diesel::{PoolManager, PostgresPoolManager};
use serde::{Deserialize, Serialize};

use diesel::{connection::LoadConnection, prelude::*};

#[schema]
#[derive(Selectable, Queryable, Debug, Serialize)]
#[diesel(table_name = downlogs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Downlog {
    pub id: i32,
    pub resource_id: i32,
    pub user_id: i32,
    pub status: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = downlogs)]
pub struct CreateDownlog {
    pub resource_id: i32,
    pub user_id: i32,
    pub status: i32,
}

#[injectable()]
pub struct DownlogEntity {
    pool: Inject<PostgresPoolManager>,
}

impl DownlogEntity {
    pub async fn all(&self) -> AppResult<Vec<Downlog>> {
        self.pool
            .query(|mut conn| downlogs::table.load::<Downlog>(&mut conn))
            .await
    }

    pub async fn create(&self, new_downlog: CreateDownlog) -> AppResult<usize> {
        self.pool
            .query(move |mut conn| {
                diesel::insert_into(downlogs::table)
                    .values(&new_downlog)
                    .execute(&mut conn)
            })
            .await
    }

    pub async fn update_status(&self, id: i32, status: i32) -> AppResult<usize> {
        self.pool
            .query(move |mut conn| {
                diesel::update(downlogs::table.find(id))
                    .set(downlogs::status.eq(status))
                    .execute(&mut conn)
            })
            .await
    }

    pub async fn find_by_id(&self, id: i32) -> AppResult<Downlog> {
        self.pool
            .query(move |mut conn| downlogs::table.find(id).first::<Downlog>(&mut conn))
            .await
    }

    pub async fn find_by_user_id(&self, user_id: i32) -> AppResult<Vec<Downlog>> {
        self.pool
            .query(move |mut conn| {
                downlogs::table
                    .filter(downlogs::user_id.eq(user_id))
                    .load::<Downlog>(&mut conn)
            })
            .await
    }

    pub async fn remove_by_id(&self, id: i32) -> AppResult<usize> {
        self.pool
            .query(move |mut conn| diesel::delete(downlogs::table.find(id)).execute(&mut conn))
            .await
    }
}
