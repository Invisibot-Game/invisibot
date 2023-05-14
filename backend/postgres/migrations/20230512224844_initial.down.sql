-- Add down migration script here

DROP TABLE shot_tile;
DROP TABLE player;
DROP TABLE round;
DROP TABLE map_starting_position;
DROP TABLE map_wall;
DROP TABLE map;
DROP TABLE game;
DROP EXTENSION "uuid-ossp";