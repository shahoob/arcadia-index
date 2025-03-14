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
    invitations SMALLINT NOT NULL DEFAULT 0
);

CREATE TABLE invitations
(
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP NOT NULL,
    invitation_key VARCHAR(50) NOT NULL,
    message TEXT NOT NULL,
    sender_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    receiver_email VARCHAR(255) NOT NULL,
    receiver_id INT REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE master_groups (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    name_aliases TEXT[],
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    created_by INT NOT NULL,
    description TEXT NOT NULL,
    original_language TEXT NOT NULL,
    country_from TEXT NOT NULL,
    covers TEXT[],
    banners TEXT[],
    fan_arts TEXT[],
    category INT NOT NULL,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE similar_master_groups (
    group_1 INT NOT NULL,
    group_2 INT NOT NULL,
    PRIMARY KEY (group_1, group_2),
    FOREIGN KEY (group_1) REFERENCES master_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (group_2) REFERENCES master_groups(id) ON DELETE CASCADE
);


CREATE TABLE title_groups (
    id SERIAL PRIMARY KEY,
    master_group INT,
    name TEXT NOT NULL,
    name_aliases TEXT[],
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    created_by INT NOT NULL,
    description TEXT NOT NULL,
    original_language TEXT NOT NULL,
    country_from TEXT NOT NULL,
    covers TEXT[],
    external_links TEXT[] NOT NULL,
    embedded_links JSONB,
    category INT NOT NULL,
    public_ratings JSONB,
    FOREIGN KEY (master_group) REFERENCES master_groups(id) ON DELETE SET NULL,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE similar_title_groups (
    group_1 INT NOT NULL,
    group_2 INT NOT NULL,
    PRIMARY KEY (group_1, group_2),
    FOREIGN KEY (group_1) REFERENCES title_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (group_2) REFERENCES title_groups(id) ON DELETE CASCADE
);

CREATE TABLE edition_groups (
    id SERIAL PRIMARY KEY,
    title_group INT NOT NULL,
    name TEXT NOT NULL,
    release_date TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    created_by INT NOT NULL,
    description TEXT,
    distributor BIGINT,
    cover TEXT[] NOT NULL,
    external_links TEXT[] NOT NULL,
    language TEXT,
    source TEXT NOT NULL,
    FOREIGN KEY (title_group) REFERENCES title_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE SET NULL
);
