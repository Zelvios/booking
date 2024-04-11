CREATE TABLE "user" (
    id       SERIAL      PRIMARY KEY,
    name     VARCHAR     NOT NULL,
    password VARCHAR(60) NOT NULL,
    created  TIMESTAMP   NOT NULL DEFAULT NOW()
);