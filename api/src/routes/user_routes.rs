use crate::models::player::User;
use crate::models::player::NewUser;
use crate::api_error::ApiError;
use actix_web::{get, 
    post, 
    put, 
    delete, 
    web, 
    HttpResponse, 
    HttpServer, 
    Responder};

#[get("/user")]
pub async fn get_users() -> 
    Result<HttpResponse, ApiError> {
    let users = User::find_all()
        .expect("Failed to get objects from db");
    Ok(HttpResponse::Ok().json(users))
}

#[get("/user/{id}")]
pub async fn get_user(user_id: web::Path<i32>) -> 
        Result<HttpResponse, ApiError> {
    let user = User::find(user_id.into_inner())
        .expect("Failed to get user from db"); 
    Ok(HttpResponse::Ok().json(user))
} 

#[post("/user")]
pub async fn post_user(
        request : web::Json<NewUser>) -> 
            Result<HttpResponse, ApiError> {
    let db_val = User::create(request.into_inner())
        .expect("Failed saving to db");
    Ok(HttpResponse::Ok().json(db_val))
}

#[post("/user/login")]
pub async fn post_player_login(
        request : web::Json<NewPlayer>) -> 
            Result<HttpResponse, ApiError> {
     
    Ok(HttpResponse::Ok().json(""))
}


#[put("/user/{id}")]
pub async fn update_user(
        request: web::Json<NewUser>,
        user_id: web::Path<i32>
    ) -> Result<HttpResponse, ApiError> {
    let user = User::update(
        user_id.into_inner(),
        request.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/user/{id}")]
pub async fn delete_user(
        user_id: web::Path<i32>
    ) -> Result<HttpResponse, ApiError> {
    let num_deleted = User::delete(user_id.into_inner())?;
    Ok(HttpResponse::Ok().json(num_deleted))
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
    cfg.service(get_user);
    cfg.service(post_user);
    cfg.service(update_user);
    cfg.service(delete_user);
}
