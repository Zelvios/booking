use crate::db::schema::user;
use anyhow::Result;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use argon2::{Algorithm, Argon2, ParamsBuilder, password_hash::{rand_core::OsRng, PasswordHasher}, Version};
use argon2::password_hash::SaltString;
use crate::db::models::dberror::DatabaseError;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub created: SystemTime,
}
#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::user)]
pub struct New<'a> {
    pub name: &'a str,
    pub password: &'a str,
}

impl User {
    pub fn create(conn: &mut PgConnection, name: &str, password: &str) -> Result<Self, DatabaseError> {
        let hashed_password = Self::hash_password(password.as_bytes())?;

        let new_user = New { name, password: &hashed_password };

        let created_user = diesel::insert_into(user::table)
                                .values(&new_user)
                                .returning(Self::as_returning())
                                .get_result(conn)?;

        Ok(created_user)
    }
    
    fn hash_password(password: &[u8]) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);

        let params = ParamsBuilder::default().output_len(32).build()?;

        let argon2 = Argon2::new(Algorithm::Argon2d, Version::V0x13, params);
        let hash = argon2.hash_password(password, &salt)?;

        Ok(hash.to_string())
    }
}