pub mod models;
mod schema;

use diesel::pg::{PgConnection};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error connecting to {database_url}: {e}"))
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn init() -> PgConnection {
    let mut conn = establish_connection();
    conn.run_pending_migrations(MIGRATIONS).expect("failed to run migrations");
    
    conn
}