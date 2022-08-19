use serde::{ Deserialize, Serialize };

use crate::api_error::ApiError;
use crate::db;
use crate::schema::*;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;
use diesel::prelude::*;

#[derive(Serialize, 
    Deserialize,
    Queryable, 
    Insertable)]
#[table_name = "player"]
pub struct Player {
    pub player_id: Uuid,
    pub player_name: String,
    pub player_race_id: i32,
    pub player_alignment_id: i32,
    pub background_id: i32,
    pub class_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Player {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;
        let users = player::table
            .load::<Player>(&conn)?;

        Ok(users)
    }

//    pub fn find(id: Uuid) -> Result<Self, ApiError> {
//        let conn = db::connection()?;
//
//        let user = player::table
//            .filter(player::id.eq(id))
//            .first(&conn)?;
//
//        Ok(user)
//    }

//    pub fn create(user: UserMessage) -> Result<Self, ApiError> {
//        let conn = db::connection()?;
//
//        let user = User::from(user);
//        let user = diesel::insert_into(user::table)
//            .values(user)
//            .get_result(&conn)?;
//
//        Ok(user)
//    }

//    pub fn update(id: Uuid, user: UserMessage) -> Result<Self, ApiError> {
//        let conn = db::connection()?;
//
//        let user = diesel::update(user::table)
//            .filter(user::id.eq(id))
//            .set(user)
//            .get_result(&conn)?;
//
//        Ok(user)
//    }

//    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
//        let conn = db::connection()?;
//
//        let res = diesel::delete(
//                player::table
//                    .filter(player::player_id.eq(id))
//            )
//            .execute(&conn)?;
//
//        Ok(res)
//    }
}
