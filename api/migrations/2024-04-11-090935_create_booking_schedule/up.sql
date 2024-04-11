CREATE TABLE "booking_schedule" (
    booking_id  INT REFERENCES "booking" (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    schedule_id INT REFERENCES "schedule" (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    CONSTRAINT booking_schedule_pkey PRIMARY KEY (booking_id, schedule_id)
);