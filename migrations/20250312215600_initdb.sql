CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    username VARCHAR(20) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    registered_from_ip VARCHAR(15) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description TEXT NOT NULL DEFAULT '',
    uploaded BIGINT NOT NULL DEFAULT 1,
    -- 1 byte uploaded
    downloaded BIGINT NOT NULL DEFAULT 0,
    ratio FLOAT NOT NULL DEFAULT 0.0,
    required_ratio FLOAT NOT NULL DEFAULT 0.0,
    last_seen TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    class VARCHAR(50) NOT NULL DEFAULT 'newbie',
    forum_posts INTEGER NOT NULL DEFAULT 0,
    forum_threads INTEGER NOT NULL DEFAULT 0,
    group_comments INTEGER NOT NULL DEFAULT 0,
    torrent_comments INTEGER NOT NULL DEFAULT 0,
    request_comments INTEGER NOT NULL DEFAULT 0,
    artist_comments BIGINT NOT NULL DEFAULT 0,
    seeding INTEGER NOT NULL DEFAULT 0,
    leeching INTEGER NOT NULL DEFAULT 0,
    snatched INTEGER NOT NULL DEFAULT 0,
    seeding_size BIGINT NOT NULL DEFAULT 0,
    requests_filled BIGINT NOT NULL DEFAULT 0,
    collages_started BIGINT NOT NULL DEFAULT 0,
    requests_voted BIGINT NOT NULL DEFAULT 0,
    average_seeding_time BIGINT NOT NULL DEFAULT 0,
    invited BIGINT NOT NULL DEFAULT 0,
    invites INT NOT NULL DEFAULT 0
);

CREATE TABLE invitations
(
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP NOT NULL,
    message TEXT NOT NULL,
    sender_id INT NOT NULL,
    receiver_id INT NOT NULL,
    CONSTRAINT fk_sender FOREIGN KEY (sender_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_receiver FOREIGN KEY (receiver_id) REFERENCES users(id) ON DELETE CASCADE
);