use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::booking_schedule)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BookingSchedule {
    pub booking_id: i32,
    pub schedule_id: i32,
}
