use crate::routes::user::create_user;
use actix_web::{web, App, HttpServer};
use diesel::PgConnection;
use tokio::sync::Mutex;

mod db;
mod routes;

// This struct represents state
struct AppState {
    connection: Mutex<PgConnection>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                connection: Mutex::new(db::init()), // Initialises connection to database
            }))
            .service(create_user)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
