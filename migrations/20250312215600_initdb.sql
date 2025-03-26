CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(20) UNIQUE NOT NULL,
    avatar TEXT,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    registered_from_ip VARCHAR(15) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description TEXT NOT NULL DEFAULT '',
    uploaded BIGINT NOT NULL DEFAULT 0,
    -- 1 byte downloaded
    downloaded BIGINT NOT NULL DEFAULT 1,
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
    invitations SMALLINT NOT NULL DEFAULT 0,
    bonus_points BIGINT NOT NULL DEFAULT 0
);
CREATE TABLE invitations (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP NOT NULL,
    invitation_key VARCHAR(50) NOT NULL,
    message TEXT NOT NULL,
    sender_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    receiver_email VARCHAR(255) NOT NULL,
    receiver_id INT REFERENCES users(id) ON DELETE
    SET NULL
);
CREATE TABLE artists (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    pictures TEXT [],
    created_by_id INT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    title_groups_amount INT NOT NULL DEFAULT 0,
    edition_groups_amount INT NOT NULL DEFAULT 0,
    torrents_amount INT NOT NULL DEFAULT 0,
    seeders_amount INT NOT NULL DEFAULT 0,
    leechers_amount INT NOT NULL DEFAULT 0,
    snatches_amount INT NOT NULL DEFAULT 0,
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE similar_artists (
    artist_1_id INT NOT NULL,
    artist_2_id INT NOT NULL,
    PRIMARY KEY (artist_1_id, artist_2_id),
    FOREIGN KEY (artist_1_id) REFERENCES artists(id) ON DELETE CASCADE,
    FOREIGN KEY (artist_2_id) REFERENCES artists(id) ON DELETE CASCADE
);
CREATE TABLE master_groups (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    -- name_aliases VARCHAR(255)[],
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by_id INT NOT NULL,
    -- description TEXT NOT NULL,
    -- original_language VARCHAR(50) NOT NULL,
    -- country_from VARCHAR(50) NOT NULL,
    -- tags VARCHAR(50)[] NOT NULL,
    -- category VARCHAR(25), -- should only be used for TV-Shows (scripted, reality-tv, etc.)
    -- covers TEXT[],
    -- banners TEXT[],
    -- fan_arts TEXT[],
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE
    SET NULL
);
CREATE TABLE similar_master_groups (
    group_1_id INT NOT NULL,
    group_2_id INT NOT NULL,
    PRIMARY KEY (group_1_id, group_2_id),
    FOREIGN KEY (group_1_id) REFERENCES master_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (group_2_id) REFERENCES master_groups(id) ON DELETE CASCADE
);
CREATE TABLE series (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    tags TEXT [],
    covers TEXT [],
    banners TEXT [],
    created_by_id INT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TYPE content_type_enum AS ENUM (
    'Movie',
    'TV-Show',
    'Music',
    'Software',
    'Book',
    'Collection'
);
CREATE TYPE category_enum AS ENUM (
    'Ep',
    'Album',
    'Single',
    'Soundtrack',
    'Anthology',
    'Compilation',
    'SingleCategory',
    'Remix',
    'Bootleg',
    'Mixtape',
    'ConcertRecording',
    'DjMix',
    'FeatureFilm',
    'ShortFilm',
    'Game',
    'Program',
    'Illustrated',
    'Periodical',
    'Book',
    'Article',
    'Manual'
);
CREATE TABLE title_groups (
    id SERIAL PRIMARY KEY,
    master_group_id INT,
    name TEXT NOT NULL,
    name_aliases TEXT [],
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by_id INT NOT NULL,
    description TEXT NOT NULL,
    original_language TEXT NOT NULL,
    original_release_date TIMESTAMP NOT NULL,
    tagline TEXT,
    tags VARCHAR(50) [] NOT NULL,
    country_from TEXT NOT NULL,
    covers TEXT [],
    external_links TEXT [] NOT NULL,
    embedded_links JSONB,
    category category_enum,
    content_type content_type_enum NOT NULL,
    public_ratings JSONB,
    series_id BIGINT,
    FOREIGN KEY (master_group_id) REFERENCES master_groups(id) ON DELETE
    SET NULL,
        FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE
    SET NULL,
        FOREIGN KEY (series_id) REFERENCES series(id) ON DELETE
    SET NULL
);
CREATE TABLE similar_title_groups (
    group_1_id INT NOT NULL,
    group_2_id INT NOT NULL,
    PRIMARY KEY (group_1_id, group_2_id),
    FOREIGN KEY (group_1_id) REFERENCES title_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (group_2_id) REFERENCES title_groups(id) ON DELETE CASCADE
);
CREATE TABLE affiliated_artists (
    title_group_id INT NOT NULL,
    artist_id INT NOT NULL,
    status VARCHAR(20) NOT NULL,
    nickname VARCHAR(255),
    created_by_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (title_group_id) REFERENCES title_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE
    SET NULL
);
-- for web: if it is a DL or a RIP should be specified at the torrent level
CREATE TYPE source_enum AS ENUM (
    'CD',
    'DVD5',
    'DVD9',
    'Vinyl',
    'Web',
    'Soundboard',
    'SACD',
    'DAT',
    'Cassette',
    'Blu-Ray',
    'LaserDisc',
    'HD-DVD',
    'HDTV',
    'PDTV',
    'TV',
    'VHS',
    'Mixed',
    'Physical-Book'
);
CREATE TABLE edition_groups (
    id SERIAL PRIMARY KEY,
    title_group_id INT NOT NULL,
    name TEXT NOT NULL,
    release_date TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by_id INT NOT NULL,
    description TEXT,
    distributor VARCHAR(255),
    covers TEXT [] NOT NULL,
    external_links TEXT [] NOT NULL,
    source source_enum NOT NULL,
    additional_information JSONB,
    FOREIGN KEY (title_group_id) REFERENCES title_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE
    SET NULL
);
CREATE TYPE audio_codec_enum AS ENUM (
    'mp2',
    'mp3',
    'aac',
    'ac3',
    'dts',
    'flac',
    'pcm',
    'true-hd',
    'opus',
    'dsd'
);
CREATE TYPE audio_bitrate_sampling_enum AS ENUM(
    '192',
    '256',
    '320',
    'APS (VBR)',
    'V2 (VBR)',
    'V1 (VBR)',
    'APX (VBR)',
    'V0 (VBR)',
    'Lossless',
    '24bit Lossless',
    'DSD64',
    'DSD128',
    'DSD256',
    'DSD512',
    'other'
);
CREATE TYPE video_codec_enum AS ENUM(
    'mpeg1',
    'mpeg2',
    'Xvid',
    'divX',
    'h264',
    'h265',
    'vc-1',
    'vp9',
    'BD50',
    'UHD100'
);
CREATE TYPE features_enum AS ENUM('HDR', 'DV', 'Commentary', 'Remux', '3D');
CREATE TABLE torrents (
    id SERIAL PRIMARY KEY,
    edition_group_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by_id INT NOT NULL,
    language VARCHAR(15),
    release_name VARCHAR(500),
    -- maybe change the size
    release_group VARCHAR(30),
    description TEXT,
    file_amount_per_type JSONB NOT NULL,
    uploaded_as_anonymous BOOLEAN NOT NULL DEFAULT FALSE,
    file_list JSONB NOT NULL,
    -- maybe change the size to the max length of a file name in a torrent
    mediainfo TEXT NOT NULL,
    trumpable TEXT,
    staff_checked BOOLEAN NOT NULL DEFAULT FALSE,
    container VARCHAR(4) NOT NULL,
    size BIGINT NOT NULL,
    -- in bytes
    FOREIGN KEY (edition_group_id) REFERENCES edition_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE
    SET NULL,
        -- audio
        duration INT,
        -- in seconds
        audio_codec audio_codec_enum,
        audio_bitrate INT,
        -- in kb/s, taken from mediainfo
        audio_bitrate_sampling audio_bitrate_sampling_enum,
        audio_channels VARCHAR(5),
        -- audio
        -- video
        video_codec video_codec_enum,
        features features_enum [],
        subtitle_languages VARCHAR(20) [],
        video_resolution VARCHAR(6) -- video
);

CREATE TABLE title_group_comments (
    id BIGSERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by_id INT NOT NULL,
    title_group_id INT NOT NULL,
    refers_to_torrent_id INT,
    answers_to_comment_id BIGINT,
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (title_group_id) REFERENCES title_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (refers_to_torrent_id) REFERENCES torrents(id) ON DELETE SET NULL,
    FOREIGN KEY (answers_to_comment_id) REFERENCES title_group_comments(id) ON DELETE SET NULL
);
CREATE TABLE torrent_requests (
    id BIGSERIAL PRIMARY KEY,
    title_group_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by_id INT NOT NULL,
    edition_name TEXT,
    release_group VARCHAR(20),
    description TEXT,
    language VARCHAR(25),
    container VARCHAR(5),
    bounty_upload BIGINT NOT NULL,
    bounty_bonus_points BIGINT NOT NULL,
    -- Audio
    audio_codec audio_codec_enum,
    audio_channels VARCHAR(8),
    -- Video
    video_codec video_codec_enum,
    features features_enum[],
    subtitle_languages TEXT[],
    video_resolution VARCHAR(6),
    FOREIGN KEY (title_group_id) REFERENCES title_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE torrent_request_votes(
    id BIGSERIAL PRIMARY KEY,
    torrent_request_id BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by_id INT NOT NULL,
    bounty_upload BIGINT NOT NULL DEFAULT 0,
    bounty_bonus_points BIGINT NOT NULL DEFAULT 0,
    FOREIGN KEY (torrent_request_id) REFERENCES torrent_requests(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE CASCADE
);
