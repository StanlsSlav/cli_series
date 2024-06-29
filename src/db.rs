use rusqlite::Connection;

pub(crate) struct DbContext {}

impl DbContext {
    pub(crate) fn new() -> rusqlite::Result<Connection> {
        Connection::open("data.db")
    }
}
