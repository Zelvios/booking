use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::booking)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Booking {
    pub id: i32,
    pub user_id: i32,
    pub schedule_id: i32,
}