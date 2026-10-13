use std::path::Path;

use rusqlite::Connection;

use crate::error::Result;

pub struct Catalog {
    #[allow(dead_code)]
    conn: std::sync::Mutex<Connection>,
}

impl Catalog {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        let conn = Connection::open(path)?;
        conn.execute_batch(include_str!("schema.sql"))?;
        Ok(Self {
            conn: std::sync::Mutex::new(conn),
        })
    }
}
