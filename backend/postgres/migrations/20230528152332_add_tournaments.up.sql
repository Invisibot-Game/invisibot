-- Add up migration script here
CREATE TABLE tournament (
    tournament_id UUID DEFAULT uuid_generate_v4(),
    tournament_name TEXT NOT NULL,
    PRIMARY KEY(tournament_id)
);

CREATE TABLE contestant (
    contestant_id UUID DEFAULT uuid_generate_v4(),
    contestant_name TEXT NOT NULL,
    contestant_dockerfile TEXT NOT NULL,
    tournament_id UUID,
    PRIMARY KEY(contestant_id),
    CONSTRAINT fk_tournament_id
        FOREIGN KEY(tournament_id)
            REFERENCES tournament(tournament_id)
            ON DELETE CASCADE
);
