use serde::{ Deserialize, Serialize };

use crate::api_error::ApiError;
use crate::db;
use crate::schema::*;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::prelude::*;

#[derive(Serialize,
    Identifiable, 
    Queryable,
    Deserialize)]
#[table_name = "player"]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub race_id: i32,
    pub alignment_id: i32,
    pub background_id: i32,
    pub class_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
#[derive(
    Deserialize,
    Insertable,
    AsChangeset)]
#[table_name = "player"]
pub struct NewPlayer {
    pub name: String,
    pub race_id: i32,
    pub alignment_id: i32,
    pub background_id: i32,
    pub class_id: i32,
    pub user_id: i32,
}

impl Player {
    pub fn find(player_id: i32) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let user = player::table
            .filter(player::id.eq(player_id))
            .first(&conn)?;
        Ok(user)
    }
    
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;
        let users = player::table
            .load::<Player>(&conn)?;
        Ok(users)
    }

    pub fn create(new_player: NewPlayer) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let user = diesel::insert_into(player::table)
            .values(new_player)
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn update(player_id: i32, 
            new_player: NewPlayer) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let user = diesel::update(player::table)
            .filter(player::id.eq(player_id))
            .set(new_player)
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn delete(player_id: i32) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
                user::table
                    .filter(user::id.eq(player_id))
            )
            .execute(&conn)?;
        Ok(res)
    }
}
