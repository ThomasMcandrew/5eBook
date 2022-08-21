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
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
#[derive(
    Deserialize,
    Insertable,
    AsChangeset)]
#[table_name = "user"]
pub struct UserPlayer {
    pub uaername: String,
    pub password: String,
}

impl User {
    pub fn find(user_id: i32) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let user = user::table
            .filter(user::id.eq(user_id))
            .first(&conn)?;
        Ok(user)
    }
    
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;
        let users = user::table
            .load::<User>(&conn)?;
        Ok(users)
    }

    pub fn create(new_user : NewUser) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let user = diesel::insert_into(user::table)
            .values(new_user)
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn update(user_id: i32, 
            new_user : NewUser) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let user = diesel::update(user::table)
            .filter(user::id.eq(user_id))
            .set(new_user)
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn delete(user_id: i32) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
                user::table
                    .filter(user::id.eq(user_id))
            )
            .execute(&conn)?;
        Ok(res)
    }
}






