CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Add up migration script here
CREATE TABLE game (
    id UUID DEFAULT uuid_generate_v4(),

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (id)
);

CREATE TABLE map (
    id UUID DEFAULT uuid_generate_v4(),

    game_id UUID NOT NULL,

    width INT NOT NULL,
    height INT NOT NULL,

    CHECK (width >= 0),
    CHECK (height >= 0),

    PRIMARY KEY (id),
    
    FOREIGN KEY (game_id) REFERENCES game(id)
);

CREATE TABLE map_wall (
    map_id UUID,
    
    x INT NOT NULL,
    y INT NOT NULL,

    PRIMARY KEY (map_id, x, y),

    FOREIGN KEY (map_id) REFERENCES map(id)
);

CREATE TABLE map_starting_position (
    map_id UUID,

    x INT NOT NULL,
    y INT NOT NULL,

    PRIMARY KEY (map_id, x, y),

    FOREIGN KEY (map_id) REFERENCES map(id)
);

CREATE TABLE round (
    game_id UUID,
    round_number INT,
    
    map_id UUID NOT NULL,

    CHECK (round_number >= 0),

    PRIMARY KEY (game_id, round_number),

    FOREIGN KEY (game_id) REFERENCES game(id),
    FOREIGN KEY (map_id) REFERENCES map(id)
);

CREATE TABLE player (
    id INT NOT NULL,
    game_id UUID,
    round_number INT,
    
    x INT NOT NULL,
    y INT NOT NULL,
    rotation TEXT NOT NULL, 

    visible BOOL NOT NULL,

    CHECK (id >= 0),
    CHECK (x >= 0),
    CHECK (y >= 0),
    CHECK (rotation IN ('up', 'down', 'right', 'left')),

    PRIMARY KEY (id, game_id, round_number),

    FOREIGN KEY (game_id) REFERENCES game(id),
    FOREIGN KEY (game_id, round_number) REFERENCES round(game_id, round_number)
);

CREATE TABLE shot_tile (
    game_id UUID,
    round_number INT,
    x INT NOT NULL,
    y INT NOT NULL,

    CHECK (x >= 0),
    CHECK (y >= 0),

    PRIMARY KEY (game_id, round_number, x, y),

    FOREIGN KEY (game_id, round_number) REFERENCES round(game_id, round_number)
);
