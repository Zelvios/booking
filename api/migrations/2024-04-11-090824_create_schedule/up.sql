CREATE TABLE "schedule" (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES "user" (id)
);

CREATE
OR REPLACE FUNCTION add_schedule()
    RETURNS TRIGGER AS
$$
BEGIN
INSERT INTO "schedule" ("user_id")
VALUES (NEW."id");
RETURN NEW;
END;
$$
LANGUAGE plpgsql;

CREATE TRIGGER create_schedule
    AFTER INSERT
    ON "user"
    FOR EACH ROW EXECUTE FUNCTION add_schedule();