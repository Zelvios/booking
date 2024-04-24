// @generated automatically by Diesel CLI.

diesel::table! {
    admin (id) {
        id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    booking (id) {
        id -> Int4,
        user_id -> Int4,
        schedule_id -> Int4,
    }
}

diesel::table! {
    booking_schedule (booking_id, schedule_id) {
        booking_id -> Int4,
        schedule_id -> Int4,
    }
}

diesel::table! {
    schedule (id) {
        id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        name -> Varchar,
        #[max_length = 254]
        email -> Varchar,
        #[max_length = 96]
        password -> Bpchar,
        created -> Timestamp,
    }
}

diesel::joinable!(admin -> user (user_id));
diesel::joinable!(booking -> schedule (schedule_id));
diesel::joinable!(booking -> user (user_id));
diesel::joinable!(booking_schedule -> booking (booking_id));
diesel::joinable!(booking_schedule -> schedule (schedule_id));
diesel::joinable!(schedule -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    admin,
    booking,
    booking_schedule,
    schedule,
    user,
);
