use crate::models::player::Player;
use crate::models::player::NewPlayer;
use crate::api_error::ApiError;
use actix_web::{get, 
    post, 
    put, 
    delete, 
    web, 
    HttpResponse, 
    HttpServer, 
    Responder};

#[get("/player")]
pub async fn get_players() -> 
    Result<HttpResponse, ApiError> {
    let players = Player::find_all()
        .expect("Failed to get objects from db");
    Ok(HttpResponse::Ok().json(players))
}

#[get("/player/{id}")]
pub async fn get_player(player_id: web::Path<i32>) -> 
        Result<HttpResponse, ApiError> {
    let player = Player::find(player_id.into_inner())
        .expect("Failed to get user from db"); 
    Ok(HttpResponse::Ok().json(player))
} 

#[post("/player")]
pub async fn post_player(
        request : web::Json<NewPlayer>) -> 
            Result<HttpResponse, ApiError> {
    let db_val = Player::create(request.into_inner())
        .expect("Failed saving to db");
    Ok(HttpResponse::Ok().json(db_val))
}

#[put("/player/{id}")]
pub async fn update_player(
        request: web::Json<NewPlayer>,
        player_id: web::Path<i32>
    ) -> Result<HttpResponse, ApiError> {
    let player = Player::update(
        player_id.into_inner(),
        request.into_inner())?;
    Ok(HttpResponse::Ok().json(player))
}

#[delete("/player/{id}")]
pub async fn delete_player(
        player_id: web::Path<i32>
    ) -> Result<HttpResponse, ApiError> {
    let num_deleted = Player::delete(player_id.into_inner())?;
    Ok(HttpResponse::Ok().json(num_deleted))
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_players);
    cfg.service(get_player);
    cfg.service(post_player);
    cfg.service(update_player);
    cfg.service(delete_player);
}
