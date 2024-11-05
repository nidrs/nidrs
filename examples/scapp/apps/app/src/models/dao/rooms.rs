use crate::models::schema::rooms;
use chrono::NaiveDateTime;
use diesel::{connection::LoadConnection, prelude::*};
use nidrs::{injectable, openapi::schema, AppResult, Inject};
use nidrs_diesel::{PoolManager, PostgresPoolManager};
use serde::{Deserialize, Serialize};

#[schema]
#[derive(Selectable, Queryable, Debug, Serialize)]
#[diesel(table_name = rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub blank: bool,
    pub creator_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = rooms)]
pub struct CreateRoom {
    pub name: String,
    pub blank: bool,
    pub creator_id: i32,
}

#[injectable()]
pub struct RoomEntity {
    pool: Inject<PostgresPoolManager>,
}

impl RoomEntity {
    pub async fn all(&self) -> AppResult<Vec<Room>> {
        self.pool
            .query(|mut conn| rooms::table.load::<Room>(&mut conn))
            .await
    }

    pub async fn create(&self, new_room: CreateRoom) -> AppResult<usize> {
        self.pool
            .query(move |mut conn| {
                diesel::insert_into(rooms::table)
                    .values(&new_room)
                    .execute(&mut conn)
            })
            .await
    }

    pub async fn update(&self, id: i32, name: String) -> AppResult<usize> {
        self.pool
            .query(move |mut conn| {
                diesel::update(rooms::table.find(id))
                    .set(rooms::name.eq(name))
                    .execute(&mut conn)
            })
            .await
    }

    pub async fn find_by_id(&self, id: i32) -> AppResult<Room> {
        self.pool
            .query(move |mut conn| rooms::table.find(id).first::<Room>(&mut conn))
            .await
    }

    pub async fn remove_by_id(&self, id: i32) -> AppResult<usize> {
        self.pool
            .query(move |mut conn| diesel::delete(rooms::table.find(id)).execute(&mut conn))
            .await
    }
}
