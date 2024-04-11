use actix_web::{App, HttpServer};
use crate::db::establish_connection;
use crate::routes::user::hello;

mod db;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}