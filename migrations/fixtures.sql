INSERT INTO users (username, email, password_hash, registered_from_ip, avatar)
VALUES
    ('picolo', 'user1@example.com', '$argon2id$v=19$m=19456,t=2,p=1$s4XJtCUk9IrGgNsTfP6Ofw$ktoGbBEoFaVgdiTn19Gh9h45LjFiv7AUEL5KHhzm4d0', '192.168.1.1','https://img.freepik.com/premium-vector/random-people-line-art-vector_567805-63.jpg'),
    ('waterbottle', 'user2@example.com', 'hashedpassword2', '192.168.1.2','https://i.pinimg.com/736x/a6/27/12/a6271204df8d387c3e614986c106f549.jpg'),
    ('coolguy', 'user3@example.com', 'hashedpassword3', '192.168.1.3','https://i.pinimg.com/474x/c1/5a/6c/c15a6c91515e22f6ea8b766f89c12f0c.jpg');

INSERT INTO "series" (name,description,created_by_id,covers,banners,tags) VALUES
    ('Astérix', 
    'Asterix (French: Astérix or Astérix le Gaulois [asteʁiks lə ɡolwa], "Asterix the Gaul"; also known as Asterix and Obelix in some adaptations or The Adventures of Asterix) is a French comic album series about a Gaulish village which, thanks to a magic potion that enhances strength, resists the forces of Julius Caesar''s Roman Republic Army in a nonhistorical telling of the time after the Gallic Wars. Many adventures take the titular hero Asterix and his friend Obelix to Rome and beyond.',
    1,
    ARRAY['https://i.pinimg.com/originals/90/33/f2/9033f2694de66ceb2edae762f0f4547b.jpg'],NULL, ARRAY['adventure']);

INSERT INTO master_groups (name, created_by_id)
VALUES
    ('Master Group One', 1);

