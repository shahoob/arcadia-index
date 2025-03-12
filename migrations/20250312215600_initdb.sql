CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    username VARCHAR(15) NOT NULL,
    -- max length of 15 chars, change to your liking
    email TEXT UNIQUE NOT NULL,
    created_from_ip CIDR
);
