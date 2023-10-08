-- Add migration script here
CREATE TABLE leagues (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    unique (name)
);

CREATE TABLE users (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    unique (name)
);

CREATE TABLE participants (
    leagueid UUID REFERENCES leagues(id),
    userid UUID REFERENCES users(id),
    PRIMARY KEY (leagueid, userid)
);

CREATE TABLE matches (
    id UUID PRIMARY KEY,
    leagueId UUID REFERENCES leagues(id),
    homeUserId UUID REFERENCES users(id),
    awayUserId UUID REFERENCES users(id),
    homeScore INT,
    awayScore INT,
    dateScheduled TIMESTAMPTZ NOT NULL,
    datePlayed TIMESTAMPTZ,
    FOREIGN KEY (leagueId) REFERENCES leagues(id),
    FOREIGN KEY (homeUserId) REFERENCES users(id),
    FOREIGN KEY (awayUserId) REFERENCES users(id)
);