INSERT INTO title_groups (master_group_id, name, created_by_id, description, original_language, original_release_date, tags, country_from, external_links, content_type, covers, category, series_id)
VALUES
    (NULL, 'Music Album 1', 1, 'An amazing album', 'English', '2020-01-01', ARRAY['rock', 'pop'], 'USA', ARRAY['https://example.com'], 'Music', ARRAY['https://archive.org/download/mbid-813d33df-ee11-4508-9bf7-98fcee7134b5/mbid-813d33df-ee11-4508-9bf7-98fcee7134b5-14497733661_thumb500.jpg', 'https://archive.org/download/mbid-c706dd61-a402-45d7-a9aa-f6b6916c76da/mbid-c706dd61-a402-45d7-a9aa-f6b6916c76da-41520333024_thumb500.jpg'], 'Album', NULL),
    (NULL, 'Movie 1', 2, 'A great movie', 'English', '2021-06-15', ARRAY['action', 'thriller'], 'UK', ARRAY['https://example.com'], 'Movie', ARRAY['https://image.tmdb.org/t/p/w780/qitnZcLP7C9DLRuPpmvZ7GiEjJN.jpg'], NULL, NULL),
    (NULL, 'Les misérables', 2, 'Interesting book', 'French', '1862-03-31', ARRAY['novel', 'historical.fiction'], 'France', ARRAY['https://en.wikipedia.org/wiki/Les_Mis%C3%A9rables', 'https://openlibrary.org/books/OL14082552M/Les_mis%C3%A9rables'], 'Book', ARRAY['https://cdn.kobo.com/book-images/a6bdd3f5-ba60-4ad3-8f6b-5f1427021961/1200/1200/False/les-miserables-305.jpg','https://m.media-amazon.com/images/I/613FCU-5u-L._SL500_.jpg'], 'Book', NULL),
    (NULL, 'Astérix le gaulois', 2, 'first book of the series !', 'French', '1961-09-01', ARRAY['action'], 'France', ARRAY['https://wikipedia.org'], 'Book', ARRAY['https://media.senscritique.com/media/000017019153/source_big/Asterix_le_Gaulois_Asterix_tome_1.jpg','https://www.hachette.fr/sites/default/files/images/livres/couv/9782012101654-T.jpg'], 'Illustrated', 1),
    (NULL, 'La Serpe d''or', 2, 'Astérix and Obélix travel to Lutetia.', 'French', '1962-10-01', 
        ARRAY['action', 'comedy'], 'France', ARRAY['https://wikipedia.org/wiki/La_Serpe_d%27or'], 
        'Book', 
        ARRAY[
            'https://www.hachette.fr/sites/default/files/images/livres/couv/9782012101340-T.jpg'
        ], 
        'Illustrated', 1),
    (NULL, 'Astérix et les Goths', 2, 'Astérix and Obélix encounter the Goths.', 'French', '1963-10-01', 
        ARRAY['adventure', 'historical'], 'France', ARRAY['https://wikipedia.org/wiki/Astérix_et_les_Goths'], 
        'Book', 
        ARRAY[
            'https://media.senscritique.com/media/000016957829/source_big/Asterix_et_les_Goths_Asterix_tome_3.jpg'
        ], 
        'Illustrated', 1),
    (NULL, 'Astérix Gladiateur', 2, 'Astérix and Obélix become gladiators in Rome.', 'French', '1964-10-01', 
        ARRAY['comedy', 'action'], 'France', ARRAY['https://wikipedia.org/wiki/Astérix_Gladiateur'], 
        'Book', 
        ARRAY[
            'https://i.pinimg.com/originals/63/67/a9/6367a9a4f55bbc39d6876025830ac758.jpg'
        ], 
        'Illustrated', 1),
    (NULL, 'Le Tour de Gaule d''Astérix', 2, 'Astérix and Obélix travel across Gaul.', 'French', '1965-10-01', 
        ARRAY['adventure', 'comedy'], 'France', ARRAY['https://wikipedia.org/wiki/Le_Tour_de_Gaule_d%27Astérix'], 
        'Book', 
        ARRAY[
            'https://media.senscritique.com/media/000011369244/source_big/Le_Tour_de_Gaule_d_Asterix_Asterix_tome_5.jpg'
        ], 
        'Illustrated', 1),
    (NULL, 'Wikipedia', 2, '', 'Multi', '2001-01-01', ARRAY['instructional'], 'France', ARRAY['https://en.wikipedia.org/'], 'Collection', ARRAY['https://upload.wikimedia.org/wikipedia/en/thumb/8/80/Wikipedia-logo-v2.svg/1200px-Wikipedia-logo-v2.svg.png'], NULL, NULL),
        (NULL, 'Astérix et les Normands', 2, 'Astérix and Obélix encounter the Vikings (Normans).', 'French', '1966-10-01', 
        ARRAY['adventure', 'comedy'], 'France', ARRAY['https://wikipedia.org/wiki/Astérix_et_les_Normands'], 
        'Book', 
        ARRAY[
            'http://media.senscritique.com/media/000011417184/source_big/Asterix_et_les_Normands_Asterix_tome_9.jpg'
        ], 
        'Illustrated', 1),
    (NULL, 'Astérix et le combat des chefs', 2, 'A challenge between village chiefs leads to a unique adventure.', 'French', '1967-10-01', 
        ARRAY['comedy', 'action'], 'France', ARRAY['https://wikipedia.org/wiki/Astérix_et_le_Combat_des_chefs'], 
        'Book', 
        ARRAY[
            'https://www.hachette.fr/sites/default/files/images/livres/couv/9782012101395-T.jpg'
        ], 
        'Illustrated', 1),
    (NULL, 'Astérix chez les Bretons', 2, 'Astérix and Obélix travel to Britain.', 'French', '1966-10-01', 
        ARRAY['adventure', 'historical', 'comedy'], 'France', ARRAY['https://wikipedia.org/wiki/Astérix_chez_les_Bretons'], 
        'Book', 
        ARRAY[
            'http://4.bp.blogspot.com/-AcmPdArdqtk/Uk7UC9rExaI/AAAAAAAAEH0/RqW6GMNg-Aw/s1600/1610.jpg'
        ], 
        'Illustrated', 1),
    (NULL, 'Astérix et Cléopâtre', 2, 'Astérix and Obélix help Cléopâtre build a palace in Egypt.', 'French', '1965-10-01', 
        ARRAY['adventure', 'historical', 'comedy'], 'France', ARRAY['https://wikipedia.org/wiki/Astérix_et_Cléopâtre'], 
        'Book', 
        ARRAY[
            'https://cdn1.booknode.com/book_cover/1360/full/asterix-tome-6-asterix-et-cleopatre-1360354.jpg'
        ], 
        'Illustrated', 1),
    (NULL, 'Astérix et les Légionnaires', 2, 'Astérix and Obélix join the Roman Legion.', 'French', '1967-10-01', 
        ARRAY['comedy', 'action'], 'France', ARRAY['https://wikipedia.org/wiki/Astérix_et_les_Légionnaires'], 
        'Book', 
        ARRAY[
            'https://www.hachette.fr/sites/default/files/images/livres/couv/9782012101425-T.jpg'
        ], 
        'Illustrated', 1);

