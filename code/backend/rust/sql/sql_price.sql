DROP TABLE price;

CREATE TABLE IF NOT EXISTS price
(
    id           serial PRIMARY KEY NOT NULL,
    movie_tconst VARCHAR(200)       NOT NULL,
    amount       REAL               NOT NULL,
    created      TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
