use rusqlite::{Connection, Error};

const DB_PATH: &str = "data.db";

pub(crate) fn get_connection() -> Result<Connection, Error> {
    Connection::open(DB_PATH)
}
