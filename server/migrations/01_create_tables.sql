CREATE TABLE IF NOT EXISTS games (
    id SERIAL PRIMARY KEY,
    team_home VARCHAR NOT NULL,
    team_away VARCHAR NOT NULL,
    sport_type VARCHAR NOT NULL,
    timer INT DEFAULT 0,
    score_home INT DEFAULT 0,
    score_away INT DEFAULT 0,
    status VARCHAR DEFAULT 'scheduled'
);

CREATE TABLE IF NOT EXISTS events (
    id SERIAL PRIMARY KEY,
    game_id INT REFERENCES games(id),
    event_type VARCHAR NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT now(),
    description TEXT
);

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR UNIQUE NOT NULL,
    password_hash VARCHAR NOT NULL,
    role VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS devices (
    id SERIAL PRIMARY KEY,
    device_name VARCHAR NOT NULL,
    connection_params JSONB NOT NULL
);

CREATE TABLE IF NOT EXISTS settings (
    id SERIAL PRIMARY KEY,
    param_key VARCHAR UNIQUE NOT NULL,
    param_value VARCHAR NOT NULL
);
