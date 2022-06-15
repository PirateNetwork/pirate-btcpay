use std::path::Path;
use rusqlite::{Connection, NO_PARAMS, OptionalExtension};
use crate::lw_rpc::BlockId;
use crate::Result;

pub struct Db {
    connection: Connection,
}

impl Db {
    pub fn open(path: &Path) -> Result<Self> {
        let connection = Connection::open(path)?;
        Self::create_schema(&connection)?;
        Ok(Db {
            connection
        })
    }

    fn create_schema(connection: &Connection) -> Result<()> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS blocks (
            height INTEGER PRIMARY KEY,
            hash BLOB NOT NULL)",
            NO_PARAMS,
        )?;
        Ok(())
    }

    pub fn get_last_synced(&self) -> Result<Option<BlockId>> {
        let block_id = self.connection.query_row("SELECT hash, height FROM blocks WHERE height = (SELECT MAX(height) FROM blocks)",
        NO_PARAMS, |row| {
                let hash: Vec<u8> = row.get(0)?;
                let height: u32 = row.get(1)?;
                Ok(BlockId {
                    hash,
                    height: height as u64,
                })
            }).optional()?;
        Ok(block_id)
    }

    pub fn rewind_blocks(&self, height: u64) -> Result<()> {
        self.connection.execute("DELETE FROM blocks WHERE height > ?1", [height as u32])?;
        Ok(())
    }
}
