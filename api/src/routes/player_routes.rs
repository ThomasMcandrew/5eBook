use actix_web::{web, HttpResponse};
use crate::models::player::Player;
use crate::models::player::PlayerRequest;
use crate::api_error::ApiError;

pub async fn get_player() -> 
    Result<HttpResponse, ApiError> {
    let players = Player::find_all()
        .expect("Failed to get objects from db");
    Ok(HttpResponse::Ok().json(players))
}

pub async fn post_player(
        request : web::Json<PlayerRequest>) -> 
            Result<HttpResponse, ApiError> {
    let player = request.into_inner();
    
    let db_val = Player::create(player)
        .expect("Failed saving to db");
    Ok(HttpResponse::Ok().json(db_val))
}
