use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

use crate::entity::{
    EntityConverter, get_nullable_string, get_nullable_string_list_of_string_array,
    get_nullable_u32,
};
use crate::tsv::{TsvLine, TsvLines};

#[derive(Debug, Deserialize, Serialize)]
pub struct Principal {
    pub id: String,
    pub tconst: String,
    pub ordering: u32,
    pub nconst: String,
    pub category: Option<String>,
    pub job: Option<String>,
    pub characters: Option<Vec<String>>,
}

fn map_to_principal(tsv_line: &TsvLine) -> Principal {
    // info!("mapping tsv_line {:?} to principal", &tsv_line);

    let tconst = get_nullable_string(&tsv_line.entries, 0).unwrap();
    let ordering = get_nullable_u32(&tsv_line.entries, 1).expect("ordering should be there");
    let nconst = get_nullable_string(&tsv_line.entries, 2).unwrap();
    let category = get_nullable_string(&tsv_line.entries, 3);
    let job = get_nullable_string(&tsv_line.entries, 4);
    let characters = get_nullable_string_list_of_string_array(&tsv_line.entries, 5);
    let id = format!("{}_{}_{}", tconst, ordering, nconst);

    Principal {
        id,
        tconst,
        ordering,
        nconst,
        job,
        category,
        characters,
    }
}

impl EntityConverter<Principal> for TsvLines {
    fn convert(&self) -> Vec<Principal> {
        self.lines.iter().map(|t| map_to_principal(&t)).collect()
    }
}
