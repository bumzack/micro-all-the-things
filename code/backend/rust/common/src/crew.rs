use std::fmt::{Debug};

use serde::Deserialize;
use serde::Serialize;
use crate::entity::{EntityConverter, get_nullable_string, get_nullable_string_list};
use crate::tsv::{TsvLine, TsvLines};

#[derive(Debug, Deserialize, Serialize)]
pub struct Crew {
    pub id: String,
    pub tconst: String,
    pub directors: Option<Vec<String>>,
    pub writers: Option<Vec<String>>,
}

fn map_to_crew(tsv_line: &TsvLine) -> Crew {
    // println!("mapping tsv_line {:?} to crew  ", &tsv_line);

    let tconst = get_nullable_string(&tsv_line.entries, 0).unwrap();
    let directors = get_nullable_string_list(&tsv_line.entries, 1);
    let writers = get_nullable_string_list(&tsv_line.entries, 2);
    let id = tconst.clone();

    Crew {
        id,
        tconst,
        directors,
        writers,
    }
}

impl EntityConverter<Crew> for TsvLines {
    fn convert(&self) -> Vec<Crew> {
        self.lines.iter().map(|t| map_to_crew(&t)).collect()
    }
}
