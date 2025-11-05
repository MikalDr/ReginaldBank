CREATE TABLE
  IF NOT EXISTS "character" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    playername TEXT UNIQUE NOT NULL,
    class TEXT NOT NULL
  );

CREATE TABLE
  IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    active_character INTEGER NOT NULL,
    FOREIGN KEY (active_character) REFERENCES "character" (id)
  );

CREATE TABLE
  IF NOT EXISTS holdings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER,
    item TEXT UNIQUE NOT NULL,
    count INTEGER NOT NULL,
    added TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (player_id) REFERENCES "character" (id)
  );

CREATE TABLE
  IF NOT EXISTS commands (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER,
    command TEXT NOT NULL,
    args INTEGER,
    executed TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (player_id) REFERENCES users (id)
  );

INSERT INTO
  holdings (item, count)
VALUES
  ("pp", 0),
  ("ep", 0),
  ("gp", 0),
  ("sp", 0),
  ("cp", 0) ON CONFLICT (item) DO NOTHING;