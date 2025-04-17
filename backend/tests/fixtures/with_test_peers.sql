INSERT INTO
    peers(torrent_id, peer_id, ip, port)
VALUES
    (1, '\x2222', '10.10.4.89', 24),
    (1, '\x3333', '10.10.4.90', 25),
    (1, '\x3333', '10.10.4.91', 26);

INSERT INTO
    user_peers(user_id, peer_id)
VALUES
    (2, 1),
    (2, 2),
    (1, 3);
