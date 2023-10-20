use rusqlite::Connection;

use crate::utils::get_path;

fn connect_to_db()->Connection{
    let conn = Connection::open(get_path("env.db")).expect("Failed to connect to db");
    return conn
}

fn prepare_db(){
    let conn = connect_to_db();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS envs (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            key TEXT NOT NULL,
            value TEXT NOT NULL
        )",
        [],
    ).expect("Failed to create table");
}
