INSERT INTO
    game (id, num_players, max_num_rounds, map_dir, started_at, finished_at)
VALUES
    ('7e6eee99-0d5d-4419-ba12-372fdf56324f', 4, 80, './resources/maps/game_map_2.bmp', '2023-05-17 17:04:33.333666+00', '2023-05-17 17:04:34.112051+00');

INSERT INTO
    map (id, game_id, width, height)
VALUES
    (
        'f8ee8442-ae63-4dd1-8c88-04c95f163318',
        '7e6eee99-0d5d-4419-ba12-372fdf56324f',
        16,
        16
    );

INSERT INTO
    map_starting_position (map_id, x, y)
VALUES
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 1, 1),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 14, 1),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 14, 14),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 1, 14);

INSERT INTO
    map_wall (map_id, x, y)
VALUES
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 0, 0),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 1, 0),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 2, 0),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 3, 0),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 4, 0),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 5, 0),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 6, 0),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 7, 0),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 8, 0),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 9, 0),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 10, 0),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 11, 0),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 12, 0),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 13, 0),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 14, 0),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 15, 0),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 0, 1),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 15, 1),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 0, 2),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 2, 2),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 3, 2),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 4, 2),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 6, 2),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 7, 2),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 8, 2),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 9, 2),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 11, 2),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 15, 2),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 0, 3),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 3, 3),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 6, 3),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 11, 3),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 12, 3),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 13, 3),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 15, 3),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 0, 4),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 1, 4),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 3, 4),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 5, 4),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 6, 4),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 8, 4),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 11, 4),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 15, 4),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 0, 5),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 15, 5),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 0, 6),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 2, 6),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 3, 6),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 4, 6),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 5, 6),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 6, 6),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 8, 6),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 10, 6),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 11, 6),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 13, 6),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 15, 6),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 0, 7),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 6, 7),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 10, 7),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 13, 7),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 15, 7),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 0, 8),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 1, 8),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 3, 8),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 4, 8),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 6, 8),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 8, 8),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 10, 8),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 13, 8),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 15, 8),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 0, 9),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 4, 9),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 6, 9),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 10, 9),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 11, 9),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 13, 9),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 15, 9),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 0, 10),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 2, 10),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 4, 10),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 8, 10),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 15, 10),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 0, 11),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 2, 11),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 4, 11),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 6, 11),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 7, 11),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 8, 11),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 9, 11),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 10, 11),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 11, 11),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 12, 11),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 14, 11),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 15, 11),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 0, 12),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 4, 12),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 12, 12),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 15, 12),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 0, 13),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 2, 13),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 3, 13),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 4, 13),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 5, 13),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 6, 13),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 7, 13),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 8, 13),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 9, 13),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 10, 13),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 12, 13),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 13, 13),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 15, 13),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 0, 14),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 15, 14),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 0, 15),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 1, 15),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 2, 15),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 3, 15),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 4, 15),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 5, 15),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 6, 15),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 7, 15),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 8, 15),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 9, 15),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 10, 15),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 11, 15),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 12, 15),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 13, 15),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 14, 15),
    ('f8ee8442-ae63-4dd1-8c88-04c95f163318', 15, 15);

INSERT INTO round(game_id, round_number, map_id)
VALUES
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	0,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	1,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	2,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	3,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	4,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	5,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	6,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	7,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	8,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	9,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	10,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	11,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	12,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	13,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	14,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	15,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	16,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	17,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	18,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	19,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	20,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	21,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	22,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	23,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	24,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	25,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	26,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	27,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	28,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	29,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	30,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	31,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	32,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	33,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	34,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	35,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	36,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	37,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	38,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	39,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	40,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	41,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	42,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	43,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	44,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	45,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	46,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	47,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	48,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	49,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	50,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	51,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	52,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	53,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	54,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	55,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	56,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	57,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	58,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	59,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	60,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	61,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	62,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	63,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	64,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	65,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	66,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	67,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	68,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	69,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	70,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	71,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	72,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	73,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	74,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	75,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	76,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	77,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	78,	'f8ee8442-ae63-4dd1-8c88-04c95f163318'),
('7e6eee99-0d5d-4419-ba12-372fdf56324f',	79,	'f8ee8442-ae63-4dd1-8c88-04c95f163318');

