CREATE TABLE "admin" (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES "user" (id)
);