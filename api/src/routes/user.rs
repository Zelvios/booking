use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/docker")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}