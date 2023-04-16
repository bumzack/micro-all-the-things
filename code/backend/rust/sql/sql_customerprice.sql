DROP TABLE customer_price;

CREATE TABLE IF NOT EXISTS customer_price
(
    id          serial PRIMARY KEY NOT NULL,
    customer_id INTEGER            NOT NULL,
    discount    REAL               NOT NULL,
    start_year  INTEGER            NOT NULL,
    end_year    INTEGER            NOT NULL,
    created     TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
