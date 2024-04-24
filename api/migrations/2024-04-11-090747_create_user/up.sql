CREATE TABLE "user" (
    id       SERIAL       PRIMARY KEY,
    name     VARCHAR      NOT NULL,
    email    VARCHAR(254) NOT NULL UNIQUE,
    password CHAR(96)     NOT NULL,
    created  TIMESTAMP    NOT NULL DEFAULT NOW()
);