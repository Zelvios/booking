CREATE TABLE "booking" (
    id          SERIAL PRIMARY KEY,
    user_id     INT NOT NULL REFERENCES "user" (id),
    schedule_id INT NOT NULL REFERENCES "schedule" (id)
);