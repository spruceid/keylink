-- Your SQL goes here
CREATE TABLE keys (
    user_id VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    jwk JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, name)
);
