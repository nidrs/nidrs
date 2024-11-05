use crate::models::schema::resources;
use chrono::NaiveDateTime;
use nidrs::{injectable, openapi::schema, AppResult, Inject};
use nidrs_diesel::{PoolManager, PostgresPoolManager};
use serde::{Deserialize, Serialize};

use diesel::{connection::LoadConnection, prelude::*};

#[schema]
#[derive(Selectable, Queryable, Debug, Serialize)]
#[diesel(table_name = resources)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Resource {
    pub id: i32,
    pub room_id: i32,
    pub name: String,
    pub size: i32,
    pub key: String,
    pub length: i32,
    pub creator_id: i32,
    pub down_count: i32,
    pub blank: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = resources)]
pub struct CreateResource {
    pub room_id: i32,
    pub name: String,
    pub size: i32,
    pub key: String,
    pub length: i32,
    pub creator_id: i32,
    pub blank: bool,
}

#[injectable()]
pub struct ResourceEntity {
    pool: Inject<PostgresPoolManager>,
}

impl ResourceEntity {
    pub async fn all(&self) -> AppResult<Vec<Resource>> {
        self.pool
            .query(|mut conn| resources::table.load::<Resource>(&mut conn))
            .await
    }

    pub async fn create(&self, new_resource: CreateResource) -> AppResult<usize> {
        self.pool
            .query(move |mut conn| {
                diesel::insert_into(resources::table)
                    .values(&new_resource)
                    .execute(&mut conn)
            })
            .await
    }

    pub async fn update(&self, id: i32, name: String) -> AppResult<usize> {
        self.pool
            .query(move |mut conn| {
                diesel::update(resources::table.find(id))
                    .set(resources::name.eq(name))
                    .execute(&mut conn)
            })
            .await
    }

    pub async fn find_by_id(&self, id: i32) -> AppResult<Resource> {
        self.pool
            .query(move |mut conn| resources::table.find(id).first::<Resource>(&mut conn))
            .await
    }

    pub async fn remove_by_id(&self, id: i32) -> AppResult<usize> {
        self.pool
            .query(move |mut conn| diesel::delete(resources::table.find(id)).execute(&mut conn))
            .await
    }
}
