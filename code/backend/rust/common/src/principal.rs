use std::fmt::{Debug};

use serde::Deserialize;
use serde::Serialize;
use crate::entity::{EntityConvert, get_nullable_string, get_nullable_string_list, get_nullable_u32};
use crate::tsv::{TsvLine, TsvLines};

#[derive(Debug, Deserialize, Serialize)]
pub struct Principal {
    pub id: String,
    pub tconst: String,
    pub ordering: u32,
    pub nconst: String,
    pub category: Option<String>,
    pub characters: Option<Vec<String>>,
}

fn map_to_principal(tsv_line: &TsvLine) -> Principal {
    // println!("mapping tsv_line {:?} to principal", &tsv_line);

    let ordering = get_nullable_u32(&tsv_line.entries, 1).expect("ordering should be there");

    let characters = get_nullable_string_list(&tsv_line.entries, 4);
    let tconst = get_nullable_string(&tsv_line.entries, 0).unwrap();
    let nconst = get_nullable_string(&tsv_line.entries, 2).unwrap();
    let id = format!("{}_{}_{}", tconst, ordering, nconst);
    let category = get_nullable_string(&tsv_line.entries, 3);

    Principal {
        id,
        tconst,
        ordering,
        nconst,
        category,
        characters,
    }
}

impl EntityConvert<Principal> for TsvLines {
    fn convert(&self) -> Vec<Principal> {
        self.lines.iter().map(|t| map_to_principal(&t)).collect()
    }
}
