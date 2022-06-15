use std::path::Path;
use rusqlite::Connection;
use crate::Result;

pub struct Db {
    connection: Connection,
}

impl Db {
    pub fn open(path: &Path) -> Result<Self> {
        let connection = Connection::open(path)?;
        Ok(Db {
            connection
        })
    }
}
