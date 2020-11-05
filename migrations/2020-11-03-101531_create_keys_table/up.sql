-- Your SQL goes here
CREATE TABLE keys (
    user VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    public_key BLOB NOT NULL,
    private_key BLOB NOT NULL,
    PRIMARY KEY (user, name)
);
