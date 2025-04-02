INSERT INTO "public"."_sqlx_migrations" ("version", "description", "installed_on", "success", "checksum", "execution_time") VALUES ('20250312215600', 'initdb', '2025-03-30 16:24:53.548906+00', true, NULL, '41104523');
INSERT INTO "public"."affiliated_artists" ("title_group_id", "artist_id", "status", "nickname", "created_by_id", "created_at") VALUES (1, 1, 'singer', '', '1', '2025-03-30 17:39:40.616037');
INSERT INTO "public"."affiliated_artists" ("title_group_id", "artist_id", "status", "nickname", "created_by_id", "created_at") VALUES (2, 2, 'director, writer', '', '1', '2025-03-31 10:47:39.654048');
INSERT INTO "public"."affiliated_artists" ("title_group_id", "artist_id", "status", "nickname", "created_by_id", "created_at") VALUES (3, 3, 'author', '', '1', '2025-03-31 11:47:39.311712');
INSERT INTO "public"."artists" ("id", "name", "description", "pictures", "created_by_id", "created_at", "title_groups_amount", "edition_groups_amount", "torrents_amount", "seeders_amount", "leechers_amount", "snatches_amount") VALUES (1, 'The Beatles', '', '["https://www.rollingstone.com/wp-content/uploads/2018/06/rs_beatles01-2598d59b-a7fa-44db-93c6-6ebe6601eb8f.jpg"]', 1, '2025-03-30 17:39:13.568689', 0, 0, 0, 0, 0, 0);
INSERT INTO "public"."artists" ("id", "name", "description", "pictures", "created_by_id", "created_at", "title_groups_amount", "edition_groups_amount", "torrents_amount", "seeders_amount", "leechers_amount", "snatches_amount") VALUES (2, 'Charlie Chaplin', '', '["https://media.themoviedb.org/t/p/w600_and_h900_bestv2/lRI9Ky1SMOehn04OqhYJ5hQfgIK.jpg"]', 1, '2025-03-31 10:47:12.069969', 0, 0, 0, 0, 0, 0);
INSERT INTO "public"."artists" ("id", "name", "description", "pictures", "created_by_id", "created_at", "title_groups_amount", "edition_groups_amount", "torrents_amount", "seeders_amount", "leechers_amount", "snatches_amount") VALUES (3, 'Victor Hugo', '', '["https://upload.wikimedia.org/wikipedia/commons/thumb/e/e6/Victor_Hugo_by_%C3%89tienne_Carjat_1876_-_full.jpg/440px-Victor_Hugo_by_%C3%89tienne_Carjat_1876_-_full.jpg"]', 1, '2025-03-31 11:47:25.165927', 0, 0, 0, 0, 0, 0);
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (1, '1', 'red label Parlophone single', '1962-01-01 00:00:00', '2025-03-30 16:43:28.474757', '2025-03-30 16:43:28.474757', 1, 'Red label with silver print. Catalog number with 45 prefix. These labels have “PARLOPHONE” in large stylized letters at the top and the Parlophone £ logo at 3 o’clock.
The publishing year was printed on the label, as “RECORDING FIRST PUBLISHED 1962″.
This release has labels without “MADE IN GT. BRITAIN” printed below the Parlophone £ logo.
Matrix numbers: Side A: 7XCE 17144-1N, Side B: 7XCE 17145-1N.', 'Parlophone', '[""]', '["https://www.discogs.com/release/1789539-The-Beatles-Love-Me-Do"]', 'Vinyl', '{}');
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (2, '1', 'CDRX 4949', '1992-01-01 00:00:00', '2025-03-30 17:45:30.799015', '2025-03-30 17:45:30.799015', 1, 'Standard release in a CD single slim jewel case', '', '[""]', '["https://musicbrainz.org/release/695ede4f-fc2f-4dff-8a1c-bdd57645285a"]', 'CD', '{}');
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (3, '1', 'quobuz edition', '2015-03-18 00:00:00', '2025-03-31 09:45:45.799102', '2025-03-31 09:45:45.799102', 1, '2015 GynMusic Srl under license to Pirames International Srl 2015 GynMusic Srl under license to Pirames International Srl', 'qobuz', '[""]', '[""]', 'Web', '{}');
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (4, '2', 'Silent version', '1925-08-15 00:00:00', '2025-03-31 10:25:44.86285', '2025-03-31 10:25:44.86285', 1, NULL, '', '[""]', '[""]', 'Blu-Ray', '{}');
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (5, '2', 'Sound version', '1942-02-11 00:00:00', '2025-03-31 10:40:17.868567', '2025-03-31 10:40:17.868567', 1, 'This version has dialogues dubbed', '', '[""]', '[""]', 'Blu-Ray', '{}');
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (6, '2', 'Sound version digital restoration', '2012-02-15 00:00:00', '2025-03-31 10:43:37.252083', '2025-03-31 10:43:37.252083', 1, 'High-definition digital restoration of the 1942 sound version (2012).', '', '[""]', '[""]', 'Blu-Ray', '{}');
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (7, '3', 'Original edition', '1862-03-31 00:16:08', '2025-03-31 11:03:45.905456', '2025-03-31 11:03:45.905456', 1, '', '', '[""]', '[""]', 'Physical-Book', '{}');
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (8, '3', 'Revised edition', '2010-05-28 00:00:00', '2025-03-31 11:08:53.447285', '2025-03-31 11:08:53.447285', 1, 'Revised edition of the book, including footnotes to explain things about the context of the time.', 'Amazon', '[""]', '[""]', 'Web', '{}');
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (10, '3', 'CD Audiobook', '2012-02-02 00:00:00', '2025-03-31 11:43:42.916292', '2025-03-31 11:43:42.916292', 1, '', '', '[""]', '[""]', 'CD', '{}');
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (11, '4', 'Original edition', '1940-05-01 00:00:00', '2025-03-31 12:04:53.496183', '2025-03-31 12:04:53.496183', 1, '', '', '[""]', '[""]', 'Physical-Book', '{}');
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (12, '5', 'Original edition', '1940-05-01 00:00:00', '2025-03-31 12:15:38.583304', '2025-03-31 12:15:38.583304', 1, '', '', '[""]', '[""]', 'Physical-Book', '{}');
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (9, '3', 'Original edition', '2011-02-09 00:00:00', '2025-03-31 11:41:20.328981', '2025-03-31 11:41:20.328981', 1, 'Audiobook of the original version', 'Audible', '[""]', '[""]', 'Web', '{}');
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (13, '6', 'Original edition', '2016-12-10 00:00:00', '2025-03-31 13:39:26.541847', '2025-03-31 13:39:26.541847', 1, '', 'casefilepodcast', '[""]', '[""]', 'Web', '{"date_from":"2016-01-09","last_item":"Case 40","first_item":"Case 01"}');
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (15, '6', 'Original edition', '2018-12-22 00:00:00', '2025-03-31 13:52:36.241175', '2025-03-31 13:52:36.241175', 1, '', 'casefilepodcast', '[""]', '[""]', 'Web', '{"date_from":"2018-01-13","last_item":"Case 104","first_item":"Case 72"}');
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (14, '6', 'Original edition', '2017-12-16 00:00:00', '2025-03-31 13:47:36.883244', '2025-03-31 13:47:36.883244', 1, '', 'casefilepodcast', '[""]', '[""]', 'Web', '{"date_from":"2017-01-07","last_item":"Case 71","first_item":"Case 41"}');
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (16, '7', 'Last edition', '2024-03-04 00:00:00', '2025-03-31 14:06:37.867433', '2025-03-31 14:06:37.867433', 1, 'Built by github workflow', 'github', '[""]', '[""]', 'Web', '{}');
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (18, '8', 'Extras', '2004-03-04 00:00:00', '2025-03-31 18:49:20.421903', '2025-03-31 18:49:20.421903', 1, 'Contains deleted scenes and behind the scenes', '', '[""]', '[""]', 'DVD9', '{}');
INSERT INTO "public"."edition_groups" ("id", "title_group_id", "name", "release_date", "created_at", "updated_at", "created_by_id", "description", "distributor", "covers", "external_links", "source", "additional_information") VALUES (17, '8', 'Season 1', '2004-03-04 00:00:00', '2025-03-31 18:39:35.622903', '2025-03-31 18:39:35.622903', 1, '', 'Amazon', '[""]', '[""]', 'Web', '{}');
INSERT INTO "public"."series" ("id", "name", "description", "tags", "covers", "banners", "created_by_id", "created_at", "updated_at") VALUES ('1', 'Skyman (Columbia Comics)', 'The Skyman is a fictional comic book superhero that appeared stories during the Golden Age of Comic Books. Created by writer Gardner Fox and artist Ogden Whitney, the character first appeared in the Columbia Comics omnibus title Big Shot Comics #1 (May 1940).[1] He is unrelated to the DC Comics character. The Skyman was Allan Turner, who was raised by his uncle to become "outstanding in mind and body". A brilliant scientist, he had no superpowers but did have a flying wing-shaped airplane, dubbed the Wing, that flew by the power of Earth''s magnetic poles. With this and money inherited from his late uncle''s will, he fought crime. In 1944, he acquired an "Icarus-Cape", a huge pair of wings which allowed him to fly without an airplane.[2] His love interest was detective Fawn Carroll.', '["adventure"]', '["http://1.bp.blogspot.com/-S4D2YF5z7Aw/UZX-wE8_IPI/AAAAAAAAAO8/X2IdMSuaIl8/s1600/Big+Shot+-+Vintage+Comic+Wallpaper+01.jpg"]', NULL, 1, '2025-03-31 12:09:50.769068', '2025-03-31 12:09:50.769068');
INSERT INTO "public"."series" ("id", "name", "description", "tags", "covers", "banners", "created_by_id", "created_at", "updated_at") VALUES ('2', 'Casefile True Crime', 'Casefile is an award-winning true-crime podcast that presents unforgettable stories in a professionally produced audio format. What started in 2016 as a one-person side project has grown to include an entire team based across multiple continents. Our episodes delve deep into the circumstances, investigations and trials of both solved and unsolved cases from all over the world. Casefile has hundreds of millions of downloads and consistently ranks highly across podcasting charts. Discover why everyone from Rolling Stone to Time magazine is calling it a must-listen experience. ', '["crime"]', '["https://casefilepodcast.com/wp-content/uploads/2020/07/casefile_icon_web.jpg"]', NULL, 1, '2025-03-31 13:50:44.733697', '2025-03-31 13:50:44.733697');
INSERT INTO "public"."series" ("id", "name", "description", "tags", "covers", "banners", "created_by_id", "created_at", "updated_at") VALUES ('3', 'Pimp my ride', 'Xzibit and the good people at West Coast Customs make people''s cars go from dirt to pimped in this 30 minute series. A pimped out car is usually worth $20,000 to $30,000. This season, the guys are back in full effect, transforming more rides and more lives. As usual, Xzibit--Pimp Master of Ceremonies himself--will be pulling drivers off the road who are guilty of committing heinous vehicular style crimes and giving their wheels the ultimate pimp-over. Want MTV To Pimp Out Your Ride? Read below for info on how to get that done. Wanna Pimp Out Your Ride? Does your car need some major work? Are you embarrassed to pick up your date because of your ride? Scared to roll up to that job interview because of your bucket? Fear no more - MTV wants to give you the hottest car on the block. We''re gonna take that hooptie and pimp it out. And we ain''t just talkin a paint job either. We''ll hook you up with the hottest ride on the street with all the hot accessories.', '["reality"]', '["https://artworks.thetvdb.com/banners/posters/74210-1.jpg"]', NULL, 1, '2025-03-31 18:46:26.670925', '2025-03-31 18:46:26.670925');
INSERT INTO "public"."title_group_comments" ("id", "content", "created_at", "updated_at", "created_by_id", "title_group_id", "refers_to_torrent_id", "answers_to_comment_id") VALUES ('1', 'Great single, nice to see that it''s now in the public domain', '2025-03-30 18:03:49.188365', '2025-03-30 18:03:49.188365', 1, '1', NULL, NULL);
INSERT INTO "public"."title_groups" ("id", "master_group_id", "name", "name_aliases", "created_at", "updated_at", "created_by_id", "description", "original_language", "original_release_date", "tagline", "tags", "country_from", "covers", "external_links", "embedded_links", "category", "content_type", "public_ratings", "series_id") VALUES ('1', NULL, 'Love Me Do / P.S. I Love You', '[]', '2025-03-30 16:35:06.418293', '2025-03-30 16:35:06.418293', 1, 'Tracklist
A - Love Me Do
B - P.S. I Love You', 'English', '1962-01-01 00:00:00', NULL, '["rock","pop"]', 'UK', '["https://ia903406.us.archive.org/16/items/mbid-20e0bad7-bfbf-4f18-b0b3-8549dfcef6f3/mbid-20e0bad7-bfbf-4f18-b0b3-8549dfcef6f3-2190513301.jpg"]', '["https://musicbrainz.org/release-group/5db85281-934d-36e5-865c-1922ad82a948","https://www.discogs.com/master/1154826-The-Beatles-Love-Me-Do"]', NULL, 'Single', 'Music', NULL, NULL);
INSERT INTO "public"."title_groups" ("id", "master_group_id", "name", "name_aliases", "created_at", "updated_at", "created_by_id", "description", "original_language", "original_release_date", "tagline", "tags", "country_from", "covers", "external_links", "embedded_links", "category", "content_type", "public_ratings", "series_id") VALUES ('2', NULL, 'The Gold Rush', '[]', '2025-03-31 10:22:23.744818', '2025-03-31 10:22:23.744818', 1, 'A gold prospector in Alaska struggles to survive the elements and win the heart of a dance hall girl.', 'English', '1925-08-15 00:00:00', NULL, '["adventure","comedy","drama"]', 'USA', '["https://image.tmdb.org/t/p/w1280/eQRFo1qwRREYwj47Yoe1PisgOle.jpg"]', '["https://www.themoviedb.org/movie/962-the-gold-rush","https://www.imdb.com/title/tt0015864/"]', NULL, 'FeatureFilm', 'Movie', NULL, NULL);
INSERT INTO "public"."title_groups" ("id", "master_group_id", "name", "name_aliases", "created_at", "updated_at", "created_by_id", "description", "original_language", "original_release_date", "tagline", "tags", "country_from", "covers", "external_links", "embedded_links", "category", "content_type", "public_ratings", "series_id") VALUES ('3', NULL, 'Les Misérables', '[]', '2025-03-31 11:02:49.707845', '2025-03-31 11:02:49.707845', 1, 'Les Misérables (/leɪ ˌmɪzəˈrɑːb(əl), -blə/,[4] French: [le mizeʁabl]) is a French epic historical novel by Victor Hugo, first published on 31 March 1862, that is considered one of the greatest novels of the 19th century. Les Misérables has been popularized through numerous adaptations for film, television, and the stage, including a musical. ', 'French', '1862-03-31 00:16:08', NULL, '["drama","historical"]', 'France', '["https://francetoday.com/wp-content/uploads/2022/03/51JEItnoKFL.jpg"]', '["https://openlibrary.org/works/OL1063588W/Les_Mis%C3%A9rables"]', NULL, 'Book', 'Book', NULL, NULL);
INSERT INTO "public"."title_groups" ("id", "master_group_id", "name", "name_aliases", "created_at", "updated_at", "created_by_id", "description", "original_language", "original_release_date", "tagline", "tags", "country_from", "covers", "external_links", "embedded_links", "category", "content_type", "public_ratings", "series_id") VALUES ('4', NULL, 'Skyman No. 1', '[]', '2025-03-31 12:02:07.049479', '2025-03-31 12:02:07.049479', 1, 'Cover by Ogden Whitney. Stories and art by Boody Rogers, Gardner Fox, Ogden Whitney, Paul Dean, Fred Schwab and Mart Bailey. Columbia Comics launches a solo book featuring aviator hero Skyman, one of the stars of Big Shot Comics. Orphaned by a plane crash and raised by his uncle to be superhuman, Allen Turner uses his experimental aircraft The Wing to fight crime as Skyman. Sparky Watts encounters The World''s Strongest Puppy, in a typically wacky Boody Rogers tale. In a story that illustrates its times, Skyman battles saboteur Red Signet and his gang, who are trying to force isolationist America into joining World War II. Plus a Face story, and an ad for the never-produced Skyman daily comic strip. Introducing the Skyman; The Paralyzing Ray; The Skyman Encounters Kidnappers; Sparky Watts; Jibby Jones; Mortimer the Monk; Saboteurs; The Red Signet; The Face: The Orphan Asylum; War Games. 64 pages, Full Color. Cover price $0.10. ', 'English', '1940-05-01 00:00:00', NULL, '["superhero"]', 'USA', '["https://media.mycomicshop.com/n_iv/600/837159.jpg"]', '["https://en.wikipedia.org/wiki/Skyman_(Columbia_Comics)"]', NULL, 'Illustrated', 'Book', NULL, '1');
INSERT INTO "public"."title_groups" ("id", "master_group_id", "name", "name_aliases", "created_at", "updated_at", "created_by_id", "description", "original_language", "original_release_date", "tagline", "tags", "country_from", "covers", "external_links", "embedded_links", "category", "content_type", "public_ratings", "series_id") VALUES ('5', NULL, 'Skyman No. 2', '[]', '2025-03-31 12:15:20.060021', '2025-03-31 12:15:20.060021', 1, 'Cover by Ogden Whitney. Stories and art by Gardner Fox, Ogden Whitney, Fred Schwab and Frank Tinsley. A solo book featuring aviator hero Skyman, one of the stars of Columbia''s Big Shot Comics. Skyman''s attacks on Axis military bases makes him their number one target. Aviator hero Yankee Doodle shows a group of kids how to fly, in a semi-educational strip - but Rusty''s redheaded, beautiful older sister doesn''t think that''s such a great idea. Skyman investigates a deadly green fog that is attacking fishermen at sea. Skyman; Mike the Mascot; Yankee Doodle; Jibby Jones. 68 pages, Full Color. Cover price $0.10. ', 'English', '1940-05-01 00:00:00', NULL, '["adventure"]', 'USA', '["https://media.mycomicshop.com/n_iv/600/1050423.jpg"]', '[""]', NULL, 'Illustrated', 'Book', NULL, '1');
INSERT INTO "public"."title_groups" ("id", "master_group_id", "name", "name_aliases", "created_at", "updated_at", "created_by_id", "description", "original_language", "original_release_date", "tagline", "tags", "country_from", "covers", "external_links", "embedded_links", "category", "content_type", "public_ratings", "series_id") VALUES ('7', NULL, 'Yuzu', '[]', '2025-03-31 14:05:02.320893', '2025-03-31 14:05:02.320893', 1, 'Yuzu is an experimental open-source emulator for the Nintendo Switch from the creators of Citra. It is written in C++ with portability in mind, with builds actively maintained for Windows and Linux. The emulator currently can play various commercial titles and homebrew applications with varying degrees of success.

Yuzu is no longer developed or available as of March 4, 2024, following a settlement with Nintendo.', '', '2018-01-14 00:00:00', NULL, '["emulator"]', '', '["https://androidgram.com/wp-content/uploads/2022/04/emulator-yuzu.jpg"]', '["https://en.wikipedia.org/wiki/Yuzu_(emulator)"]', NULL, 'Program', 'Software', NULL, NULL);
INSERT INTO "public"."title_groups" ("id", "master_group_id", "name", "name_aliases", "created_at", "updated_at", "created_by_id", "description", "original_language", "original_release_date", "tagline", "tags", "country_from", "covers", "external_links", "embedded_links", "category", "content_type", "public_ratings", "series_id") VALUES ('8', NULL, 'Season 1', '[]', '2025-03-31 18:35:58.583204', '2025-03-31 18:35:58.583204', 1, 'Season 1 of the series', 'English', '2004-03-04 00:00:00', NULL, '["reality"]', 'USA', '["https://thetvdb.com/banners/seasons/26075-1.jpg"]', '["https://thetvdb.com/series/pimp-my-ride"]', NULL, NULL, 'TV-Show', NULL, '3');
INSERT INTO "public"."title_groups" ("id", "master_group_id", "name", "name_aliases", "created_at", "updated_at", "created_by_id", "description", "original_language", "original_release_date", "tagline", "tags", "country_from", "covers", "external_links", "embedded_links", "category", "content_type", "public_ratings", "series_id") VALUES ('6', NULL, 'Casefile True Crime', '[]', '2025-03-31 13:38:28.755554', '2025-03-31 13:38:28.755554', 1, '
Casefile is an award-winning true-crime podcast that presents unforgettable stories in a professionally produced audio format. What started in 2016 as a one-person side project has grown to include an entire team based across multiple continents.

Our episodes delve deep into the circumstances, investigations and trials of both solved and unsolved cases from all over the world. Casefile has hundreds of millions of downloads and consistently ranks highly across podcasting charts. Discover why everyone from Rolling Stone to Time magazine is calling it a must-listen experience.
', 'English', '2016-01-09 00:00:00', NULL, '["crime"]', 'USA', '["https://casefilepodcast.com/wp-content/uploads/2020/07/casefile_icon_web.jpg"]', '["https://en.wikipedia.org/wiki/Casefile"]', NULL, 'Other', 'Collection', NULL, '2');
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (2, 1, '2025-03-30 17:31:55.998504', '2025-03-30 17:31:55.998504', 1, 'English', 'The Beatles - Love Me Do - P.S. I Love You (Parlophone Single) [16-48]', '', 'sox resample of [existing torrent]
scans resized', '{"png":6,"txt":1,"flac":2}', false, '{"files":[{"name":"01 Love Me Do.flac","size":9123619},{"name":"02 P.S. I Love You.flac","size":7866920},{"name":"Scans/Both A.png","size":6222163},{"name":"Scans/Both B.png","size":6199786},{"name":"Scans/Record A.png","size":3917306},{"name":"Scans/Record B.png","size":3942248},{"name":"Scans/Sleeve A.png","size":5648922},{"name":"Scans/Sleeve B.png","size":5659186},{"name":"lineage.txt","size":492}],"parent_folder":"The Beatles - Love Me Do - P.S. I Love You (Parlophone Single) [16-48]"}', 'none', '', false, 'FLAC', '48580642', NULL, 'flac', NULL, 'Lossless', NULL, NULL, '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (1, 1, '2025-03-30 16:44:41.458969', '2025-03-30 16:44:41.458969', 1, NULL, 'The Beatles - Love Me Do - P.S. I Love You (Parlophone Single) [24-96]', '', 'Full scans included

Lineage:

Original red label Parlophone single > Numark TTXUSB (45.11 RPM) > Shure M97XE/Jico SAS > WAV > Click Repair, conversion to mono, EQ and normalization


A notable needledrop since no tape source exists for the "Ringo version" of Love Me Do - all later releases are ultimately sourced from the original red label Parlophone single
Some official releases are even needledrops of needledrops!

This copy is far from perfect condition, but hopefully you find it listenable.', '{"png":6,"txt":1,"flac":2}', false, '{"files":[{"name":"01 Love Me Do.flac","size":28507764},{"name":"02 P.S. I Love You.flac","size":24491889},{"name":"Notes.txt","size":492},{"name":"Scans/Both A.png","size":118490649},{"name":"Scans/Both B.png","size":117217894},{"name":"Scans/Record A.png","size":94433739},{"name":"Scans/Record B.png","size":93792174},{"name":"Scans/Sleeve A.png","size":112202145},{"name":"Scans/Sleeve B.png","size":112577343}],"parent_folder":"The Beatles - Love Me Do - P.S. I Love You (Parlophone Single) [24-96]"}', 'none', '', false, 'FLAC', '701714089', NULL, 'flac', 650, '24bit Lossless', NULL, NULL, '{Booklet}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (5, 3, '2025-03-31 09:46:30.39388', '2025-03-31 09:46:30.39388', 1, NULL, 'Love Me Do / PS. I Love You', '', '', '{"cue":1,"log":1,"flac":3}', false, '{"files":[{"name":"01 Love Me Do.flac","size":11268945},{"name":"02 P.S. I Love You.flac","size":9505300},{"name":"03 Love Me Do (original single version).flac","size":11238335},{"name":"Love Me Do (Single).cue","size":848},{"name":"Love Me Do (Single).log","size":3852}],"parent_folder":"The Beatles - Love Me Do (Single) (1992) [FLAC]"}', 'none', '', false, 'FLAC', '32017280', NULL, 'flac', NULL, 'Lossless', NULL, NULL, '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (4, 2, '2025-03-30 18:02:52.525114', '2025-03-30 18:02:52.525114', 1, NULL, 'The Beatles - Love Me Do (Single) (1992) [320]', '', 'Transcoded from flac torrent here (same edition ofc)', '{"mp3":3}', false, '{"files":[{"name":"01 Love Me Do.mp3","size":5741017},{"name":"02 P.S. I Love You.mp3","size":4980341},{"name":"03 Love Me Do (original single version).mp3","size":5689869}],"parent_folder":"The Beatles - Love Me Do (Single) (1992) [320]"}', 'none', '', false, 'MP3', '16411227', NULL, 'mp3', NULL, '320', NULL, NULL, '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (7, 4, '2025-03-31 10:37:49.789648', '2025-03-31 10:37:49.789648', 1, 'English', 'The Gold Rush 1925 1080p Blu-Ray DTS x264-WiHD', 'WiHD', '', '{"mkv":1}', false, '{"files":[{"name":"The Gold Rush 1925 1080p Blu-Ray DTS x264-WiHD.mkv","size":11427139815}],"parent_folder":""}', 'General
Unique ID                                : 204121119036637567174132304195361219226 (0x99904C2BCEB75751B1296D8ACE24A29A)
Complete name                            : The Gold Rush 1925 1080p Blu-Ray DTS x264-WiHD.mkv
Format                                   : Matroska
Format version                           : Version 2
File size                                : 10.6 GiB
Duration                                 : 1 h 29 min
Overall bit rate                         : 17.1 Mb/s
Movie name                               : The Gold Rush 1925 1080p Blu-Ray DTS x264-WiHD
Encoded date                             : UTC 2012-06-20 00:57:00
Writing application                      : mkvmerge v5.6.0 (''Kenya Kane'') built on May 27 2012 16:44:04
Writing library                          : libebml v1.2.3 + libmatroska v1.3.0

Video
ID                                       : 1
Format                                   : AVC
Format/Info                              : Advanced Video Codec
Format profile                           : High@L4.1
Format settings                          : CABAC / 5 Ref Frames
Format settings, CABAC                   : Yes
Format settings, ReFrames                : 5 frames
Codec ID                                 : V_MPEG4/ISO/AVC
Duration                                 : 1 h 29 min
Bit rate                                 : 15.2 Mb/s
Width                                    : 1 440 pixels
Height                                   : 1 080 pixels
Display aspect ratio                     : 4:3
Frame rate mode                          : Constant
Frame rate                               : 23.976 (24000/1001) FPS
Color space                              : YUV
Chroma subsampling                       : 4:2:0
Bit depth                                : 8 bits
Scan type                                : Progressive
Bits/(Pixel*Frame)                       : 0.408
Stream size                              : 9.49 GiB (89%)
Title                                    : The Gold Rush 1925 1080p @ 15.3 Mb/s
Writing library                          : x264 core 125 r2200 999b753
Encoding settings                        : cabac=1 / ref=5 / deblock=1:-3:-3 / analyse=0x3:0x113 / me=umh / subme=11 / psy=1 / psy_rd=1.00:0.01 / mixed_ref=1 / me_range=64 / chroma_me=1 / trellis=2 / 8x8dct=1 / cqm=0 / deadzone=6,6 / fast_pskip=0 / chroma_qp_offset=-3 / threads=12 / lookahead_threads=2 / sliced_threads=0 / nr=0 / decimate=0 / interlaced=0 / bluray_compat=0 / constrained_intra=0 / bframes=5 / b_pyramid=2 / b_adapt=2 / b_bias=0 / direct=3 / weightb=1 / open_gop=1 / weightp=2 / keyint=240 / keyint_min=23 / scenecut=40 / intra_refresh=0 / rc_lookahead=240 / rc=crf / mbtree=1 / crf=18.2 / qcomp=0.60 / qpmin=10 / qpmax=51 / qpstep=4 / vbv_maxrate=50000 / vbv_bufsize=50000 / crf_max=0.0 / nal_hrd=none / ip_ratio=1.40 / aq=2:1.00
Language                                 : English
Default                                  : Yes
Forced                                   : No
Color range                              : Limited
Color primaries                          : BT.709
Transfer characteristics                 : BT.709
Matrix coefficients                      : BT.709

Audio
ID                                       : 2
Format                                   : DTS
Format/Info                              : Digital Theater Systems
Codec ID                                 : A_DTS
Duration                                 : 1 h 29 min
Bit rate mode                            : Constant
Bit rate                                 : 1 509 kb/s
Channel(s)                               : 6 channels
Channel positions                        : Front: L C R, Side: L R, LFE
Sampling rate                            : 48.0 kHz
Frame rate                               : 93.750 FPS (512 SPF)
Bit depth                                : 24 bits
Compression mode                         : Lossy
Stream size                              : 963 MiB (9%)
Title                                    : The Gold Rush 1925 DTS @ 1.5 Mb/s 5.1 ENG
Language                                 : English
Default                                  : Yes
Forced                                   : No

Menu
00:00:00.000                             : fr:00:00:00.000
00:02:35.781                             : fr:00:02:35.781
00:07:02.422                             : fr:00:07:02.422
00:12:44.389                             : fr:00:12:44.389
00:17:27.630                             : fr:00:17:27.630
00:20:27.393                             : fr:00:20:27.393
00:27:07.793                             : fr:00:27:07.793
00:29:31.979                             : fr:00:29:31.979
00:35:53.318                             : fr:00:35:53.318
00:42:21.956                             : fr:00:42:21.956
00:46:25.449                             : fr:00:46:25.449
00:53:15.150                             : fr:00:53:15.150
00:55:05.385                             : fr:00:55:05.385
00:59:44.414                             : fr:00:59:44.414
01:05:47.068                             : fr:01:05:47.068
01:07:29.379                             : fr:01:07:29.379
01:11:46.552                             : fr:01:11:46.552
01:14:42.228                             : fr:01:14:42.228
01:22:11.260                             : fr:01:22:11.260', '', false, 'MKV', '11427139815', NULL, 'dts', NULL, NULL, NULL, 'h264', '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (8, 5, '2025-03-31 10:41:14.342926', '2025-03-31 10:41:14.342926', 1, 'English', 'The.Gold.Rush.1925.1080p.BluRay.AC3.x264-HDMaNiAcS', 'HDMaNiAcS', '', '{"mkv":1}', false, '{"files":[{"name":"The.Gold.Rush.1925.1080p.BluRay.AC3.x264-HDMaNiAcS.mkv","size":8321233920}],"parent_folder":"The.Gold.Rush.1925.1080p.BluRay.AC3.x264-HDMaNiAcS"}', 'General
Unique ID                                : 179639502249987895749525887233464447925 (0x87254F45BBE9FB1887C338F60283B3B5)
Complete name                            : The.Gold.Rush.1925.1080p.BluRay.AC3.x264-HDMaNiAcS.mkv
Format                                   : Matroska
Format version                           : Version 2
File size                                : 7.75 GiB
Duration                                 : 1h 12mn
Overall bit rate                         : 15.3 Mbps
Encoded date                             : UTC 2012-06-22 18:16:06
Writing application                      : mkvmerge v5.5.0 (''Healer'') built on Apr  6 2012 21:43:24
Writing library                          : libebml v1.2.3 + libmatroska v1.3.0

Video
ID                                       : 1
Format                                   : AVC
Format/Info                              : Advanced Video Codec
Format profile                           : High@L4.1
Format settings, CABAC                   : Yes
Format settings, ReFrames                : 4 frames
Codec ID                                 : V_MPEG4/ISO/AVC
Duration                                 : 1h 12mn
Bit rate                                 : 14.9 Mbps
Width                                    : 1 436 pixels
Height                                   : 1 080 pixels
Display aspect ratio                     : 4:3
Frame rate                               : 23.976 fps
Color space                              : YUV
Chroma subsampling                       : 4:2:0
Bit depth                                : 8 bits
Scan type                                : Progressive
Bits/(Pixel*Frame)                       : 0.399
Stream size                              : 7.37 GiB (95%)
Title                                    : The.Gold.Rush.1925.1080p.BluRay.AC3.x264-HDMaNiAcS
Writing library                          : x264 core 120 r2164 da19765
Encoding settings                        : cabac=1 / ref=4 / deblock=1:-3:-3 / analyse=0x3:0x133 / me=tesa / subme=11 / psy=1 / psy_rd=0.77:0.00 / mixed_ref=1 / me_range=24 / chroma_me=1 / trellis=2 / 8x8dct=1 / cqm=0 / deadzone=21,11 / fast_pskip=1 / chroma_qp_offset=-2 / threads=12 / sliced_threads=0 / nr=0 / decimate=1 / interlaced=0 / bluray_compat=0 / constrained_intra=0 / bframes=8 / b_pyramid=2 / b_adapt=2 / b_bias=0 / direct=3 / weightb=1 / open_gop=0 / weightp=2 / keyint=240 / keyint_min=23 / scenecut=40 / intra_refresh=0 / rc_lookahead=90 / rc=2pass / mbtree=1 / bitrate=14850 / ratetol=1.0 / qcomp=0.60 / qpmin=10 / qpmax=69 / qpstep=4 / cplxblur=20.0 / qblur=0.5 / ip_ratio=1.40 / aq=2:0.50
Language                                 : English
Default                                  : Yes
Forced                                   : No

Audio
ID                                       : 2
Format                                   : AC-3
Format/Info                              : Audio Coding 3
Mode extension                           : CM (complete main)
Format settings, Endianness              : Big
Codec ID                                 : A_AC3
Duration                                 : 1h 12mn
Bit rate mode                            : Constant
Bit rate                                 : 448 Kbps
Channel(s)                               : 1 channel
Channel positions                        : Front: C
Sampling rate                            : 48.0 KHz
Bit depth                                : 16 bits
Compression mode                         : Lossy
Stream size                              : 232 MiB (3%)
Language                                 : English
Default                                  : Yes
Forced                                   : No

Text #1
ID                                       : 3
Format                                   : PGS
Codec ID                                 : S_HDMV/PGS
Codec ID/Info                            : The same subtitle format used on BDs/HD-DVDs
Language                                 : English
Default                                  : Yes
Forced                                   : No

Text #2
ID                                       : 4
Format                                   : UTF-8
Codec ID                                 : S_TEXT/UTF8
Codec ID/Info                            : UTF-8 Plain Text
Language                                 : English
Default                                  : No
Forced                                   : No

Text #3
ID                                       : 5
Format                                   : UTF-8
Codec ID                                 : S_TEXT/UTF8
Codec ID/Info                            : UTF-8 Plain Text
Language                                 : Greek
Default                                  : No
Forced                                   : No

Text #4
ID                                       : 6
Format                                   : UTF-8
Codec ID                                 : S_TEXT/UTF8
Codec ID/Info                            : UTF-8 Plain Text
Language                                 : French
Default                                  : No
Forced                                   : No

Text #5
ID                                       : 7
Format                                   : UTF-8
Codec ID                                 : S_TEXT/UTF8
Codec ID/Info                            : UTF-8 Plain Text
Language                                 : Portuguese
Default                                  : No
Forced                                   : No

Text #6
ID                                       : 8
Format                                   : UTF-8
Codec ID                                 : S_TEXT/UTF8
Codec ID/Info                            : UTF-8 Plain Text
Language                                 : Spanish
Default                                  : No
Forced                                   : No

Text #7
ID                                       : 9
Format                                   : UTF-8
Codec ID                                 : S_TEXT/UTF8
Codec ID/Info                            : UTF-8 Plain Text
Language                                 : Turkish
Default                                  : No
Forced                                   : No

Menu
00:00:00.000                             : en:00:00:00.000
00:01:37.889                             : en:00:01:37.889
00:04:35.650                             : en:00:04:35.650
00:09:56.012                             : en:00:09:56.012
00:14:08.556                             : en:00:14:08.556
00:17:07.693                             : en:00:17:07.693
00:23:12.975                             : en:00:23:12.975
00:25:13.721                             : en:00:25:13.721
00:30:22.195                             : en:00:30:22.195
00:35:50.690                             : en:00:35:50.690
00:39:10.014                             : en:00:39:10.014
00:44:58.321                             : en:00:44:58.321
00:46:28.244                             : en:00:46:28.244
00:50:36.784                             : en:00:50:36.784
00:54:58.754                             : en:00:54:58.754
00:55:40.379                             : en:00:55:40.379
00:57:52.427                             : en:00:57:52.427
01:00:25.914                             : en:01:00:25.914
01:06:57.972                             : en:01:06:57.972', '', false, 'MKV', '8321233920', NULL, 'ac3', NULL, NULL, NULL, 'h264', '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (10, 4, '2025-03-31 10:49:49.903392', '2025-03-31 10:49:49.903392', 1, 'English', 'The.Gold.Rush.1925.Silent.Version.Criterion.Collection.BluRay.1080p.DTS-HD.MA.5.1.AVC.REMUX-FraMeSToR', 'FraMeSToR', 'none', '{"mkv":1,"nfo":1}', false, '{"files":[{"name":"The.Gold.Rush.1925.Silent.Version.Criterion.Collection.BluRay.1080p.DTS-HD.MA.5.1.AVC.REMUX-FraMeSToR.mkv","size":18110440501},{"name":"The.Gold.Rush.1925.Silent.Version.Criterion.Collection.BluRay.1080p.DTS-HD.MA.5.1.AVC.REMUX-FraMeSToR.nfo","size":3877}],"parent_folder":"The.Gold.Rush.1925.Silent.Version.Criterion.Collection.BluRay.1080p.DTS-HD.MA.5.1.AVC.REMUX-FraMeSToR"}', 'General
Unique ID                                : 28086799486511508475396472836896080231 (0x152152211542A719FD736D1DB9544167)
Complete name                            : The.Gold.Rush.1925.Silent.Version.Criterion.Collection.BluRay.1080p.DTS-HD.MA.5.1.AVC.REMUX-FraMeSToR.mkv
Format                                   : Matroska
Format version                           : Version 4
File size                                : 16.9 GiB
Duration                                 : 1 h 29 min
Overall bit rate mode                    : Variable
Overall bit rate                         : 27.0 Mb/s
Movie name                               : The Gold Rush (1925)
Encoded date                             : UTC 2018-10-21 14:41:44
Writing application                      : mkvmerge v25.0.0 (''Prog Noir'') 64-bit
Writing library                          : libebml v1.3.6 + libmatroska v1.4.9
IsTruncated                              : Yes

Video
ID                                       : 1
Format                                   : AVC
Format/Info                              : Advanced Video Codec
Format profile                           : High@L4.1
Format settings                          : CABAC / 4 Ref Frames
Format settings, CABAC                   : Yes
Format settings, Reference frames        : 4 frames
Codec ID                                 : V_MPEG4/ISO/AVC
Duration                                 : 1 h 29 min
Bit rate mode                            : Variable
Maximum bit rate                         : 37.0 Mb/s
Width                                    : 1 920 pixels
Height                                   : 1 080 pixels
Display aspect ratio                     : 16:9
Frame rate mode                          : Constant
Frame rate                               : 23.976 (24000/1001) FPS
Color space                              : YUV
Chroma subsampling                       : 4:2:0
Bit depth                                : 8 bits
Scan type                                : Progressive
Default                                  : Yes
Forced                                   : No

Audio #1
ID                                       : 2
Format                                   : DTS XLL
Format/Info                              : Digital Theater Systems
Commercial name                          : DTS-HD Master Audio
Codec ID                                 : A_DTS
Duration                                 : 1 h 29 min
Bit rate mode                            : Variable
Channel(s)                               : 6 channels
Channel layout                           : C L R Ls Rs LFE
Sampling rate                            : 48.0 kHz
Frame rate                               : 93.750 FPS (512 SPF)
Bit depth                                : 24 bits
Compression mode                         : Lossless
Title                                    : DTS-HD MA 5.1
Language                                 : English
Default                                  : Yes
Forced                                   : No

Audio #2
ID                                       : 3
Format                                   : AC-3
Format/Info                              : Audio Coding 3
Commercial name                          : Dolby Digital
Codec ID                                 : A_AC3
Duration                                 : 1 h 29 min
Bit rate mode                            : Constant
Bit rate                                 : 192 kb/s
Channel(s)                               : 2 channels
Channel layout                           : L R
Sampling rate                            : 48.0 kHz
Frame rate                               : 31.250 FPS (1536 SPF)
Compression mode                         : Lossy
Stream size                              : 123 MiB (1%)
Title                                    : Commentary by Chaplin Biographer Jeffrey Vance
Language                                 : English
Service kind                             : Complete Main
Default                                  : No
Forced                                   : No

Menu
00:00:00.000                             : en:Opening Titles
00:02:35.781                             : en:Chilkoot Pass
00:07:02.422                             : en:Black Larsen
00:12:44.389                             : en:Three Hungry Men
00:17:27.630                             : en:"Thanksgiving Dinner"
00:20:27.393                             : en:An Appetizing Friend
00:27:07.793                             : en:"Parting of the Ways"
00:29:31.979                             : en:Georgia
00:35:53.318                             : en:Dancing at the Monte Carlo
00:42:21.956                             : en:Hank Curtis and His Cabin
00:46:25.449                             : en:Surprise Guests
00:53:15.150                             : en:"Begged, Borrowed, and Shoveled"
00:55:05.385                             : en:New Year''s Eve
00:59:44.414                             : en:Georgia Remembers
01:05:47.068                             : en:The Return of Big Jim
01:07:29.379                             : en:"Please Forgive Me"
01:11:46.552                             : en:"The Cabin at Last!"
01:14:42.228                             : en:"All Was Calm"
01:22:11.260                             : en:Homeward-Bound', '', false, 'MKV', '18110444378', NULL, 'dts', NULL, '24bit Lossless', NULL, 'h264', '{Remux}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (3, 2, '2025-03-30 17:46:29.779262', '2025-03-30 17:46:29.779262', 1, NULL, 'The Beatles - Love Me Do (Single) (1992) [FLAC]', '', 'Track 1: Album version + reissue Single versions
Track 3: Initial Single version

"The first version of ''Love Me Do'' was available only on initial pressings of the 45 while the one included on the album ''Please Please Me'' has always been the version with Andy White on drums. The single came out on Friday 5 October 1962." (Mark Lewisohn - Author of The Complete Beatles Chronicle).', '{"cue":1,"log":1,"flac":3}', false, '{"files":[{"name":"01 Love Me Do.flac","size":11268945},{"name":"02 P.S. I Love You.flac","size":9505300},{"name":"03 Love Me Do (original single version).flac","size":11238335},{"name":"Love Me Do (Single).cue","size":848},{"name":"Love Me Do (Single).log","size":3852}],"parent_folder":"The Beatles - Love Me Do (Single) (1992) [FLAC]"}', 'none', '', false, 'FLAC', '32017280', NULL, 'flac', NULL, 'Lossless', NULL, NULL, '{Cue}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (9, 6, '2025-03-31 10:45:04.865918', '2025-03-31 10:45:04.865918', 1, 'English', 'The.Gold.Rush.1925.Sound.Version.Criterion.Collection.BluRay.1080p.DTS-HD.MA.1.0.AVC.REMUX-FraMeSToR', 'FraMeSToR', '-----------------------------------------------------------------------------------
| ''||''''''''|                 ''||    ||''          .|''''''.|  |''''||''''|         ''||''''|.   |
|  ||  .   ... ..   ....    |||  |||    ....   ||..  ''     ||      ...    ||   ||  |
|  ||''''|    ||'' '''' '''' .||   |''|..''||  .|...||   ''''|||.     ||    .|  ''|.  ||''''|''   |
|  ||       ||     .|'' ||   | ''|'' ||  ||      .     ''||    ||    ||   ||  ||   |.  |
| .||.     .||.    ''|..''|'' .|. | .||.  ''|...'' |''....|''    .||.    ''|..|'' .||.  ''|'' |
-----------------------------------------------------------------------------------
                                 Proudly Presents
-----------------------------------------------------------------------------------
The.Gold.Rush.1925.Sound.Version.Criterion.Collection.BluRay.1080p.DTS-HD.MA.1.0.AVC.REMUX-FraMeSToR


GENERAL INFO
SOURCE                  : Blu-ray Disc Criterion Collection (???) (Thanks!)
FORMAT                  : MKV (Matroska)
SIZE                    : 12.1 GiB
DURATION                : 01:12:24 (h:m:s)
CHAPTERS                : Named (01-19)
IMDB                    : https://www.imdb.com/title/tt0015864/

VIDEO
CODEC                   : AVC
TYPE                    : 1080p (Progressive)
FRAME RATE              : 23.976 fps
DISPLAY ASPECT RATIO    : 16:9
FORMAT PROFILE LEVEL    : High@L4.1
BITRATE                 : 22.9 Mbps
WIDTH x HEIGHT          : 1920 x 1080 pixels

AUDIO
CODEC                   : DTS-HD Master Audio
LANGUAGE                : English
CHANNEL(S)              : 1.0
BITRATE                 : 1024 kbps
SAMPLING RATE           : 48 kHz
BIT DEPTH               : 24 bits
OTHER INFO              : DTS Core: 1.0 / 48 kHz / 768 kbps / 24-bit

SUBTITLES
English (SDH)

SCREENSHOTS




The subtitles have the following format PGS (Blu-Ray) / .sup that are
merged with the video and audio.

RELEASE NOTES
New high-definition digital restoration of the 1942 sound version (2012).

GREETS
To all those that have made us what we are today!

Big Shout Out to all who support our group, our fellow colleague Encoders / Remuxers
and our community members, No FraMeSToR without you guys.

GROUP NOTES
Please do not alter our releases when uploading them elsewhere, keep the NFO intact
and file names the same.', '{"mkv":1,"nfo":1}', false, '{"files":[{"name":"The.Gold.Rush.1925.Sound.Version.Criterion.Collection.BluRay.1080p.DTS-HD.MA.1.0.AVC.REMUX-FraMeSToR.mkv","size":13022303084},{"name":"The.Gold.Rush.1925.Sound.Version.Criterion.Collection.BluRay.1080p.DTS-HD.MA.1.0.AVC.REMUX-FraMeSToR.nfo","size":3516}],"parent_folder":"The.Gold.Rush.1925.Sound.Version.Criterion.Collection.BluRay.1080p.DTS-HD.MA.1.0.AVC.REMUX-FraMeSToR"}', 'General
Unique ID                                : 279351979850065970759465187068304942630 (0xD2293BFB32669D23555AA2184D27E626)
Complete name                            : The.Gold.Rush.1925.Sound.Version.Criterion.Collection.BluRay.1080p.DTS-HD.MA.1.0.AVC.REMUX-FraMeSToR.mkv
Format                                   : Matroska
Format version                           : Version 4
File size                                : 12.1 GiB
Duration                                 : 1 h 12 min
Overall bit rate mode                    : Variable
Overall bit rate                         : 24.0 Mb/s
Movie name                               : The Gold Rush (1925)
Encoded date                             : UTC 2018-10-21 15:38:18
Writing application                      : mkvmerge v25.0.0 (''Prog Noir'') 64-bit
Writing library                          : libebml v1.3.6 + libmatroska v1.4.9

Video
ID                                       : 1
Format                                   : AVC
Format/Info                              : Advanced Video Codec
Format profile                           : High@L4.1
Format settings                          : CABAC / 4 Ref Frames
Format settings, CABAC                   : Yes
Format settings, Reference frames        : 4 frames
Codec ID                                 : V_MPEG4/ISO/AVC
Duration                                 : 1 h 12 min
Bit rate mode                            : Variable
Bit rate                                 : 22.9 Mb/s
Maximum bit rate                         : 37.0 Mb/s
Width                                    : 1 920 pixels
Height                                   : 1 080 pixels
Display aspect ratio                     : 16:9
Frame rate mode                          : Constant
Frame rate                               : 23.976 (24000/1001) FPS
Color space                              : YUV
Chroma subsampling                       : 4:2:0
Bit depth                                : 8 bits
Scan type                                : Progressive
Bits/(Pixel*Frame)                       : 0.461
Stream size                              : 11.6 GiB (96%)
Default                                  : Yes
Forced                                   : No

Audio
ID                                       : 2
Format                                   : DTS XLL
Format/Info                              : Digital Theater Systems
Commercial name                          : DTS-HD Master Audio
Codec ID                                 : A_DTS
Duration                                 : 1 h 12 min
Bit rate mode                            : Variable
Bit rate                                 : 1 035 kb/s
Channel(s)                               : 1 channel
Channel layout                           : C
Sampling rate                            : 48.0 kHz
Frame rate                               : 93.750 FPS (512 SPF)
Bit depth                                : 24 bits
Compression mode                         : Lossless
Stream size                              : 536 MiB (4%)
Title                                    : DTS-HD MA 1.0
Language                                 : English
Default                                  : Yes
Forced                                   : No

Text
ID                                       : 3
Format                                   : PGS
Codec ID                                 : S_HDMV/PGS
Codec ID/Info                            : Picture based subtitle format used on BDs/HD-DVDs
Duration                                 : 1 h 11 min
Bit rate                                 : 25.3 kb/s
Count of elements                        : 768
Stream size                              : 12.9 MiB (0%)
Title                                    : English (SDH)
Language                                 : English
Default                                  : No
Forced                                   : No

Menu
00:00:00.000                             : en:Opening Titles
00:01:37.889                             : en:Chilkoot Pass
00:04:35.650                             : en:Black Larsen
00:09:56.012                             : en:Three Hungry Men
00:14:08.556                             : en:"Thanksgiving Dinner"
00:17:07.693                             : en:An Appetizing Friend
00:23:12.975                             : en:"Parting of the Ways"
00:25:13.721                             : en:Georgia
00:30:22.195                             : en:Dancing at the Monte Carlo
00:35:50.690                             : en:Hank Curtis and His Cabin
00:39:10.014                             : en:Surprise Guests
00:44:58.321                             : en:"Begged, Borrowed, and Shoveled"
00:46:28.244                             : en:New Year''s Eve
00:50:36.784                             : en:Georgia Remembers
00:54:58.754                             : en:The Return of Big Jim
00:55:40.379                             : en:"Please Forgive Me"
00:57:52.427                             : en:"The Cabin at Last!"
01:00:25.914                             : en:"All Was Calm"
01:06:57.972                             : en:Homeward-Bound', '', false, 'MKV', '13022306600', NULL, 'dts', NULL, '24bit Lossless', NULL, 'h264', '{Remux}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (11, 7, '2025-03-31 11:04:55.635493', '2025-03-31 11:04:55.635493', 1, 'French', 'none', '', '', '{"jpg":1,"pdf":1,"epub":1,"mobi":1}', false, '{"files":[{"name":"Les Miserables - Tome I - Fantine - Victor Hugo.epub","size":501485},{"name":"Les Miserables - Tome I - Fantine - Victor Hugo.mobi","size":646996},{"name":"Les Miserables - Tome I - Fantine - Victor Hugo.pdf","size":5006768},{"name":"cover.jpg","size":117560}],"parent_folder":"Les Miserables - Tome I - Fantine"}', 'none', '', false, 'PDF', '6272809', NULL, NULL, NULL, NULL, NULL, NULL, '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (12, 8, '2025-03-31 11:09:16.447595', '2025-03-31 11:09:16.447595', 1, 'English', '', '', '', '{"jpg":1,"pdf":1,"epub":1,"mobi":1}', false, '{"files":[{"name":"Les Miserables - Tome I - Fantine - Victor Hugo.epub","size":501485},{"name":"Les Miserables - Tome I - Fantine - Victor Hugo.mobi","size":646996},{"name":"Les Miserables - Tome I - Fantine - Victor Hugo.pdf","size":5006768},{"name":"cover.jpg","size":117560}],"parent_folder":"Les Miserables - Tome I - Fantine"}', 'none', '', false, 'EPUB', '6272809', NULL, NULL, NULL, NULL, NULL, NULL, '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (13, 8, '2025-03-31 11:10:39.764144', '2025-03-31 11:10:39.764144', 1, 'French', '', '', '', '{"jpg":1,"pdf":1,"epub":1,"mobi":1}', false, '{"files":[{"name":"Les Miserables - Tome I - Fantine - Victor Hugo.epub","size":501485},{"name":"Les Miserables - Tome I - Fantine - Victor Hugo.mobi","size":646996},{"name":"Les Miserables - Tome I - Fantine - Victor Hugo.pdf","size":5006768},{"name":"cover.jpg","size":117560}],"parent_folder":"Les Miserables - Tome I - Fantine"}', 'none', '', false, 'EPUB', '6272809', NULL, NULL, NULL, NULL, NULL, NULL, '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (18, 11, '2025-03-31 12:05:21.264012', '2025-03-31 12:05:21.264012', 1, 'English', '', '', '', '{"jpg":1,"pdf":1,"epub":1,"mobi":1}', false, '{"files":[{"name":"Les Miserables - Tome I - Fantine - Victor Hugo.epub","size":501485},{"name":"Les Miserables - Tome I - Fantine - Victor Hugo.mobi","size":646996},{"name":"Les Miserables - Tome I - Fantine - Victor Hugo.pdf","size":5006768},{"name":"cover.jpg","size":117560}],"parent_folder":"Les Miserables - Tome I - Fantine"}', 'none', '', false, 'PDF', '6272809', NULL, NULL, NULL, NULL, NULL, NULL, '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (19, 12, '2025-03-31 12:15:51.683872', '2025-03-31 12:15:51.683872', 1, 'English', '', '', '', '{"jpg":1,"pdf":1,"epub":1,"mobi":1}', false, '{"files":[{"name":"Les Miserables - Tome I - Fantine - Victor Hugo.epub","size":501485},{"name":"Les Miserables - Tome I - Fantine - Victor Hugo.mobi","size":646996},{"name":"Les Miserables - Tome I - Fantine - Victor Hugo.pdf","size":5006768},{"name":"cover.jpg","size":117560}],"parent_folder":"Les Miserables - Tome I - Fantine"}', 'none', '', false, 'PDF', '6272809', NULL, NULL, NULL, NULL, NULL, NULL, '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (17, 9, '2025-03-31 11:42:20.339344', '2025-03-31 11:42:20.339344', 1, 'French', '', '', '', '{"mp3":264}', false, '{"files":[{"name":"1. Fantine/01 01_FANTINE_LES_MISERABLES_T1.mp3","size":19089789},{"name":"1. Fantine/02 02_FANTINE_LES_MISERABLES_T1.mp3","size":21961668},{"name":"1. Fantine/03 03_FANTINE_LES_MISERABLES_T1.mp3","size":19490526},{"name":"1. Fantine/04 04_FANTINE_LES_MISERABLES_T1.mp3","size":15883476},{"name":"1. Fantine/05 05_FANTINE_LES_MISERABLES_T1.mp3","size":22128051},{"name":"1. Fantine/06 06_FANTINE_LES_MISERABLES_T1.mp3","size":14582019},{"name":"1. Fantine/07 07_FANTINE_LES_MISERABLES_T1.mp3","size":9889935},{"name":"1. Fantine/08 08_FANTINE_LES_MISERABLES_T1.mp3","size":23024601},{"name":"1. Fantine/09 09_FANTINE_LES_MISERABLES_T1.mp3","size":25439031},{"name":"1. Fantine/10 10_FANTINE_LES_MISERABLES_T1.mp3","size":20517180},{"name":"1. Fantine/11 11_FANTINE_LES_MISERABLES_T1.mp3","size":23851929},{"name":"1. Fantine/12 12_FANTINE_LES_MISERABLES_T1.mp3","size":17854635},{"name":"1. Fantine/13 13_FANTINE_LES_MISERABLES_T1.mp3","size":22403271},{"name":"1. Fantine/14 14_FANTINE_LES_MISERABLES_T1.mp3","size":25885221},{"name":"1. Fantine/15 15_FANTINE_LES_MISERABLES_T1.mp3","size":25306842},{"name":"1. Fantine/16 16_FANTINE_LES_MISERABLES_T1.mp3","size":24007887},{"name":"1. Fantine/17 17_FANTINE_LES_MISERABLES_T1.mp3","size":24565833},{"name":"1. Fantine/18 18_FANTINE_LES_MISERABLES_T1.mp3","size":20440035},{"name":"1. Fantine/19 19_FANTINE_LES_MISERABLES_T1.mp3","size":11358192},{"name":"1. Fantine/20 20_FANTINE_LES_MISERABLES_T1.mp3","size":24106716},{"name":"1. Fantine/21 21_FANTINE_LES_MISERABLES_T1.mp3","size":22213953},{"name":"1. Fantine/22 22_FANTINE_LES_MISERABLES_T1.mp3","size":26445252},{"name":"1. Fantine/23 23_FANTINE_LES_MISERABLES_T1.mp3","size":27280920},{"name":"1. Fantine/24 24_FANTINE_LES_MISERABLES_T1.mp3","size":28240854},{"name":"1. Fantine/25 25_FANTINE_LES_MISERABLES_T1.mp3","size":16341759},{"name":"1. Fantine/26 26_FANTINE_LES_MISERABLES_T1.mp3","size":23070054},{"name":"1. Fantine/27 27_FANTINE_LES_MISERABLES_T1.mp3","size":18679878},{"name":"1. Fantine/28 28_FANTINE_LES_MISERABLES_T1.mp3","size":9483777},{"name":"1. Fantine/29 29_FANTINE_LES_MISERABLES_T1.mp3","size":22940367},{"name":"1. Fantine/30 30_FANTINE_LES_MISERABLES_T1.mp3","size":24766827},{"name":"1. Fantine/31 31_FANTINE_LES_MISERABLES_T1.mp3","size":23725578},{"name":"1. Fantine/32 32_FANTINE_LES_MISERABLES_T1.mp3","size":16473114},{"name":"1. Fantine/33 33_FANTINE_LES_MISERABLES_T1.mp3","size":10657215},{"name":"1. Fantine/34 34_FANTINE_LES_MISERABLES_T1.mp3","size":22520031},{"name":"1. Fantine/35 35_FANTINE_LES_MISERABLES_T1.mp3","size":21407058},{"name":"2. Cosette/01 01_COSETTE_LES_MISERABLES_T2.mp3","size":4211767},{"name":"2. Cosette/02 02_COSETTE_LES_MISERABLES_T2.mp3","size":15643787},{"name":"2. Cosette/03 03_COSETTE_LES_MISERABLES_T2.mp3","size":5300551},{"name":"2. Cosette/04 04_COSETTE_LES_MISERABLES_T2.mp3","size":4196303},{"name":"2. Cosette/05 05_COSETTE_LES_MISERABLES_T2.mp3","size":6214210},{"name":"2. Cosette/06 06_COSETTE_LES_MISERABLES_T2.mp3","size":6161547},{"name":"2. Cosette/07 07_COSETTE_LES_MISERABLES_T2.mp3","size":11911829},{"name":"2. Cosette/08 08_COSETTE_LES_MISERABLES_T2.mp3","size":5279653},{"name":"2. Cosette/09 09_COSETTE_LES_MISERABLES_T2.mp3","size":7989282},{"name":"2. Cosette/10 10_COSETTE_LES_MISERABLES_T2.mp3","size":10207392},{"name":"2. Cosette/11 11_COSETTE_LES_MISERABLES_T2.mp3","size":3229563},{"name":"2. Cosette/12 12_COSETTE_LES_MISERABLES_T2.mp3","size":3433945},{"name":"2. Cosette/13 13_COSETTE_LES_MISERABLES_T2.mp3","size":5413400},{"name":"2. Cosette/14 14_COSETTE_LES_MISERABLES_T2.mp3","size":2665318},{"name":"2. Cosette/15 15_COSETTE_LES_MISERABLES_T2.mp3","size":4646445},{"name":"2. Cosette/16 16_COSETTE_LES_MISERABLES_T2.mp3","size":10654608},{"name":"2. Cosette/17 17_COSETTE_LES_MISERABLES_T2.mp3","size":3853576},{"name":"2. Cosette/18 18_COSETTE_LES_MISERABLES_T2.mp3","size":5580584},{"name":"2. Cosette/19 19_COSETTE_LES_MISERABLES_T2.mp3","size":12964668},{"name":"2. Cosette/20 20_COSETTE_LES_MISERABLES_T2.mp3","size":5601899},{"name":"2. Cosette/21 21_COSETTE_LES_MISERABLES_T2.mp3","size":9572930},{"name":"2. Cosette/22 22_COSETTE_LES_MISERABLES_T2.mp3","size":16299147},{"name":"2. Cosette/23 23_COSETTE_LES_MISERABLES_T2.mp3","size":7249077},{"name":"2. Cosette/24 24_COSETTE_LES_MISERABLES_T2.mp3","size":11666905},{"name":"2. Cosette/25 25_COSETTE_LES_MISERABLES_T2.mp3","size":4515624},{"name":"2. Cosette/26 26_COSETTE_LES_MISERABLES_T2.mp3","size":3099578},{"name":"2. Cosette/27 27_COSETTE_LES_MISERABLES_T2.mp3","size":11619258},{"name":"2. Cosette/28 28_COSETTE_LES_MISERABLES_T2.mp3","size":10163506},{"name":"2. Cosette/29 29_COSETTE_LES_MISERABLES_T2.mp3","size":5028042},{"name":"2. Cosette/30 30_COSETTE_LES_MISERABLES_T2.mp3","size":35739682},{"name":"2. Cosette/31 31_COSETTE_LES_MISERABLES_T2.mp3","size":15651310},{"name":"2. Cosette/32 32_COSETTE_LES_MISERABLES_T2.mp3","size":9892668},{"name":"2. Cosette/33 33_COSETTE_LES_MISERABLES_T2.mp3","size":3084949},{"name":"2. Cosette/34 34_COSETTE_LES_MISERABLES_T2.mp3","size":13426095},{"name":"2. Cosette/35 35_COSETTE_LES_MISERABLES_T2.mp3","size":3359548},{"name":"2. Cosette/36 36_COSETTE_LES_MISERABLES_T2.mp3","size":8960619},{"name":"2. Cosette/37 37_COSETTE_LES_MISERABLES_T2.mp3","size":4146984},{"name":"2. Cosette/38 38_COSETTE_LES_MISERABLES_T2.mp3","size":6899663},{"name":"2. Cosette/39 39_COSETTE_LES_MISERABLES_T2.mp3","size":7479790},{"name":"2. Cosette/40 40_COSETTE_LES_MISERABLES_T2.mp3","size":3220786},{"name":"2. Cosette/41 41_COSETTE_LES_MISERABLES_T2.mp3","size":6345449},{"name":"2. Cosette/42 42_COSETTE_LES_MISERABLES_T2.mp3","size":5104528},{"name":"2. Cosette/43 43_COSETTE_LES_MISERABLES_T2.mp3","size":7243225},{"name":"2. Cosette/44 44_COSETTE_LES_MISERABLES_T2.mp3","size":5063986},{"name":"2. Cosette/45 45_COSETTE_LES_MISERABLES_T2.mp3","size":4236845},{"name":"2. Cosette/46 46_COSETTE_LES_MISERABLES_T2.mp3","size":3589008},{"name":"2. Cosette/47 47_COSETTE_LES_MISERABLES_T2.mp3","size":6891722},{"name":"2. Cosette/48 48_COSETTE_LES_MISERABLES_T2.mp3","size":17751973},{"name":"2. Cosette/49 49_COSETTE_LES_MISERABLES_T2.mp3","size":8071620},{"name":"2. Cosette/50 50_COSETTE_LES_MISERABLES_T2.mp3","size":13790974},{"name":"2. Cosette/51 51_COSETTE_LES_MISERABLES_T2.mp3","size":2977534},{"name":"2. Cosette/52 52_COSETTE_LES_MISERABLES_T2.mp3","size":6564459},{"name":"2. Cosette/53 53_COSETTE_LES_MISERABLES_T2.mp3","size":10658370},{"name":"2. Cosette/54 54_COSETTE_LES_MISERABLES_T2.mp3","size":5587271},{"name":"2. Cosette/55 55_COSETTE_LES_MISERABLES_T2.mp3","size":4771833},{"name":"2. Cosette/56 56_COSETTE_LES_MISERABLES_T2.mp3","size":3526314},{"name":"2. Cosette/57 57_COSETTE_LES_MISERABLES_T2.mp3","size":4064228},{"name":"2. Cosette/58 58_COSETTE_LES_MISERABLES_T2.mp3","size":3319424},{"name":"2. Cosette/59 59_COSETTE_LES_MISERABLES_T2.mp3","size":3781269},{"name":"2. Cosette/60 60_COSETTE_LES_MISERABLES_T2.mp3","size":1272260},{"name":"2. Cosette/61 61_COSETTE_LES_MISERABLES_T2.mp3","size":6530605},{"name":"2. Cosette/62 62_COSETTE_LES_MISERABLES_T2.mp3","size":4974125},{"name":"2. Cosette/63 63_COSETTE_LES_MISERABLES_T2.mp3","size":3149733},{"name":"2. Cosette/64 64_COSETTE_LES_MISERABLES_T2.mp3","size":2708368},{"name":"2. Cosette/65 65_COSETTE_LES_MISERABLES_T2.mp3","size":4687405},{"name":"2. Cosette/66 66_COSETTE_LES_MISERABLES_T2.mp3","size":1664724},{"name":"2. Cosette/67 67_COSETTE_LES_MISERABLES_T2.mp3","size":4339245},{"name":"2. Cosette/68 68_COSETTE_LES_MISERABLES_T2.mp3","size":15326556},{"name":"2. Cosette/69 69_COSETTE_LES_MISERABLES_T2.mp3","size":4563689},{"name":"2. Cosette/70 70_COSETTE_LES_MISERABLES_T2.mp3","size":17199431},{"name":"2. Cosette/71 71_COSETTE_LES_MISERABLES_T2.mp3","size":10035193},{"name":"2. Cosette/72 72_COSETTE_LES_MISERABLES_T2.mp3","size":11956133},{"name":"2. Cosette/73 73_COSETTE_LES_MISERABLES_T2.mp3","size":3450664},{"name":"2. Cosette/74 74_COSETTE_LES_MISERABLES_T2.mp3","size":13539362},{"name":"2. Cosette/75 75_COSETTE_LES_MISERABLES_T2.mp3","size":6098853},{"name":"2. Cosette/76 76_COSETTE_LES_MISERABLES_T2.mp3","size":14790732},{"name":"3. Marius/01 01_MARIUS_LES_MISERABLES_T3.mp3","size":1727328},{"name":"3. Marius/02 02_MARIUS_LES_MISERABLES_T3.mp3","size":2929956},{"name":"3. Marius/03 03_MARIUS_LES_MISERABLES_T3.mp3","size":2741889},{"name":"3. Marius/04 04_MARIUS_LES_MISERABLES_T3.mp3","size":1805307},{"name":"3. Marius/05 05_MARIUS_LES_MISERABLES_T3.mp3","size":5341467},{"name":"3. Marius/06 06_MARIUS_LES_MISERABLES_T3.mp3","size":4273113},{"name":"3. Marius/07 07_MARIUS_LES_MISERABLES_T3.mp3","size":4234332},{"name":"3. Marius/08 08_MARIUS_LES_MISERABLES_T3.mp3","size":3621759},{"name":"3. Marius/09 09_MARIUS_LES_MISERABLES_T3.mp3","size":1867440},{"name":"3. Marius/10 10_MARIUS_LES_MISERABLES_T3.mp3","size":7738383},{"name":"3. Marius/11 11_MARIUS_LES_MISERABLES_T3.mp3","size":5233047},{"name":"3. Marius/12 12_MARIUS_LES_MISERABLES_T3.mp3","size":2464584},{"name":"3. Marius/13 13_MARIUS_LES_MISERABLES_T3.mp3","size":5486166},{"name":"3. Marius/14 14_MARIUS_LES_MISERABLES_T3.mp3","size":4280619},{"name":"3. Marius/15 15_MARIUS_LES_MISERABLES_T3.mp3","size":2652651},{"name":"3. Marius/16 16_MARIUS_LES_MISERABLES_T3.mp3","size":2314047},{"name":"3. Marius/17 17_MARIUS_LES_MISERABLES_T3.mp3","size":1929156},{"name":"3. Marius/18 18_MARIUS_LES_MISERABLES_T3.mp3","size":3003348},{"name":"3. Marius/19 19_MARIUS_LES_MISERABLES_T3.mp3","size":4603377},{"name":"3. Marius/20 20_MARIUS_LES_MISERABLES_T3.mp3","size":1545933},{"name":"3. Marius/21 21_MARIUS_LES_MISERABLES_T3.mp3","size":5391924},{"name":"3. Marius/22 22_MARIUS_LES_MISERABLES_T3.mp3","size":8515254},{"name":"3. Marius/23 23_MARIUS_LES_MISERABLES_T3.mp3","size":16086723},{"name":"3. Marius/24 24_MARIUS_LES_MISERABLES_T3.mp3","size":18134193},{"name":"3. Marius/25 25_MARIUS_LES_MISERABLES_T3.mp3","size":7938543},{"name":"3. Marius/26 26_MARIUS_LES_MISERABLES_T3.mp3","size":3838182},{"name":"3. Marius/27 27_MARIUS_LES_MISERABLES_T3.mp3","size":14773590},{"name":"3. Marius/28 28_MARIUS_LES_MISERABLES_T3.mp3","size":10349220},{"name":"3. Marius/29 29_MARIUS_LES_MISERABLES_T3.mp3","size":10847952},{"name":"3. Marius/30 30_MARIUS_LES_MISERABLES_T3.mp3","size":32791743},{"name":"3. Marius/31 31_MARIUS_LES_MISERABLES_T3.mp3","size":7703355},{"name":"3. Marius/32 32_MARIUS_LES_MISERABLES_T3.mp3","size":4508301},{"name":"3. Marius/33 33_MARIUS_LES_MISERABLES_T3.mp3","size":16783947},{"name":"3. Marius/34 34_MARIUS_LES_MISERABLES_T3.mp3","size":8390571},{"name":"3. Marius/35 35_MARIUS_LES_MISERABLES_T3.mp3","size":4988685},{"name":"3. Marius/36 36_MARIUS_LES_MISERABLES_T3.mp3","size":5159238},{"name":"3. Marius/37 37_MARIUS_LES_MISERABLES_T3.mp3","size":7234647},{"name":"3. Marius/38 38_MARIUS_LES_MISERABLES_T3.mp3","size":10752042},{"name":"3. Marius/39 39_MARIUS_LES_MISERABLES_T3.mp3","size":9514803},{"name":"3. Marius/40 40_MARIUS_LES_MISERABLES_T3.mp3","size":5111283},{"name":"3. Marius/41 41_MARIUS_LES_MISERABLES_T3.mp3","size":10035636},{"name":"3. Marius/42 42_MARIUS_LES_MISERABLES_T3.mp3","size":7103292},{"name":"3. Marius/43 43_MARIUS_LES_MISERABLES_T3.mp3","size":4982847},{"name":"3. Marius/44 44_MARIUS_LES_MISERABLES_T3.mp3","size":2673918},{"name":"3. Marius/45 45_MARIUS_LES_MISERABLES_T3.mp3","size":6192564},{"name":"3. Marius/46 46_MARIUS_LES_MISERABLES_T3.mp3","size":2720622},{"name":"3. Marius/47 47_MARIUS_LES_MISERABLES_T3.mp3","size":5550801},{"name":"3. Marius/48 48_MARIUS_LES_MISERABLES_T3.mp3","size":4102977},{"name":"3. Marius/49 49_MARIUS_LES_MISERABLES_T3.mp3","size":3830259},{"name":"3. Marius/50 50_MARIUS_LES_MISERABLES_T3.mp3","size":4788942},{"name":"3. Marius/51 51_MARIUS_LES_MISERABLES_T3.mp3","size":5795163},{"name":"3. Marius/52 52_MARIUS_LES_MISERABLES_T3.mp3","size":3945768},{"name":"3. Marius/53 53_MARIUS_LES_MISERABLES_T3.mp3","size":5797248},{"name":"3. Marius/54 54_MARIUS_LES_MISERABLES_T3.mp3","size":5939028},{"name":"3. Marius/55 55_MARIUS_LES_MISERABLES_T3.mp3","size":4254765},{"name":"3. Marius/56 56_MARIUS_LES_MISERABLES_T3.mp3","size":3936177},{"name":"3. Marius/57 57_MARIUS_LES_MISERABLES_T3.mp3","size":8963529},{"name":"3. Marius/58 58_MARIUS_LES_MISERABLES_T3.mp3","size":13738596},{"name":"3. Marius/59 59_MARIUS_LES_MISERABLES_T3.mp3","size":5388588},{"name":"3. Marius/60 60_MARIUS_LES_MISERABLES_T3.mp3","size":9145758},{"name":"3. Marius/61 61_MARIUS_LES_MISERABLES_T3.mp3","size":6461529},{"name":"3. Marius/62 62_MARIUS_LES_MISERABLES_T3.mp3","size":4380282},{"name":"3. Marius/63 63_MARIUS_LES_MISERABLES_T3.mp3","size":7623708},{"name":"3. Marius/64 64_MARIUS_LES_MISERABLES_T3.mp3","size":6851007},{"name":"3. Marius/65 65_MARIUS_LES_MISERABLES_T3.mp3","size":5330625},{"name":"3. Marius/66 66_MARIUS_LES_MISERABLES_T3.mp3","size":9497706},{"name":"3. Marius/67 67_MARIUS_LES_MISERABLES_T3.mp3","size":5454057},{"name":"3. Marius/68 68_MARIUS_LES_MISERABLES_T3.mp3","size":6923982},{"name":"3. Marius/69 69_MARIUS_LES_MISERABLES_T3.mp3","size":4714716},{"name":"3. Marius/70 70_MARIUS_LES_MISERABLES_T3.mp3","size":6947334},{"name":"3. Marius/71 71_MARIUS_LES_MISERABLES_T3.mp3","size":8279649},{"name":"3. Marius/72 72_MARIUS_LES_MISERABLES_T3.mp3","size":3538776},{"name":"3. Marius/73 73_MARIUS_LES_MISERABLES_T3.mp3","size":8803818},{"name":"3. Marius/74 74_MARIUS_LES_MISERABLES_T3.mp3","size":49923354},{"name":"3. Marius/75 75_MARIUS_LES_MISERABLES_T3.mp3","size":7386852},{"name":"3. Marius/76 76_MARIUS_LES_MISERABLES_T3.mp3","size":3841101},{"name":"4. L''idylle & l''épopée/01 01_L_IDYLLE_LES_MISERABLES_T4.mp3","size":21818488},{"name":"4. L''idylle & l''épopée/02 02_L_IDYLLE_LES_MISERABLES_T4.mp3","size":30923311},{"name":"4. L''idylle & l''épopée/03 03_L_IDYLLE_LES_MISERABLES_T4.mp3","size":29757205},{"name":"4. L''idylle & l''épopée/04 04_L_IDYLLE_LES_MISERABLES_T4.mp3","size":21266782},{"name":"4. L''idylle & l''épopée/05 05_L_IDYLLE_LES_MISERABLES_T4.mp3","size":16940905},{"name":"4. L''idylle & l''épopée/06 06_L_IDYLLE_LES_MISERABLES_T4.mp3","size":14864066},{"name":"4. L''idylle & l''épopée/07 07_L_IDYLLE_LES_MISERABLES_T4.mp3","size":19010221},{"name":"4. L''idylle & l''épopée/08 08_L_IDYLLE_LES_MISERABLES_T4.mp3","size":16112092},{"name":"4. L''idylle & l''épopée/09 09_L_IDYLLE_LES_MISERABLES_T4.mp3","size":29961587},{"name":"4. L''idylle & l''épopée/10 10_L_IDYLLE_LES_MISERABLES_T4.mp3","size":21015589},{"name":"4. L''idylle & l''épopée/11 11_L_IDYLLE_LES_MISERABLES_T4.mp3","size":11178084},{"name":"4. L''idylle & l''épopée/12 12_L_IDYLLE_LES_MISERABLES_T4.mp3","size":15028742},{"name":"4. L''idylle & l''épopée/13 13_L_IDYLLE_LES_MISERABLES_T4.mp3","size":12780121},{"name":"4. L''idylle & l''épopée/14 14_L_IDYLLE_LES_MISERABLES_T4.mp3","size":49587697},{"name":"4. L''idylle & l''épopée/15 15_L_IDYLLE_LES_MISERABLES_T4.mp3","size":28699350},{"name":"4. L''idylle & l''épopée/16 16_L_IDYLLE_LES_MISERABLES_T4.mp3","size":37419651},{"name":"4. L''idylle & l''épopée/17 17_L_IDYLLE_LES_MISERABLES_T4.mp3","size":21621212},{"name":"4. L''idylle & l''épopée/18 18_L_IDYLLE_LES_MISERABLES_T4.mp3","size":19095484},{"name":"4. L''idylle & l''épopée/19 19_L_IDYLLE_LES_MISERABLES_T4.mp3","size":20995109},{"name":"4. L''idylle & l''épopée/20 20_L_IDYLLE_LES_MISERABLES_T4.mp3","size":14833973},{"name":"4. L''idylle & l''épopée/21 21_L_IDYLLE_LES_MISERABLES_T4.mp3","size":26667233},{"name":"4. L''idylle & l''épopée/22 22_L_IDYLLE_LES_MISERABLES_T4.mp3","size":10359302},{"name":"4. L''idylle & l''épopée/23 23_L_IDYLLE_LES_MISERABLES_T4.mp3","size":9437284},{"name":"4. L''idylle & l''épopée/24 24_L_IDYLLE_LES_MISERABLES_T4.mp3","size":24938136},{"name":"4. L''idylle & l''épopée/25 25_L_IDYLLE_LES_MISERABLES_T4.mp3","size":25066449},{"name":"4. L''idylle & l''épopée/26 26_L_IDYLLE_LES_MISERABLES_T4.mp3","size":6539990},{"name":"4. L''idylle & l''épopée/27 27_L_IDYLLE_LES_MISERABLES_T4.mp3","size":11998955},{"name":"4. L''idylle & l''épopée/28 28_L_IDYLLE_LES_MISERABLES_T4.mp3","size":7347906},{"name":"4. L''idylle & l''épopée/29 29_L_IDYLLE_LES_MISERABLES_T4.mp3","size":5682338},{"name":"4. L''idylle & l''épopée/30 30_L_IDYLLE_LES_MISERABLES_T4.mp3","size":31256425},{"name":"4. L''idylle & l''épopée/31 31_L_IDYLLE_LES_MISERABLES_T4.mp3","size":14137653},{"name":"4. L''idylle & l''épopée/32 32_L_IDYLLE_LES_MISERABLES_T4.mp3","size":9550133},{"name":"4. L''idylle & l''épopée/33 33_L_IDYLLE_LES_MISERABLES_T4.mp3","size":18190185},{"name":"4. L''idylle & l''épopée/34_L_IDYLLE_LES_MISERABLES_T4.mp3","size":13747266},{"name":"4. L''idylle & l''épopée/35_L_IDYLLE_LES_MISERABLES_T4.mp3","size":14811397},{"name":"4. L''idylle & l''épopée/36_L_IDYLLE_LES_MISERABLES_T4.mp3","size":12383472},{"name":"4. L''idylle & l''épopée/37_L_IDYLLE_LES_MISERABLES_T4.mp3","size":8097718},{"name":"4. L''idylle & l''épopée/38_L_IDYLLE_LES_MISERABLES_T4.mp3","size":13949983},{"name":"4. L''idylle & l''épopée/39_L_IDYLLE_LES_MISERABLES_T4.mp3","size":8182146},{"name":"4. L''idylle & l''épopée/40_L_IDYLLE_LES_MISERABLES_T4.mp3","size":29780187},{"name":"4. L''idylle & l''épopée/41_L_IDYLLE_LES_MISERABLES_T4.mp3","size":13470166},{"name":"5. Jean ValJean/01 01_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":18247285},{"name":"5. Jean ValJean/02 02_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":13023631},{"name":"5. Jean ValJean/03 03_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":23649407},{"name":"5. Jean ValJean/04 04_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":13196666},{"name":"5. Jean ValJean/05 05_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":9896461},{"name":"5. Jean ValJean/06 06_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":11610511},{"name":"5. Jean ValJean/07 07_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":11422430},{"name":"5. Jean ValJean/08 08_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":10813045},{"name":"5. Jean ValJean/09 09_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":24201950},{"name":"5. Jean ValJean/10 10_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":13882955},{"name":"5. Jean ValJean/11 11_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":31538805},{"name":"5. Jean ValJean/12 12_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":14796614},{"name":"5. Jean ValJean/13 13_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":6200447},{"name":"5. Jean ValJean/14 14_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":17414292},{"name":"5. Jean ValJean/15 15_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":15136415},{"name":"5. Jean ValJean/16 16_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":14157554},{"name":"5. Jean ValJean/17 17_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":19077352},{"name":"5. Jean ValJean/18 18_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":17137603},{"name":"5. Jean ValJean/19 19_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":15522609},{"name":"5. Jean ValJean/20 20_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":16911905},{"name":"5. Jean ValJean/21 21_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":13452457},{"name":"5. Jean ValJean/22 22_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":18248539},{"name":"5. Jean ValJean/23 23_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":30634341},{"name":"5. Jean ValJean/24 24_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":19738981},{"name":"5. Jean ValJean/25 25_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":18682798},{"name":"5. Jean ValJean/26 26_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":22890812},{"name":"5. Jean ValJean/27 27_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":14848441},{"name":"5. Jean ValJean/28 28_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":19155510},{"name":"5. Jean ValJean/29 29_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":29154766},{"name":"5. Jean ValJean/30 30_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":11190462},{"name":"5. Jean ValJean/31 31_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":61236895},{"name":"5. Jean ValJean/32 32_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":16760604},{"name":"5. Jean ValJean/33 33_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":14239461},{"name":"5. Jean ValJean/34 34_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":9072245},{"name":"5. Jean ValJean/35 35_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":48860287},{"name":"5. Jean ValJean/36 36_JEAN_VALJEAN_LES_MISERABLES_T5.mp3","size":31350305}],"parent_folder":"Les Misérables"}', 'none', '', false, 'MP3', '3311208978', NULL, 'aac', NULL, NULL, NULL, NULL, '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (16, 10, '2025-03-31 11:41:42.59817', '2025-03-31 11:41:42.59817', 1, 'English', '', '', '', '{"cue":1,"jpg":1,"m4b":1,"nfo":1}', false, '{"files":[{"name":"Les Misérables.cue","size":21749},{"name":"Les Misérables.jpg","size":58086},{"name":"Les Misérables.m4b","size":1793502945},{"name":"Les Misérables.nfo","size":1406}],"parent_folder":"Victor Hugo, Isabel F. Hapgood - translator - Les Misérables.M4B"}', 'none', '', false, 'M4A', '1793584186', NULL, 'aac', NULL, NULL, NULL, NULL, '{Cue}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (20, 13, '2025-03-31 13:40:01.861757', '2025-03-31 13:40:01.861757', 1, 'English', '', '', 'ripped from casefilepodcast.com', '{"mp3":46}', false, '{"files":[{"name":"2016-04-23 Case 16 Chris & Cru Kahui.mp3","size":84141645},{"name":"2016-01-16 Case 02 The Somerton Man.mp3","size":45786191},{"name":"2016-01-23 Case 03 Lauria Bible & Ashley Freeman.mp3","size":38215303},{"name":"2016-01-30 Case 04 Who Put Bella In The ‘Witch’ Elm.mp3","size":33386207},{"name":"2016-02-06 Case 05 Donna Wheeler.mp3","size":24917614},{"name":"2016-02-13 Case 06 Roger Dean.mp3","size":34471071},{"name":"2016-02-20 Case 07 Julian Buchwald & Carolynne Watson.mp3","size":22220928},{"name":"2016-02-27 Case 08 Holly Wells & Jessica Chapman.mp3","size":34088253},{"name":"2016-03-05 Case 09 Jonathan Luna.mp3","size":37194983},{"name":"2016-03-12 Case 10 Peter Shellard.mp3","size":22624695},{"name":"2016-03-19 Case 11 Anneliese Michel.mp3","size":46114165},{"name":"2016-03-26 Case 12 Katherine Knight.mp3","size":47534755},{"name":"2016-04-02 Case 13 The Family Court Murders.mp3","size":2023758},{"name":"2016-04-09 Case 14 Helen Munnings.mp3","size":26132139},{"name":"2016-04-16 Case 15 The Weepy Voiced Killer.mp3","size":28137474},{"name":"2016-01-09 Case 01 The Wanda Beach Murders.mp3","size":74680017},{"name":"2016-04-30 Case 17 The Eriksson Twins.mp3","size":33966853},{"name":"2016-05-07 Case 18 The North Hollywood Shootout.mp3","size":63229329},{"name":"2016-05-14 Case 19 Snowtown.mp3","size":1909508},{"name":"2016-05-21 Case 20 Stoni Blair & Stephen Berry.mp3","size":34723380},{"name":"2016-05-28 Case 21 Pamela Lawrence.mp3","size":65232328},{"name":"2016-06-11 Case 22 Marguerite Edwards.mp3","size":29244743},{"name":"2016-06-18 Case 23 The Frankston Serial Killer (Part 1).mp3","size":34072490},{"name":"2016-06-25 Case 23 The Frankston Serial Killer (Part 2).mp3","size":30600114},{"name":"2016-07-02 Case 24 Russell Street Bombing (Part 1).mp3","size":24994730},{"name":"2016-07-09 Case 24 Russell Street Bombing (Part 2).mp3","size":38199549},{"name":"2016-07-16 Case 25 Prue Bird.mp3","size":30656191},{"name":"2016-07-23 Case 26 Lisa Marie Young.mp3","size":23864541},{"name":"2016-07-30 Case 27 The West Mesa Bone Collector.mp3","size":33133516},{"name":"2016-08-06 Case 28 Lindsay Buziak.mp3","size":76678687},{"name":"2016-08-13 Case 29 The Burgate House Murders.mp3","size":25223340},{"name":"2016-08-20 Case 30 The Claremont Serial Killer.mp3","size":1945374},{"name":"2016-08-24 Case 28 Lindsay Buziak Follow Up.mp3","size":7737810},{"name":"2016-08-27 Case 31 The Killer Couple.mp3","size":41453567},{"name":"2016-09-04 Case 32 Grace & Kathleen Holmes.mp3","size":35630974},{"name":"2016-09-17 Case 33 Jaycee Lee Dugard.mp3","size":67521083},{"name":"2016-09-24 Case 34 The Catholic Mafia.mp3","size":57822291},{"name":"2016-10-08 Case 35 Operation Mayan.mp3","size":48447335},{"name":"2016-10-15 Case 36 Amok.mp3","size":36727614},{"name":"2016-10-22 Case 37 The Yorkshire Ripper (Part 1).mp3","size":65711808},{"name":"2016-10-29 Case 37 The Yorkshire Ripper (Part 2).mp3","size":79069491},{"name":"2016-11-05 Case 37 The Yorkshire Ripper (Part 3).mp3","size":52381007},{"name":"2016-11-12 Case 38 The Pikuls.mp3","size":81465559},{"name":"2016-12-03 Case 39 Janelle Patton.mp3","size":62046461},{"name":"2016-12-10 Case 40 John Newman.mp3","size":64170702},{"name":"2016-12-23 Case 26 Lisa Marie Young Update.mp3","size":6782709}],"parent_folder":"Casefile True Crime [January 2016 - December 2016 - MP3 - 128kps]"}', 'none', '', false, 'MP3', '1856312282', NULL, 'mp3', NULL, NULL, NULL, NULL, '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (21, 14, '2025-03-31 13:47:59.40496', '2025-03-31 13:47:59.40496', 1, 'English', '', '', '', '{"mp3":39}', false, '{"files":[{"name":"2017-04-15 Case 50 Jennifer Pan.mp3","size":157438970},{"name":"2017-01-14 Case 42 Sherri Rasmussen.mp3","size":65522168},{"name":"2017-01-21 Case 43 Keith Warren.mp3","size":46537256},{"name":"2017-01-28 Case 44 Peter Falconio.mp3","size":112147947},{"name":"2017-02-11 Case 45 Port Arthur.mp3","size":80080569},{"name":"2017-02-18 Case 46 The Frankston and Tynong North Serial Killer.mp3","size":66807254},{"name":"2017-02-25 Case 47 Yara Gambirasio.mp3","size":46009430},{"name":"2017-03-04 Case 48 Suzy Lamplugh.mp3","size":57932360},{"name":"2017-03-18 Case 49 The Moors Murders (Part 1).mp3","size":83922061},{"name":"2017-03-25 Case 49 The Moors Murders (Part 2).mp3","size":100784917},{"name":"2017-04-01 Case 49 The Moors Murders (Part 3).mp3","size":114287270},{"name":"2017-01-07 Case 41 Mr Cruel.mp3","size":64606039},{"name":"2017-04-22 Case 51 Tina Watson.mp3","size":146970272},{"name":"2017-05-06 Case 52 Mary & Beth Stauffer, Jason Wilkman.mp3","size":114525764},{"name":"2017-05-13 Case 53 The East Area Rapist 1976 (Part 1).mp3","size":112535854},{"name":"2017-05-20 Case 53 The East Area Rapist 1977 (Part 2).mp3","size":99735981},{"name":"2017-05-27 Case 53 The East Area Rapist 1977–1978 (Part 3).mp3","size":98859529},{"name":"2017-06-03 Case 53 The East Area Rapist 1978–1979 (Part 4).mp3","size":87437297},{"name":"2017-06-03 Case 53 The East Area RapistOriginal Night Stalker (Part 5).mp3","size":91105271},{"name":"2017-07-01 Case 54 Daniel Morcombe.mp3","size":86897489},{"name":"2017-07-22 Case 56 Anita Cobby.mp3","size":96087691},{"name":"2017-07-29 Case 57 Walsh Street.mp3","size":106566700},{"name":"2017-08-12 Case 58 Shannon Matthews.mp3","size":64091968},{"name":"2017-08-18 Case 59 Amy Lynn Bradley.mp3","size":73565178},{"name":"2017-09-16 Case 60 Jonestown (Part 1).mp3","size":83371292},{"name":"2017-09-16 Case 60 Jonestown (Part 2).mp3","size":61093884},{"name":"2017-09-23 Case 60 Jonestown (Part 3).mp3","size":60678729},{"name":"2017-09-30 Case 61 The Lin Family.mp3","size":64935754},{"name":"2017-10-07 Case 62 The Honolulu Strangler.mp3","size":43861083},{"name":"2017-10-14 Case 63 Catherine Holmes & Georgina Watmore.mp3","size":43272435},{"name":"2017-10-22 Case 46 The Frankston & Tynong North Serial Killer Update.mp3","size":33578369},{"name":"2017-10-28 Case 64 Peter Weinberger.mp3","size":56830105},{"name":"2017-11-04 Case 65 Allison Baden-Clay.mp3","size":92321470},{"name":"2017-11-11 Case 66 The Black Widow.mp3","size":63569089},{"name":"2017-11-18 Case 67 The Battle of Alcatraz.mp3","size":66247627},{"name":"2017-11-25 Case 68 Escape from Alcatraz.mp3","size":63293854},{"name":"2017-12-02 Case 69 Gary Patterson.mp3","size":57791530},{"name":"2017-12-09 Case 70 The Kimberley Killer.mp3","size":55007380},{"name":"2017-12-16 Case 71 Elodie Morel.mp3","size":54914289}],"parent_folder":"Casefile True Crime [January 2017 - December 2017 - MP3 - 128kps]"}', 'none', '', false, 'MP3', '3075222125', NULL, 'mp3', NULL, NULL, NULL, NULL, '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (22, 15, '2025-03-31 13:52:51.246603', '2025-03-31 13:52:51.246603', 1, 'English', '', '', '', '{"mp3":40}', false, '{"files":[{"name":"2018-11-17 Case 100 The Beaumont Children.mp3","size":110142302},{"name":"2018-01-17 Case 73 The Lady in the Barrel.mp3","size":43849447},{"name":"2018-01-20 Case 74 Eric Coy.mp3","size":59610999},{"name":"2018-01-27 Case 75 Graeme Thorne.mp3","size":62378450},{"name":"2018-02-10 Case 76 Silk Road (Part 1).mp3","size":80558656},{"name":"2018-02-17 Case 76 Silk Road (Part 2).mp3","size":77029236},{"name":"2018-02-24 Case 76 Silk Road (Part 3).mp3","size":63717529},{"name":"2018-03-10 Case 77 Mia Zapata.mp3","size":59229595},{"name":"2018-03-17 Case 78 The Janabi Family.mp3","size":102812356},{"name":"2018-03-24 Case 79 Rayna Rison.mp3","size":62883158},{"name":"2018-04-07 Case 80 Beth Barnard.mp3","size":4877695},{"name":"2018-04-14 Case 81 Brian Wells.mp3","size":95412794},{"name":"2018-04-21 Case 82 Maria Korp.mp3","size":96032519},{"name":"2018-04-24 Case 03 Lauria Bible and Ashley Freeman Update.mp3","size":9782076},{"name":"2018-04-26 Case 53 EAR-ONSGolden State Killer Update.mp3","size":35222412},{"name":"2018-05-05 Case 83 Chantelle & Leela McDougall, Tony Popic.mp3","size":63761821},{"name":"2018-05-12 Case 84 Lesley Molseed (Part 1).mp3","size":75870426},{"name":"2018-05-19 Case 84 Lesley Molseed (Part 2).mp3","size":60139749},{"name":"2018-06-02 Case 85 Tom Brown.mp3","size":52797445},{"name":"2018-06-09 Case 86 Amy Allwine.mp3","size":77811479},{"name":"2018-06-16 Case 87 Elaine O’Hara.mp3","size":60954100},{"name":"2018-07-07 Case 88 Stephen Hilder.mp3","size":60222070},{"name":"2018-07-14 Case 89 Ella Tundra.mp3","size":58325550},{"name":"2018-07-28 Case 90 Hoddle Street.mp3","size":66907212},{"name":"2018-08-04 Case 91 Carly Ryan.mp3","size":64489086},{"name":"2018-08-11 Case 92 Dnepropetrovsk Maniacs.mp3","size":60309090},{"name":"2018-08-25 Case 93 Susan Snow & Bruce Nickell.mp3","size":92735022},{"name":"2018-09-01 Case 94 Millie & Trevor Horn, Janice Saunders.mp3","size":71276598},{"name":"2018-09-08 Case 95 The Vampire of Krakow.mp3","size":70619840},{"name":"2018-09-22 Case 96 The Toy Box (Part 1).mp3","size":66204467},{"name":"2018-09-29 Case 96 The Toy Box (Part 2).mp3","size":73296324},{"name":"2018-10-05 Case 96 The Toy Box (Part 3).mp3","size":69117800},{"name":"2018-10-20 Case 97 Rebecca Schaeffer.mp3","size":78189188},{"name":"2018-10-27 Case 98 The Pillow Pyro.mp3","size":93366788},{"name":"2018-11-10 Case 99 Becky Watts.mp3","size":100592147},{"name":"2018-01-13 Case 72 Wilhelmina Kruger & Anna Dowlingkoa.mp3","size":58831209},{"name":"2018-11-24 Case 101 Sian Kingi.mp3","size":85932820},{"name":"2018-12-01 Case 102 Britt Lapthorne.mp3","size":109402327},{"name":"2018-12-15 Case 103 The Gonzales Family.mp3","size":89457389},{"name":"2018-12-22 Case 104 Mark & John.mp3","size":59426340}],"parent_folder":"Casefile True Crime [January 2018 - December 2018 - MP3 - 128kps]"}', 'none', '', false, 'MP3', '2783575511', NULL, 'mp3', NULL, NULL, NULL, NULL, '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (23, 16, '2025-03-31 14:09:39.642721', '2025-03-31 14:09:39.642721', 1, NULL, 'linux', '', '', '{"xz":1}', false, '{"files":[{"name":"yuzu-linux-20240304-537296095.tar.xz","size":7489924}],"parent_folder":""}', 'none', '', false, 'XZ', '7489924', NULL, NULL, NULL, NULL, NULL, NULL, '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (24, 16, '2025-03-31 14:13:10.082476', '2025-03-31 14:13:10.082476', 1, NULL, '', '', '', '{"AppImage":1}', false, '{"files":[{"name":"yuzu-mainline-20240304-537296095.AppImage","size":67220672}],"parent_folder":""}', 'none', '', false, 'AppImage', '67220672', NULL, NULL, NULL, NULL, NULL, NULL, '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (25, 16, '2025-03-31 14:13:59.89458', '2025-03-31 14:13:59.89458', 1, NULL, 'windows', '', '', '{"zip":1}', false, '{"files":[{"name":"yuzu-windows-msvc-20240304-537296095.zip","size":100060892}],"parent_folder":""}', 'none', '', false, 'ZIP', '100060892', NULL, NULL, NULL, NULL, NULL, NULL, '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (26, 17, '2025-03-31 18:40:58.13163', '2025-03-31 18:40:58.13163', 1, 'English', 'Pimp.My.Ride.S01E10.Shondas.Mustang.Convertible.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX', 'FLUX', '', '{"mkv":15}', false, '{"files":[{"name":"Pimp.My.Ride.S01E01.Wyatts.Daihatsu.Hi-Jet.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX.mkv","size":349415938},{"name":"Pimp.My.Ride.S01E02.Niles.Cadillac.Sedan.DeVille.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX.mkv","size":358509537},{"name":"Pimp.My.Ride.S01E03.Logans.Oldsmobile.Cutlass.Supreme.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX.mkv","size":358037970},{"name":"Pimp.My.Ride.S01E04.Christines.Honda.Civic.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX.mkv","size":358767407},{"name":"Pimp.My.Ride.S01E05.Antwons.Mitsubishi.Mirage.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX.mkv","size":346491744},{"name":"Pimp.My.Ride.S01E06.Marys.Ford.Mustang.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX.mkv","size":358595355},{"name":"Pimp.My.Ride.S01E07.Jareds.Ford.Ranger.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX.mkv","size":358666482},{"name":"Pimp.My.Ride.S01E08.Danelles.Pontiac.Trans.Am.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX.mkv","size":350524311},{"name":"Pimp.My.Ride.S01E09.Neils.Chev.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX.mkv","size":346162274},{"name":"Pimp.My.Ride.S01E10.Shondas.Mustang.Convertible.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX.mkv","size":346443667},{"name":"Pimp.My.Ride.S01E11.Ezras.Nissan.Maxima.Station.Wagon.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX.mkv","size":344269866},{"name":"Pimp.My.Ride.S01E12.Krissys.Volkswagen.Baja.Bug.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX.mkv","size":350027035},{"name":"Pimp.My.Ride.S01E13.Brians.Honda.CRX.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX.mkv","size":350356998},{"name":"Pimp.My.Ride.S01E14.Gabys.Toyota.Land.Cruiser.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX.mkv","size":350489018},{"name":"Pimp.My.Ride.S01E15.Kerrys.Ford.Escort.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX.mkv","size":349962251}],"parent_folder":"Pimp.My.Ride.S01.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX"}', 'General
Unique ID : 121305246697873301058184902193704181839 (0x5B428A6B7D82F98C69F9DEACA5B5A04F)
Complete name : Pimp.My.Ride.S01E10.Shondas.Mustang.Convertible.480p.AMZN.WEB-DL.DDP.2.0.H.264-FLUX.mkv
Format : Matroska
Format version : Version 4
File size : 330 MiB
Duration : 20 min 48 s
Overall bit rate : 2 220 kb/s
Movie name : Pimp My Ride - S01E10 - Shondas Mustang Convertible
Description : Shanda''s 1989 Mustang convertible gives new meaning to the term \"rag top.\" This ride is so busted, there''s a gaping hole in the ceiling. there''s also trouble for backseat passengers, one window is missing and the other won''t roll down. And forget about listening to the radio, turning on the heat, or the air conditioning, ''cause none of this stuff works.
Encoded date : UTC 2021-07-21 08:29:00
Writing application : mkvmerge v59.0.0 (''Shining Star'') 64-bit
Writing library : libebml v1.4.2 + libmatroska v1.6.4
IMDB : tt0395891
TMDB : tv/1976

Video
ID : 1
Format : AVC
Format/Info : Advanced Video Codec
Format profile : Main@L3
Format settings : CABAC / 4 Ref Frames
Format settings, CABAC : Yes
Format settings, Reference frames : 4 frames
Codec ID : V_MPEG4/ISO/AVC
Duration : 20 min 48 s
Bit rate mode : Constant
Bit rate : 2 000 kb/s
Width : 640 pixels
Height : 480 pixels
Display aspect ratio : 4:3
Frame rate mode : Constant
Frame rate : 29.970 (30000/1001) FPS
Color space : YUV
Chroma subsampling : 4:2:0
Bit depth : 8 bits
Scan type : Progressive
Bits/(Pixel*Frame) : 0.217
Stream size : 297 MiB (90%)
Language : English
Default : Yes
Forced : No
Color range : Limited

Audio
ID : 2
Format : E-AC-3
Format/Info : Enhanced AC-3
Commercial name : Dolby Digital Plus
Codec ID : A_EAC3
Duration : 20 min 48 s
Bit rate mode : Constant
Bit rate : 224 kb/s
Channel(s) : 2 channels
Channel layout : L R
Sampling rate : 48.0 kHz
Frame rate : 31.250 FPS (1536 SPF)
Compression mode : Lossy
Stream size : 33.3 MiB (10%)
Language : English
Service kind : Complete Main
Default : Yes
Forced : No

Text
ID : 3
Format : UTF-8
Codec ID : S_TEXT/UTF8
Codec ID/Info : UTF-8 Plain Text
Duration : 20 min 41 s
Bit rate : 108 b/s
Count of elements : 774
Stream size : 16.4 KiB (0%)
Title : English [CC]
Language : English
Default : No
Forced : No', '', false, 'MKV', '5276719853', NULL, 'ac3', NULL, NULL, NULL, 'h264', '{}', '[]', NULL);
INSERT INTO "public"."torrents" ("id", "edition_group_id", "created_at", "updated_at", "created_by_id", "language", "release_name", "release_group", "description", "file_amount_per_type", "uploaded_as_anonymous", "file_list", "mediainfo", "trumpable", "staff_checked", "container", "size", "duration", "audio_codec", "audio_bitrate", "audio_bitrate_sampling", "audio_channels", "video_codec", "features", "subtitle_languages", "video_resolution") VALUES (27, 18, '2025-03-31 18:50:33.203661', '2025-03-31 18:50:33.203661', 1, 'English', 'Pimp.My.Ride.S01.EXTRAS.DVDRip.H.264-BTN', 'BTN', '', '{"mkv":5}', false, '{"files":[{"name":"Pimp.My.Ride.S01.Deleted.Scenes.DVDRip.x264-BTN.mkv","size":1324152702},{"name":"Pimp.My.Ride.S01.West.Coast.Customs.What.We.Drive.DVDRip.x264-BTN.mkv","size":165963144},{"name":"Pimp.My.Ride.S01.Xzibit.What.U.See.Is.What.U.Get.DVDRip.x264-BTN.mkv","size":161392549},{"name":"Pimp.My.Ride.S01.Outtakes.DVDRip.x264-BTN.mkv","size":68286421},{"name":"Pimp.My.Ride.S01.Travis.Barker from.Blink.182.DVDRip.x264-BTN.mkv","size":55448336}],"parent_folder":"Pimp.My.Ride.S01.EXTRAS.DVDRip.x264-BTN"}', 'General
Unique ID : 127058491620517286577746533525025030111 (0x5F96934B73D3391A8F596F28E82D7FDF)
Complete name : Pimp.My.Ride.S01.Deleted.Scenes.DVDRip.x264-BTN.mkv
Format : Matroska
Format version : Version 4
File size : 1.23 GiB
Duration : 1 h 9 min
Overall bit rate : 2 542 kb/s
Frame rate : 59.940 FPS
Encoded date : 2024-12-08 13:27:46 UTC
Writing application : mkvmerge v88.0 (''All I Know'') 64-bit
Writing library : libebml v1.4.5 + libmatroska v1.7.1

Video
ID : 1
Format : AVC
Format/Info : Advanced Video Codec
Format profile : High@L3.1
Format settings : CABAC / 9 Ref Frames
Format settings, CABAC : Yes
Format settings, Reference frames : 9 frames
Codec ID : V_MPEG4/ISO/AVC
Duration : 1 h 9 min
Bit rate : 2 347 kb/s
Width : 706 pixels
Height : 476 pixels
Display aspect ratio : 4:3
Original display aspect ratio : 4:3
Frame rate mode : Constant
Frame rate : 59.940 (60000/1001) FPS
Color space : YUV
Chroma subsampling : 4:2:0
Bit depth : 8 bits
Scan type : Progressive
Bits/(Pixel*Frame) : 0.116
Stream size : 1.14 GiB (92%)
Writing library : x264 core 164 r3198 da14df5
Encoding settings : cabac=1 / ref=9 / deblock=1:-3:-3 / analyse=0x3:0x113 / me=umh / subme=8 / psy=1 / psy_rd=0.80:0.00 / mixed_ref=1 / me_range=24 / chroma_me=1 / trellis=2 / 8x8dct=1 / cqm=0 / deadzone=21,11 / fast_pskip=1 / chroma_qp_offset=-2 / threads=15 / lookahead_threads=3 / sliced_threads=0 / nr=0 / decimate=1 / interlaced=0 / bluray_compat=0 / constrained_intra=0 / bframes=6 / b_pyramid=2 / b_adapt=2 / b_bias=0 / direct=3 / weightb=1 / open_gop=0 / weightp=2 / keyint=599 / keyint_min=59 / scenecut=40 / intra_refresh=0 / rc_lookahead=50 / rc=crf / mbtree=0 / crf=18.0 / qcomp=0.60 / qpmin=0 / qpmax=69 / qpstep=4 / vbv_maxrate=17500 / vbv_bufsize=17500 / crf_max=0.0 / nal_hrd=none / filler=0 / ip_ratio=1.40 / pb_ratio=1.30 / aq=3:0.70
Default : Yes
Forced : No

Audio
ID : 2
Format : AC-3
Format/Info : Audio Coding 3
Commercial name : Dolby Digital
Codec ID : A_AC3
Duration : 1 h 9 min
Bit rate mode : Constant
Bit rate : 192 kb/s
Channel(s) : 2 channels
Channel layout : L R
Sampling rate : 48.0 kHz
Frame rate : 31.250 FPS (1536 SPF)
Compression mode : Lossy
Stream size : 95.4 MiB (8%)
Language : English
Service kind : Complete Main
Default : Yes
Forced : No
Dialog Normalization : -31 dB
compr : -0.28 dB
dialnorm_Average : -31 dB
dialnorm_Minimum : -31 dB
dialnorm_Maximum : -31 dB

Menu
00:00:00.000 : en:Chapter 01
00:01:21.334 : en:Chapter 02
00:04:25.101 : en:Chapter 03
00:06:50.635 : en:Chapter 04
00:25:07.935 : en:Chapter 05
00:29:56.202 : en:Chapter 06
00:34:57.235 : en:Chapter 07
00:38:25.735 : en:Chapter 08
00:41:12.436 : en:Chapter 09
00:42:52.870 : en:Chapter 10
00:45:05.137 : en:Chapter 11
00:48:01.270 : en:Chapter 12
00:51:52.871 : en:Chapter 13
00:55:41.405 : en:Chapter 14
01:06:17.905 : en:Chapter 15', '', false, 'MKV', '1775243152', NULL, 'ac3', NULL, NULL, NULL, 'h264', '{}', '[]', NULL);
INSERT INTO "public"."users" ("id", "username", "avatar", "email", "password_hash", "registered_from_ip", "created_at", "description", "uploaded", "downloaded", "ratio", "required_ratio", "last_seen", "class", "forum_posts", "forum_threads", "group_comments", "torrent_comments", "request_comments", "artist_comments", "seeding", "leeching", "snatched", "seeding_size", "requests_filled", "collages_started", "requests_voted", "average_seeding_time", "invited", "invitations", "bonus_points") VALUES (2, 'waterbottle', 'https://i.pinimg.com/736x/a6/27/12/a6271204df8d387c3e614986c106f549.jpg', 'user2@example.com', 'hashedpassword2', '192.168.1.2', '2025-03-30 16:24:57.388152', '', '0', '1', 0, 0, '2025-03-30 16:24:57.388152', 'newbie', 0, 0, 0, 0, 0, '0', 0, 0, 0, '0', '0', '0', '0', '0', '0', 0, '0');
INSERT INTO "public"."users" ("id", "username", "avatar", "email", "password_hash", "registered_from_ip", "created_at", "description", "uploaded", "downloaded", "ratio", "required_ratio", "last_seen", "class", "forum_posts", "forum_threads", "group_comments", "torrent_comments", "request_comments", "artist_comments", "seeding", "leeching", "snatched", "seeding_size", "requests_filled", "collages_started", "requests_voted", "average_seeding_time", "invited", "invitations", "bonus_points") VALUES (3, 'coolguy', 'https://i.pinimg.com/474x/c1/5a/6c/c15a6c91515e22f6ea8b766f89c12f0c.jpg', 'user3@example.com', 'hashedpassword3', '192.168.1.3', '2025-03-30 16:24:57.388152', '', '0', '1', 0, 0, '2025-03-30 16:24:57.388152', 'newbie', 0, 0, 0, 0, 0, '0', 0, 0, 0, '0', '0', '0', '0', '0', '0', 0, '0');
INSERT INTO "public"."users" ("id", "username", "avatar", "email", "password_hash", "registered_from_ip", "created_at", "description", "uploaded", "downloaded", "ratio", "required_ratio", "last_seen", "class", "forum_posts", "forum_threads", "group_comments", "torrent_comments", "request_comments", "artist_comments", "seeding", "leeching", "snatched", "seeding_size", "requests_filled", "collages_started", "requests_voted", "average_seeding_time", "invited", "invitations", "bonus_points") VALUES (1, 'picolo', 'https://img.freepik.com/premium-vector/random-people-line-art-vector_567805-63.jpg', 'user1@example.com', '$argon2id$v=19$m=19456,t=2,p=1$s4XJtCUk9IrGgNsTfP6Ofw$ktoGbBEoFaVgdiTn19Gh9h45LjFiv7AUEL5KHhzm4d0', '192.168.1.1', '2025-03-30 16:24:57.388152', '', '10000', '1', 0, 0, '2025-03-31 19:07:55.032931', 'newbie', 0, 0, 0, 0, 0, '0', 0, 0, 0, '0', '0', '0', '0', '0', '0', 0, '1000000000');
