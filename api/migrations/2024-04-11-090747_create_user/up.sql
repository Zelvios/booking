CREATE TABLE "user" (
    id       SERIAL      PRIMARY KEY,
    name     VARCHAR     NOT NULL,
    password CHAR(96)    NOT NULL,
    created  TIMESTAMP   NOT NULL DEFAULT NOW()
);