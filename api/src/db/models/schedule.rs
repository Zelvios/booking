use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::schedule)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Schedule {
    pub id: i32,
    pub user_id: i32,
}