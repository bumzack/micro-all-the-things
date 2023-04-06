use std::fmt::{Debug, Formatter};

use serde::Deserialize;
use serde::Serialize;

use crate::entity::Entity;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TsvFileImportRequest {
    pub tsv_type: Entity,
    pub start: i32,
    pub end: i32,
    pub page_size: i32,
}

#[derive(Deserialize, Serialize)]
pub struct TsvLine {
    pub original: String,
    pub entries: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TsvLines {
    pub lines: Vec<TsvLine>,
}


impl Debug for TsvLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsvLine")
            .field("original", &self.original)
            .field("entries", &self.entries.join(" // "))
            .finish()
    }
}