-- Add down migration script here
DROP TABLE config;

ALTER TABLE game 
DROP COLUMN client_response_timeout_millis;