use actix_web::{post, get, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::db::models::user::User;
use crate::AppState;

#[derive(Deserialize)]
struct Query {
    username: String,
    password: String,
}

#[post("/user/create")]
pub async fn create_user(data: web::Data<AppState>, user: web::Json<Query>) -> impl Responder {
    let user = user.into_inner();

    let result = User::create(
        &mut *data.connection.lock().await,
        &user.username,
        &user.password,
    );
    match result {
        Ok(_) => HttpResponse::Ok().body("Successfully created new user."),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error creating new user: {e}")),
    }
}

#[get("/user/add")]
pub async fn add_user() -> impl Responder {
     HttpResponse::Ok().body("Successfully.")
}