use std::path::Path;

use bobine_core::{Catalog, ClipSummary, Volume};

pub struct BobineService {
    catalog: Catalog,
}

impl BobineService {
    pub fn open(db_path: &Path) -> Result<Self, bobine_core::Error> {
        Ok(Self {
            catalog: Catalog::open(db_path)?,
        })
    }

    pub fn list_volumes(&self) -> Result<Vec<Volume>, bobine_core::Error> {
        self.catalog.list_volumes()
    }

    pub fn list_clips(&self) -> Result<Vec<ClipSummary>, bobine_core::Error> {
        self.catalog.list_clips()
    }
}