INSERT INTO edition_groups (title_group_id, name, release_date, created_by_id, description, source, covers, external_links, additional_information)
VALUES
    (1, 'Original Edition', '2020-02-01', 1, 'Original edition, nothing specific', 'CD', '{}', '{}', NULL),
    (1, 'Original Edition', '2020-02-01', 1, 'Original edition, nothing specific', 'Web', '{}', '{}', NULL),
    (1, 'Deluxe Edition', '2020-02-01', 1, 'Deluxe edition, additional tracks 3 and 4', 'Web', '{}', '{}', NULL),
    (2, 'Original Edition', '2020-02-01', 1, 'Movie original edition back in the days', 'DVD5', '{}', '{}', NULL),
    (2, 'Director''s Cut', '2021-07-01', 2, 'Extended version', 'Blu-Ray', '{}', '{}', NULL),
    (3, 'Original edition', '1862-03-31', 2, 'the original edition of the book', 'Physical-Book', '{}', '{}', NULL),
    (3, 'Audiobook', '2005-01-01', 2, 'audiobook version', 'Web', '{}', '{}', NULL),
    (4, 'Original edition', '1961-09-01', 2, 'the original edition of the book', 'Physical-Book', '{}', '{}', NULL),
    (9, 'English version', '2021-01-01', 2, 'full dump of wikipedia''s English version', 'Web', '{}', '{}', NULL),
    (9, 'French version', '2021-01-01', 2, 'full dump of wikipedia''s French version', 'Web', '{}', '{}', NULL),
    (9, 'English version', '2025-01-01', 2, 'full dump of wikipedia''s English version', 'Web', '{}', '{}', NULL),
    (9, 'English version', '2025-01-01', 2, 'partial dump of wikipedia''s English version', 'Web', '{}', '{}', '{"date_from": "2020-01-01", "first_item": "A", "last_item": "M"}'::jsonb),
    (9, 'English version', '2025-01-01', 2, 'partial dump of wikipedia''s English version articles created in between 2020-01-01 and 2025-01-01, for the titles starting from N to Z', 'Web', '{}', '{}', '{"date_from": "2020-01-01", "first_item": "N", "last_item": "Z"}'::jsonb);

