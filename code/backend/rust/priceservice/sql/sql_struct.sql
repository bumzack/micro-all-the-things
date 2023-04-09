DROP TABLE prices;

CREATE TABLE IF NOT EXISTS prices
(
    id           serial PRIMARY KEY NOT NULL,
    movie_tconst VARCHAR(200)       NOT NULL,
    amount       MONEY              NOT NULL,
    created      TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
