DROP TABLE logging;

CREATE TABLE IF NOT EXISTS logging
(
    id         serial PRIMARY KEY       NOT NULL,
    service_id VARCHAR(200)             NOT NULL,
    log_type   VARCHAR(200)             NOT NULL,
    message    TEXT                     NOT NULL,
    logtime    TIMESTAMP WITH TIME ZONE NOT NULL,
    created    TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
