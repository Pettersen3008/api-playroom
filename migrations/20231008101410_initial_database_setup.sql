-- Add migration script here
CREATE TABLE leagues (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    unique (name)
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    unique (name)
);

CREATE TABLE participants (
    leagueid INT REFERENCES leagues(id),
    userid INT REFERENCES users(id),
    PRIMARY KEY (leagueid, userid)
);

CREATE TABLE matches (
    id SERIAL PRIMARY KEY,
    leagueId INT REFERENCES leagues(id),
    homeUserId INT REFERENCES users(id),
    awayUserId INT REFERENCES users(id),
    homeScore INT,
    awayScore INT,
    dateScheduled TIMESTAMPTZ NOT NULL,
    datePlayed TIMESTAMPTZ,
    FOREIGN KEY (leagueId) REFERENCES leagues(id),
    FOREIGN KEY (homeUserId) REFERENCES users(id),
    FOREIGN KEY (awayUserId) REFERENCES users(id)
);
