DROP TABLE customer;

CREATE TABLE IF NOT EXISTS customer
(
    id         serial PRIMARY KEY  NOT NULL,
    first_name VARCHAR(200)        NOT NULL,
    last_name  VARCHAR(200)        NOT NULL,
    email      VARCHAR(200) UNIQUE NOT NULL,
    password   VARCHAR(200)        NOT NULL,
    created    TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

