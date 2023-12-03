/// This file is responsible for all the database operations
/// The database used is SQLite
/// Handles the basic CRUD operations

use rusqlite::{params, Connection};

use crate::{
    file::join_app_path,
    utils::{DisplayEnv, Env},
};

pub struct Entry {
    pub id: i32,
    pub name: String,
    pub key: String,
    pub value: Vec<u8>,
}

/// Connects to the database.
/// 
/// # Returns
/// 
/// Returns a `Connection` object representing the connection to the database.
fn connect_to_db() -> Connection {
    Connection::open(join_app_path("env.db")).expect("Failed to connect to db")
}

/// Prepares the database for usage.
///
/// This function initializes the necessary resources and configurations for the database.
pub fn prepare_db() {
    let conn = connect_to_db();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS envs (
            id INTEGER PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            key TEXT NOT NULL,
            value BLOB NOT NULL
        )",
        [],
    )
    .expect("Failed to create table");
}

/// Inserts an environment variable into the database.
///
/// # Arguments
///
/// * `env` - The environment variable to be inserted.
///
/// # Returns
///
/// Returns `true` if the insertion is successful, `false` otherwise.
pub fn insert_env(env: Env) -> bool {
    let conn = connect_to_db();
    let mut stmt = conn
        .prepare("INSERT OR REPLACE INTO envs (name, key, value) VALUES (?1, ?2, ?3)")
        .expect("Failed to prepare");
    let _ = stmt
        .execute(params![env.name, env.key, env.value])
        .expect("Failed to execute");
    true
}

/// Retrieves an entry from the database by its name.
///
/// # Arguments
///
/// * `name` - The name of the entry to retrieve.
///
/// # Returns
///
/// An `Option<Entry>` representing the retrieved entry, or `None` if no entry with the given name exists.
pub fn get_by_name(name: &str) -> Option<Entry> {
    let conn = connect_to_db();
    let mut stmt = conn
        .prepare("SELECT * FROM envs WHERE name = ?1")
        .expect("Failed to prepare");
    let mut rows = stmt.query(params![name]).expect("Failed to query");
    let row = rows
        .next()
        .expect("Failed to get row")
        .expect("Failed to get row");
    let id: i32 = row.get(0).expect("Failed to get id");
    let name: String = row.get(1).expect("Failed to get name");
    let key: String = row.get(2).expect("Failed to get key");
    let value: Vec<u8> = row.get(3).expect("Failed to get value");

    Some(Entry {
        id,
        name,
        key,
        value,
    })
}

/// Checks if a record with the given name exists in the database.
///
/// # Arguments
///
/// * `name` - The name of the record to check.
///
/// # Returns
///
/// Returns `true` if a record with the given name exists in the database, otherwise `false`.
pub fn does_exist(name: &str) -> bool {
    let conn = connect_to_db();
    let mut stmt = conn
        .prepare("SELECT * FROM envs WHERE name = ?1")
        .expect("Failed to prepare");
    let mut rows = stmt.query(params![name]).expect("Failed to query");
    let row = rows.next().expect("Failed to get row");
    row.is_some()
}

pub fn delete_entry_by_name(name: &str) -> bool {
    let conn = connect_to_db();
    let mut stmt = conn
        .prepare("DELETE FROM envs WHERE name = ?1")
        .expect("Failed to prepare");
    let _ = stmt.execute(params![name]).expect("Failed to execute");
    true
}

/// Retrieves all names from the database.
///
/// # Returns
///
/// A vector of `DisplayEnv` structs representing the names.
pub fn get_all_names() -> Vec<DisplayEnv> {
    let conn = connect_to_db();
    let mut stmt = conn
        .prepare("SELECT * FROM envs")
        .expect("Failed to prepare");
    let mut rows = stmt.query([]).expect("Failed to query");
    let mut envs = Vec::new();

    while let Some(row) = rows.next().expect("Failed to get row") {
        let id: i32 = row.get(0).expect("Failed to get id");
        let name: String = row.get(1).expect("Failed to get name");
        let key: String = row.get(2).expect("Failed to get key");
        let value: Vec<u8> = row.get(3).expect("Failed to get value");

        let env = Entry {
            id,
            name,
            key,
            value,
        };

        let env = crate::utils::decrypt_struct(env);

        envs.push(env);
    }

    envs
}