INSERT INTO torrents (edition_group_id, created_by_id, release_name, release_group, file_amount_per_type, file_list, mediainfo, container, size, audio_codec, audio_bitrate, video_codec, video_resolution, description)
VALUES
    (1, 1, 'music.album.1.cd.flac-groupx', 'GroupX', '{"flac": 2}'::jsonb, '{"files": [{"name": "01 - Aurora - Echo of My Shadow.flac","size": 18851309},{"name": "02 - Aurora - To Be Alright.flac","size": 27026040},{"name": "03 - Aurora - Your Blood.flac","size": 27520816},{"name": "04 - Aurora - The Conflict of the Mind.flac","size": 23263639},{"name": "05 - Aurora - Some Type of Skin.flac","size": 17919049},{"name": "06 - Aurora - The Essence.flac","size": 17171342},{"name": "07 - Aurora - Earthly Delights.flac","size": 20658200},{"name": "08 - Aurora - When The Dark Dresses Lightly.flac","size": 22438722},{"name": "09 - Aurora - A Soul With No King.flac","size": 28975261},{"name": "10 - Aurora - Dreams.flac","size": 20777186},{"name": "11 - Aurora featuring Ane Brun - My Name.flac","size": 21775646},{"name": "12 - Aurora - Do You Feel.flac","size": 22840257},{"name": "13 - Aurora - Starvation.flac","size": 22950931},{"name": "14 - Aurora - The Blade.flac","size": 32164508},{"name": "15 - Aurora - My Body Is Not Mine.flac","size": 28558966},{"name": "16 - Aurora - Invisible Wounds.flac","size": 24177756},{"name": "Aurora - What Happened to the Heart.cue","size": 3476},{"name": "Aurora - What Happened to the Heart.jpg","size": 75795},{"name": "Aurora - What Happened to the Heart.log","size": 19700},{"name": "Aurora - What Happened to the Heart.m3u","size": 1220}],"parent_folder": "Aurora - What Happened To The Heart (2024) {CD}"}'::jsonb, 'Mediainfo data', 'flac', 500000000, 'flac', 1500, NULL, NULL, 'rip of my own CD'),
    (1, 1, 'music.album.1.cd.mp3.320-groupx', 'GroupX', '{"mp3": 2}'::jsonb, '["track1.mp3", "track2.mp3"]'::jsonb, 'Mediainfo data', 'mp3', 300000000, 'mp3', 320, NULL, NULL, NULL),
    (2, 1, 'music.album.1.web.flac-groupx', 'GroupX', '{"flac": 2}'::jsonb, '["track1.flac", "track2.flac"]'::jsonb, 'Mediainfo data', 'flac', 500000000, 'flac', 1500, NULL, NULL, NULL),
    (2, 1, 'music.album.1.web.mp3.320-groupx', 'GroupX', '{"mp3": 2}'::jsonb, '["track1.mp3", "track2.mp3"]'::jsonb, 'Mediainfo data', 'mp3', 300000000, 'mp3', 320, NULL, NULL, NULL),
    (3, 1, 'music.album.1.web.flac-groupx', 'GroupX', '{"flac": 4}'::jsonb, '["track1.flac", "track2.flac", "track3.flac", "track4.flac"]'::jsonb, 'Mediainfo data', 'flac', 600000000, 'flac', 1500, NULL, NULL, NULL),
    (3, 1, 'music.album.1.web.mp3-groupx', 'GroupX', '{"mp3": 4}'::jsonb, '["track1.mp3", "track2.mp3", "track3.mp3", "track4.mp3"]'::jsonb, 'Mediainfo data', 'mp3', 400000000, 'mp3', 320, NULL, NULL, NULL),
    (4, 1, 'movie.1.web.720p.aac-flix.mkv', 'FLIX', '{"mkv": 1}'::jsonb, '["movie.mkv"]'::jsonb, 'Mediainfo data', 'mkv', 500000000, 'aac', 540, 'h264', '720p', NULL),
    (5, 2, 'Movie_2021_DirectorsCut', 'GroupY', '{"mkv": 1}'::jsonb, '["movie.mkv"]'::jsonb, 'Mediainfo data', 'mkv', 8000000000, 'true-hd', 256, 'h265', '2160p', NULL),
    (5, 2, 'Movie_2021_DirectorsCut', NULL, '{"mkv": 1}'::jsonb, '["movie.mkv"]'::jsonb, 
    E'General\nUnique ID                                : 29418572109828145996436348782017551176 (0x1621CF96CC430FC1E0C12600A4049B48)\nComplete name : Civil.War.2024.1080p.AMZN.WEB-DL.DDP5.1.Atmos.H.264-FLUX.mkv\nFormat                                   : Matroska\nFormat version                           : Version 4\nFile size                                : 6.30 GiB\nDuration                                 : 1 h 48 min\nOverall bit rate                         : 8 291 kb/s\nFrame rate                               : 23.976 FPS\nWriting application                      : mkvmerge v79.0 (\'Funeral Pyres\') 64-bit\nWriting library                          : libebml v1.4.4 + libmatroska v1.7.1\n\nVideo\nID                                       : 1\nFormat                                   : AVC\nFormat/Info                              : Advanced Video Codec\nFormat profile                           : High@L4\nFormat settings                          : CABAC / 4 Ref Frames\nFormat settings, CABAC                   : Yes\nFormat settings, Reference frames        : 4 frames\nCodec ID                                 : V_MPEG4/ISO/AVC\nDuration                                 : 1 h 48 min\nBit rate mode                            : Constant\nBit rate                                 : 7 521 kb/s\nNominal bit rate                         : 10 000 kb/s\nWidth                                    : 1 920 pixels\nHeight                                   : 1 040 pixels\nDisplay aspect ratio                     : 1.85:1\nFrame rate mode                          : Constant\nFrame rate                               : 23.976 (24000/1001) FPS\nColor space                              : YUV\nChroma subsampling                       : 4:2:0\nBit depth                                : 8 bits\nScan type                                : Progressive\nBits/(Pixel*Frame)                       : 0.157\nStream size                              : 5.71 GiB (91%)\nLanguage                                 : English (US)\nDefault                                  : Yes\nForced                                   : No\nColor range                              : Limited\nColor primaries                          : BT.709\nTransfer characteristics                 : BT.709\nMatrix coefficients                      : BT.709\n\nAudio\nID                                       : 2\nFormat                                   : E-AC-3 JOC\nFormat/Info                              : Enhanced AC-3 with Joint Object Coding\nCommercial name                          : Dolby Digital Plus with Dolby Atmos\nCodec ID                                 : A_EAC3\nDuration                                 : 1 h 48 min\nBit rate mode                            : Constant\nBit rate                                 : 768 kb/s\nChannel(s)                               : 6 channels\nChannel layout                           : L R C LFE Ls Rs\nSampling rate                            : 48.0 kHz\nFrame rate                               : 31.250 FPS (1536 SPF)\nCompression mode                         : Lossy\nDelay relative to video                  : 19 ms\nStream size                              : 597 MiB (9%)\nLanguage                                 : English (US)\nService kind                             : Complete Main\nDefault                                  : Yes\nForced                                   : No\nComplexity index                         : 16\nNumber of dynamic objects                : 15\nBed channel count                        : 1 channel\nBed channel configuration                : LFE\n\nText #1\nID                                       : 3\nFormat                                   : UTF-8\nCodec ID                                 : S_TEXT/UTF8\nCodec ID/Info                            : UTF-8 Plain Text\nDuration                                 : 1 h 39 min\nBit rate                                 : 34 b/s\nFrame rate                               : 0.165 FPS\nCount of elements                        : 989\nStream size                              : 25.4 KiB (0%)\nLanguage                                 : English (US)\nDefault                                  : No\nForced                                   : No\n\nText #2\nID                                       : 4\nFormat                                   : UTF-8\nCodec ID                                 : S_TEXT/UTF8\nCodec ID/Info                            : UTF-8 Plain Text\nDuration                                 : 1 h 40 min\nBit rate                                 : 39 b/s\nFrame rate                               : 0.189 FPS\nCount of elements                        : 1139\nStream size                              : 28.6 KiB (0%)\nTitle                                    : SDH\nLanguage                                 : English (US)\nDefault                                  : No\nForced                                   : No\n\nText #3\nID                                       : 5\nFormat                                   : UTF-8\nCodec ID                                 : S_TEXT/UTF8\nCodec ID/Info                            : UTF-8 Plain Text\nDuration                                 : 1 h 48 min\nBit rate                                 : 32 b/s\nFrame rate                               : 0.143 FPS\nCount of elements                        : 928\nStream size                              : 25.9 KiB (0%)\nTitle                                    : Latin American\nLanguage                                 : Spanish (Latin America)\nDefault                                  : No\nForced                                   : No\n\nMenu\n00:00:00.000                             : en:Chapter 01\n00:14:27.409                             : en:Chapter 02\n00:19:16.281                             : en:Chapter 03\n00:28:41.304                             : en:Chapter 04\n00:44:34.341                             : en:Chapter 05\n00:56:47.866                             : en:Chapter 06\n01:12:51.329                             : en:Chapter 07\n01:22:38.249                             : en:Chapter 08\n01:33:07.921                             : en:Chapter 09\n01:41:07.150                             : en:Chapter 10\n\nReportBy                                 : MediaInfoLib - v23.10', 
    'mkv', 4000000000, 'aac', 256, 'h264', '1080p', 'rip of my own blu-ray'),
    (5, 2, 'Movie_2021_DirectorsCut', NULL, '{"mkv": 1}'::jsonb, '["movie.mkv"]'::jsonb, 'Mediainfo data', 'mkv', 3000000000, 'aac', 256, 'h264', '720p', NULL),
    (6, 1, '', NULL, '{"pdf": 1}'::jsonb, '{"files": [{"name": "book.pdf","size": 18851309}]}'::jsonb, 'Mediainfo data', 'pdf', 500000000, NULL, NULL, NULL, NULL, 'scanned from my grandma''s book'),
    (7, 1, '', NULL, '{"m4b": 1}'::jsonb, '{"files": [{"name": "book.m4b","size": 58851309}]}'::jsonb, 'Mediainfo data', 'm4b', 800000000, 'mp3', 320, NULL, NULL, 'ripped from youtube'),
    (8, 1, '', NULL, '{"pdf": 1}'::jsonb, '{"files": [{"name": "book.pdf","size": 18851309}]}'::jsonb, 'Mediainfo data', 'pdf', 500000000, NULL, NULL, NULL, NULL, 'scanned from my grandma''s book'),
    (9, 1, '', NULL, '{"zip": 1}'::jsonb, '{"files": [{"name": "dump.zip","size": 80000000}]}'::jsonb, 'Mediainfo data', 'zip', 800000000, NULL, NULL, NULL, NULL, 'downloaded from the oficial wikipedia archive project'),
    (10, 1, '', NULL, '{"zip": 1}'::jsonb, '{"files": [{"name": "dump.zip","size": 81000000}]}'::jsonb, 'Mediainfo data', 'zip', 810000000, NULL, NULL, NULL, NULL, 'downloaded from the oficial wikipedia archive project'),
    (11, 1, '', NULL, '{"zip": 1}'::jsonb, '{"files": [{"name": "dump.zip","size": 100000000}]}'::jsonb, 'Mediainfo data', 'zip', 1000000000, NULL, NULL, NULL, NULL, 'downloaded from the oficial wikipedia archive project'),
    (12, 1, '', NULL, '{"zip": 1}'::jsonb, '{"files": [{"name": "dump.zip","size": 100000000}]}'::jsonb, 'Mediainfo data', 'zip', 1000000000, NULL, NULL, NULL, NULL, 'downloaded from the oficial wikipedia archive project'),
    (13, 1, '', NULL, '{"zip": 1}'::jsonb, '{"files": [{"name": "dump.zip","size": 100000000}]}'::jsonb, 'Mediainfo data', 'zip', 1000000000, NULL, NULL, NULL, NULL, 'downloaded from the oficial wikipedia archive project');


