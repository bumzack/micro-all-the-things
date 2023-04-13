DROP TABLE authentication;

CREATE TABLE IF NOT EXISTS authentication
(
    id          serial PRIMARY KEY NOT NULL,
    customer_id INTEGER            NOT NULL,
    jwt         VARCHAR(255),
    logged_in   TIMESTAMP WITH TIME ZONE,
    logged_out  TIMESTAMP WITH TIME ZONE,
    created     TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

