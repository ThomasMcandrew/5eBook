use actix_web::{web , post, Result};
use serde;
use crate::models::player::Player;
use std::fs::File;
use std::io::prelude::*;


pub async fn post_player(
        request : web::Json<Player>) 
            -> Result<String> {
    let player : Player = request.into_inner();
    let json = serde_json::to_string(&player)
        .expect("Failed to serialize Player");
    
    let mut file = File::create("players")?;
    
    //let mut old_data : String;
    //file.read_to_string(&old_data)
    //    .expect("Failed to read from file");
    file.write_all(json.as_bytes())?;
    Ok(format!("{}",json))
}