INSERT INTO artists (name, description, pictures, created_by_id) 
VALUES 
    ('Victor Hugo', 'description of Victor Hugo', ARRAY['https://upload.wikimedia.org/wikipedia/commons/thumb/e/e6/Victor_Hugo_by_%C3%89tienne_Carjat_1876_-_full.jpg/440px-Victor_Hugo_by_%C3%89tienne_Carjat_1876_-_full.jpg'],1),
    ('René Goscinny', 'description of René Goscinny', ARRAY['https://upload.wikimedia.org/wikipedia/commons/thumb/b/b6/Ren%C3%A9_Goscinny.jpg/500px-Ren%C3%A9_Goscinny.jpg'],1),
    ('Albert Uderzo', 'description of Albert Uderzo', ARRAY['https://upload.wikimedia.org/wikipedia/commons/thumb/9/9b/Albert_Uderzo_2012.jpg/500px-Albert_Uderzo_2012.jpg'],1);

INSERT INTO affiliated_artists (title_group_id, artist_id, status, nickname, created_by_id) 
VALUES
    (3, 1, 'Writer', NULL, 1),
    (4, 2, 'Writer', NULL, 1),
    (4, 3, 'Artist', NULL, 1);

INSERT INTO "title_group_comments" ("content", "created_at", "updated_at", "created_by_id", "title_group_id", "refers_to_torrent_id", "answers_to_comment_id") VALUES 
    ('Great book, I really enjoyed it, thanks for the upload !', '2025-03-24 10:47:09.630279', '2025-03-24 10:47:09.630279', 1, 3, NULL, NULL),
    ('Indeed ! +1', '2025-03-24 10:49:00.277003', '2025-03-24 10:49:00.277003', 2, 3, NULL, '1'),
    ('Thanks for the scanned version, good to have the original one archived',NOW(),NOW(), 3, 3, 12, NULL);

INSERT INTO "public"."torrent_requests" ("id", "title_group_id", "created_at", "updated_at", "created_by_id", "edition_name", "release_group", "description", "language", "container", "bounty_upload", "bounty_bonus_points", "audio_codec", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES 
    ('1', 4, '2025-03-25 21:52:10.458757', '2025-03-25 21:52:10.458757', 1, 'original edition', 'flix', 'always wanted to see this one', 'English', 'MKV', '20000', '500', 'aac', '5.1', 'h265', '{HDR}', '["English","French"]', '2160p'),
    ('2', 4, '2025-03-25 21:53:18.741246', '2025-03-25 21:53:18.741246', 1, 'original edition', 'flix', 'always wanted to see this one', 'English', 'MKV', '10000', '500', 'aac', '5.1', 'h264', '{Remux}', '["English","French"]', '1080p');
