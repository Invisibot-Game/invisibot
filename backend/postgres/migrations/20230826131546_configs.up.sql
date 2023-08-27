-- Add up migration script here
ALTER TABLE game 
ADD COLUMN 
client_response_timeout_millis INT NOT NULL DEFAULT 250;

CREATE TABLE config(
    config_name VARCHAR(50) NOT NULL,
    config_value VARCHAR(200) NOT NULL,
    config_type VARCHAR(20) NOT NULL,

    PRIMARY KEY (config_name),

    CHECK (config_type IN ('u32'))
);

INSERT INTO config(config_name, config_value, config_type)
VALUES ('client_connect_response_timeout_millis', '2000', 'u32');