INSERT INTO shot_tile(game_id, round_number, x, y)
VALUES
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 4,	9,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 4,	10,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 4,	6,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 4,	5,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 4,	2,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 4,	3,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 4,	7,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 4,	1,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 4,	8,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 4,	4,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 6,	9,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 6,	1,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 6,	5,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 6,	10,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 6,	2,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 6,	8,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 6,	6,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 6,	3,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 6,	7,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 6,	4,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 10,	3,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 10,	7,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 10,	3,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 10,	5,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 10,	4,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 10,	1,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 10,	5,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 10,	6,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 10,	2,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 10,	2,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 10,	4,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 10,	1,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 14,	4,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 14,	1,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 14,	3,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 14,	2,	1),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 15,	1,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 20,	11,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 20,	7,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 20,	10,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 20,	12,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 20,	9,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 20,	14,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 20,	13,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 20,	8,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 22,	9,	12),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 22,	8,	12),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 22,	10,	12),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 22,	11,	12),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 27,	5,	7),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 27,	3,	7),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 27,	4,	7),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 30,	14,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 30,	13,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 38,	3,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 38,	4,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 38,	1,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 38,	2,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 39,	3,	11),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 39,	3,	12),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 42,	1,	9),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 48,	12,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 48,	11,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 48,	5,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 48,	6,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 48,	9,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 48,	10,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 48,	4,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 48,	7,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 48,	13,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 48,	8,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 48,	14,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 50,	8,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 50,	10,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 50,	3,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 50,	12,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 50,	14,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 50,	11,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 50,	13,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 50,	9,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 50,	7,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 50,	5,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 50,	4,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 50,	6,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 56,	12,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 56,	13,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 56,	14,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 56,	9,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 56,	8,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 56,	10,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 56,	11,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 57,	3,	9),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 61,	1,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 61,	1,	12),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 61,	1,	13),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 72,	4,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 72,	6,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 72,	5,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 72,	3,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 72,	2,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 72,	1,	14),
('7e6eee99-0d5d-4419-ba12-372fdf56324f', 79,	1,	9);


