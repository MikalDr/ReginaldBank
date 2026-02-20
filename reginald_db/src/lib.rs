use anyhow::Result;
use reginald_ast::ast::{
    base::{env::ReginaldEnv, money::Denomination},
    typechecker::typed_ast::Value,
};
use reginald_conf::Player;
use sqlite::{Connection, ConnectionThreadSafe, State};
use std::str::FromStr;

#[cfg(test)]
mod tests;

/// # Query
/// ```sql
/// CREATE TABLE
///   IF NOT EXISTS "character" (
///     id INTEGER PRIMARY KEY AUTOINCREMENT,
///     playername TEXT UNIQUE NOT NULL,
///     class TEXT NOT NULL
///   );
///
/// CREATE TABLE
///   IF NOT EXISTS users (
///     id INTEGER PRIMARY KEY AUTOINCREMENT,
///     username TEXT UNIQUE NOT NULL,
///     active_character INTEGER NOT NULL,
///     FOREIGN KEY (active_character) REFERENCES "character" (id)
///   );
/// CREATE TABLE
///   IF NOT EXISTS holdings (
///     id INTEGER PRIMARY KEY AUTOINCREMENT,
///     player_id INTEGER,
///     item TEXT UNIQUE NOT NULL,
///     count INTEGER NOT NULL,
///     added TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
///     FOREIGN KEY (player_id) REFERENCES "character" (id)
///   );
///
/// CREATE TABLE
///   IF NOT EXISTS commands (
///     id INTEGER PRIMARY KEY AUTOINCREMENT,
///     player_id INTEGER,
///     command TEXT NOT NULL,
///     args INTEGER,
///     executed TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
///     FOREIGN KEY (player_id) REFERENCES users (id)
///   );
///
/// INSERT INTO
///   holdings (item, count)
/// VALUES
///   ("pp", 0),
///   ("ep", 0),
///   ("gp", 0),
///   ("sp", 0),
///   ("cp", 0) ON CONFLICT (item) DO NOTHING;
/// ```
const SETUP_QUERIES: &str = include_str!("../queries/db_setup.sql");

/// # Query
/// ```sql
/// INSERT INTO "character" (playername, class) VALUES (:name, :class);
/// ```
const INSERT_PLAYER_QUERY: &str = include_str!("../queries/db_insert_character.sql");

/// # Query
/// ```sql
/// SELECT
///   item,
///   count
/// FROM
///   holdings
/// WHERE
///   item = "PP"
///   OR ITEM = "EP"
///   OR item = "GP"
///   OR item = "SP"
///   OR item = "CP"
/// ```
const GET_ENV_QUERY: &str = include_str!("../queries/get_env.sql");

/// # Query
/// ```sql
/// DATE "holdings"
/// SET count = CASE item
///     WHEN 'pp' THEN :pp
///     WHEN 'ep' THEN :ep
///     WHEN 'gp' THEN :gp
///     WHEN 'sp' THEN :sp
///     WHEN 'cp' THEN :cp
///     ELSE count
/// END
/// WHERE item IN ('pp', 'ep', 'gp', 'sp', 'cp');
/// ```
const UPDATE_ENV_QUERY: &str = include_str!("../queries/update_env.sql");

pub struct DBHandle {
    conn: ConnectionThreadSafe,
}

impl DBHandle {
    /// Initialises the database, creating tables if they dont exist
    pub fn new() -> Result<Self> {
        let conn = Connection::open_thread_safe(".reginald_db")?;
        conn.execute(SETUP_QUERIES)?;
        Ok(Self { conn })
    }

    pub fn get_env(&self) -> Result<ReginaldEnv> {
        let mut stmt = self.conn.prepare(GET_ENV_QUERY)?;
        let mut env = ReginaldEnv::default();
        while let Ok(State::Row) = stmt.next() {
            let denomination = stmt.read::<String, _>("item")?;
            let count = stmt.read::<i64, _>("count")?;
            env = env.checked_add(Value::Piece(count, Denomination::from_str(&denomination)?))?;
        }

        Ok(env)
    }

    pub fn update_env(&self, env: ReginaldEnv) -> Result<()> {
        let mut stmt = self.conn.prepare(UPDATE_ENV_QUERY)?;
        stmt.bind(
            &[
                (":pp", env.pp),
                (":ep", env.ep),
                (":gp", env.gp),
                (":sp", env.sp),
                (":cp", env.cp),
            ][..],
        )?;
        while let Ok(res) = stmt.next() {
            if res == State::Done {
                break;
            }
        }
        Ok(())
    }

    pub fn add_player(&self, player: &Player) -> Result<()> {
        let mut stmt = self.conn.prepare(INSERT_PLAYER_QUERY)?;
        stmt.bind(&[(":name", player.name()), (":class", player.class())][..])?;
        while let Ok(res) = stmt.next() {
            if res == State::Done {
                break;
            }
        }
        Ok(())
    }
}
