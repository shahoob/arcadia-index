INSERT INTO users (username, email, password_hash, registered_from_ip)
VALUES
    ('test', 'user1@example.com', '$argon2id$v=19$m=19456,t=2,p=1$s4XJtCUk9IrGgNsTfP6Ofw$ktoGbBEoFaVgdiTn19Gh9h45LjFiv7AUEL5KHhzm4d0', '192.168.1.1'),
    ('user2', 'user2@example.com', 'hashedpassword2', '192.168.1.2'),
    ('user3', 'user3@example.com', 'hashedpassword3', '192.168.1.3');

INSERT INTO artists (name, description, created_by_id)
VALUES
    ('Music Artist 1', 'A famous musician', 1);

INSERT INTO master_groups (name, created_by_id)
VALUES
    ('Master Group One', 1);

INSERT INTO title_groups (master_group_id, name, created_by_id, description, original_language, original_release_date, tags, country_from, external_links, content_type)
VALUES
    (NULL, 'Music Album 1', 1, 'An amazing album', 'English', '2020-01-01', ARRAY['rock', 'pop'], 'USA', ARRAY['https://example.com'], 'Music'),
    (NULL, 'Movie 1', 2, 'A great movie', 'English', '2021-06-15', ARRAY['action', 'thriller'], 'UK', ARRAY['https://example.com'], 'Movie');

INSERT INTO edition_groups (title_group_id, name, release_date, created_by_id, description, source, covers, external_links)
VALUES
    (1, 'Original Edition', '2020-02-01', 1, 'Original edition, nothing specific', 'CD', '{}', '{}'),
    (1, 'Original Edition', '2020-02-01', 1, 'Original edition, nothing specific', 'Web', '{}', '{}'),
    (1, 'Deluxe Edition', '2020-02-01', 1, 'Deluxe edition, additional tracks 3 and 4', 'Web', '{}', '{}'),
    (2, 'Original Edition', '2020-02-01', 1, 'Movie original edition back in the days', 'DVD5', '{}', '{}'),
    (2, 'Director''s Cut', '2021-07-01', 2, 'Extended version', 'Blu-Ray', '{}', '{}');

INSERT INTO torrents (edition_group_id, created_by_id, release_name, release_group, file_amount_per_type, file_list, mediainfo, container, size, audio_codec, audio_bitrate, video_codec, video_resolution)
VALUES
    (1, 1, 'music.album.1.cd.flac-groupx', 'GroupX', '{"flac": 2}'::jsonb, '["track1.flac", "track2.flac"]'::jsonb, 'Mediainfo data', 'flac', 500000000, 'flac', 1500, NULL, NULL),
    (1, 1, 'music.album.1.cd.mp3.320-groupx', 'GroupX', '{"mp3": 2}'::jsonb, '["track1.mp3", "track2.mp3"]'::jsonb, 'Mediainfo data', 'mp3', 300000000, 'mp3', 320, NULL, NULL),
    (2, 1, 'music.album.1.web.flac-groupx', 'GroupX', '{"flac": 2}'::jsonb, '["track1.flac", "track2.flac"]'::jsonb, 'Mediainfo data', 'flac', 500000000, 'flac', 1500, NULL, NULL),
    (2, 1, 'music.album.1.web.mp3.320-groupx', 'GroupX', '{"mp3": 2}'::jsonb, '["track1.mp3", "track2.mp3"]'::jsonb, 'Mediainfo data', 'mp3', 300000000, 'mp3', 320, NULL, NULL),
    (3, 1, 'music.album.1.web.flac-groupx', 'GroupX', '{"flac": 4}'::jsonb, '["track1.flac", "track2.flac", "track3.flac", "track4.flac"]'::jsonb, 'Mediainfo data', 'flac', 600000000, 'flac', 1500, NULL, NULL),
    (3, 1, 'music.album.1.web.mp3-groupx', 'GroupX', '{"mp3": 4}'::jsonb, '["track1.mp3", "track2.mp3", "track3.mp3", "track4.mp3"]'::jsonb, 'Mediainfo data', 'mp3', 400000000, 'mp3', 320, NULL, NULL),
    (4, 1, 'movie.1.web.720p.aac-flix.mkv', 'FLIX', '{"mkv": 1}'::jsonb, '["movie.mkv"]'::jsonb, 'Mediainfo data', 'mkv', 500000000, 'aac', 540, 'h264', '720p'),
    (5, 2, 'Movie_2021_DirectorsCut', 'GroupY', '{"mkv": 1}'::jsonb, '["movie.mkv"]'::jsonb, 'Mediainfo data', 'mkv', 8000000000, 'true-hd', 256, 'h265', '2160p'),
    (5, 2, 'Movie_2021_DirectorsCut', NULL, '{"mkv": 1}'::jsonb, '["movie.mkv"]'::jsonb, 'Mediainfo data', 'mkv', 4000000000, 'aac', 256, 'h264', '1080p'),
    (5, 2, 'Movie_2021_DirectorsCut', NULL, '{"mkv": 1}'::jsonb, '["movie.mkv"]'::jsonb, 'Mediainfo data', 'mkv', 3000000000, 'aac', 256, 'h264', '720p');