INSERT INTO player (id, game_id, round_number, x, y, rotation, visible)
VALUES
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	0,	14,	1,	'up',	'f'),
(2, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	0,	1,	14,	'up',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	0,	1,	1,	'up',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	0,	14,	14,	'up',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	1,	14,	2,	'down',	'f'),
(2, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	1,	2,	14,	'right',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	1,	1,	2,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	1,	13,	14,	'left',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	2,	1,	3,	'down',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	2,	13,	2,	'left',	'f'),
(2, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	2,	3,	14,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	2,	12,	14,	'left',	'f'),
(2, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	3,	4,	14,	'right',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	3,	12,	2,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	3,	11,	14,	'left',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	3,	2,	3,	'right',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	4,	12,	1,	'up',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	4,	11,	14,	'left',	't'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	4,	2,	4,	'down',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	5,	2,	5,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	5,	10,	14,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	5,	11,	1,	'left',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	6,	2,	5,	'down',	't'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	6,	9,	14,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	6,	11,	1,	'left',	't'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	7,	1,	5,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	7,	10,	1,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	7,	8,	14,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	8,	9,	1,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	8,	7,	14,	'left',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	8,	1,	6,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	9,	6,	14,	'left',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	9,	1,	7,	'down',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	9,	8,	1,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	10,	8,	1,	'left',	't'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	10,	2,	7,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	10,	6,	14,	'left',	't'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	11,	7,	1,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	11,	5,	14,	'left',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	11,	3,	7,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	12,	4,	14,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	12,	6,	1,	'left',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	12,	4,	7,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	13,	3,	14,	'left',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	13,	5,	7,	'right',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	13,	5,	1,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	14,	2,	14,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	14,	5,	1,	'left',	't'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	14,	5,	7,	'right',	't'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	15,	2,	14,	'left',	't'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	15,	5,	8,	'down',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	15,	5,	2,	'down',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	16,	5,	3,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	16,	3,	14,	'right',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	16,	5,	9,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	17,	4,	14,	'right',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	17,	5,	10,	'down',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	17,	4,	3,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	18,	4,	4,	'down',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	18,	5,	11,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	18,	5,	14,	'right',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	19,	4,	5,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	19,	6,	14,	'right',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	19,	5,	12,	'down',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	20,	6,	12,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	20,	6,	14,	'right',	't'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	20,	4,	5,	'down',	't'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	21,	3,	5,	'left',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	21,	7,	12,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	21,	5,	14,	'left',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	22,	7,	12,	'right',	't'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	22,	4,	14,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	22,	2,	5,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	23,	3,	14,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	23,	1,	5,	'left',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	23,	8,	12,	'right',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	24,	9,	12,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	24,	2,	14,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	24,	1,	6,	'down',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	25,	1,	7,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	25,	1,	14,	'left',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	25,	10,	12,	'right',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	26,	11,	12,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	26,	1,	14,	'left',	't'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	26,	2,	7,	'right',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	27,	2,	7,	'right',	't'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	27,	11,	13,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	27,	1,	13,	'up',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	28,	1,	7,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	28,	1,	12,	'up',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	28,	11,	14,	'down',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	29,	1,	6,	'up',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	29,	12,	14,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	29,	2,	12,	'right',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	30,	1,	5,	'up',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	30,	12,	14,	'right',	't'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	30,	3,	12,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	31,	3,	11,	'up',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	31,	1,	5,	'up',	't'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	31,	11,	14,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	32,	3,	10,	'up',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	32,	10,	14,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	32,	1,	6,	'down',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	33,	1,	7,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	33,	3,	9,	'up',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	33,	9,	14,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	34,	3,	9,	'up',	't'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	34,	2,	7,	'right',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	34,	8,	14,	'left',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	35,	7,	14,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	35,	2,	8,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	35,	3,	10,	'down',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	36,	2,	9,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	36,	3,	11,	'down',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	36,	6,	14,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	37,	3,	12,	'down',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	37,	3,	9,	'right', 'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	37,	5,	14,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	38,	2,	12,	'left',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	38,	5,	14,	'left',	't'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	38,	3,	10,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	39,	1,	12,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	39,	3,	10,	'down',	't'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	39,	4,	14,	'left',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	40,	3,	14,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	40,	3,	11,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	40,	1,	11,	'up',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	41,	3,	12,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	41,	1,	10,	'up',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	41,	2,	14,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	42,	1,	10,	'up',	't'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	42,	1,	14,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	42,	2,	12,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	43,	1,	12,	'left',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	43,	1,	14,	'left',	't'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	43,	1,	11,	'down',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	44,	1,	13,	'up',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	44,	1,	12,	'left',	't'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	44,	1,	11,	'down',	't'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	45,	1,	11,	'down',	't'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	45,	1,	14,	'down',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	45,	1,	12,	'left',	't'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	46,	1,	13,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	46,	1,	11,	'down',	't'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	46,	2,	14,	'right',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	47,	1,	14,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	47,	1,	12,	'down',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	47,	3,	14,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	48,	1,	13,	'down',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	48,	3,	14,	'right',	't'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	48,	2,	14,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	49,	1,	14,	'down',	'f'),
(1, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	49,	3,	14,	'right',	't'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	49,	2,	14,	'right',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	50,	2,	14,	'right',	't'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	50,	1,	14,	'down',	't'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	51,	1,	13,	'up',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	51,	3,	14,	'right',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	52,	4,	14,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	52,	1,	12,	'up',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	53,	5,	14,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	53,	1,	11,	'up',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	54,	6,	14,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	54,	1,	10,	'up',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	55,	7,	14,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	55,	1,	9,	'up',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	56,	2,	9,	'right',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	56,	7,	14,	'right',	't'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	57,	2,	9,	'right',	't'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	57,	8,	14,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	58,	1,	9,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	58,	9,	14,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	59,	1,	10,	'down',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	59,	10,	14,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	60,	1,	11,	'down',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	60,	11,	14,	'right',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	61,	12,	14,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	61,	1,	11,	'down',	't'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	62,	13,	14,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	62,	1,	10,	'up',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	63,	1,	9,	'up',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	63,	14,	14,	'right',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	64,	2,	9,	'right',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	64,	14,	14,	'right',	't'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	65,	3,	9,	'right',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	65,	13,	14,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	66,	3,	9,	'right',	't'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	66,	12,	14,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	67,	3,	10,	'down',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	67,	11,	14,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	68,	10,	14,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	68,	3,	11,	'down',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	69,	9,	14,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	69,	3,	12,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	70,	2,	12,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	70,	8,	14,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	71,	7,	14,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	71,	1,	12,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	72,	7,	14,	'left',	't'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	72,	1,	13,	'down',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	73,	1,	14,	'down',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	73,	6,	14,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	74,	5,	14,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	74,	1,	14,	'down',	't'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	75,	1,	13,	'up',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	75,	4,	14,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	76,	1,	12,	'up',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	76,	3,	14,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	77,	2,	14,	'left',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	77,	1,	11,	'up',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	78,	1,	10,	'up',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	78,	1,	14,	'left',	'f'),
(3, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	79,	1,	13,	'up',	'f'),
(0, '7e6eee99-0d5d-4419-ba12-372fdf56324f',	79,	1,	10,	'up',	't');
