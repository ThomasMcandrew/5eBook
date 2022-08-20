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
#[derive(Serialize, 
    Deserialize)]
pub struct PlayerRequest {
    pub player_name: String,
    pub player_race_id: i32,
    pub player_alignment_id: i32,
    pub background_id: i32,
    pub class_id: i32,
}

impl Player {
    pub fn from_request(player: PlayerRequest) -> Player {
        Player {
            player_id: Uuid::new_v4(),
            player_name: player.player_name,
            player_race_id: player.player_race_id,
            player_alignment_id: player.player_alignment_id,
            background_id: player.background_id,
            class_id: player.class_id,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
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

    pub fn create(request: PlayerRequest) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let player = Player::from_request(request);
        let user = diesel::insert_into(player::table)
            .values(player)
            .get_result(&conn)?;

        Ok(user)
    }

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
