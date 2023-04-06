use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

use crate::entity::{
    get_nullable_bool, get_nullable_string, get_nullable_string_list, get_nullable_u32,
    EntityConverter,
};
use crate::tsv::{TsvLine, TsvLines};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieAkas {
    pub id: String,
    pub title_id: String,
    pub ordering: u32,
    pub title: Option<String>,
    pub region: Option<String>,
    pub language: Option<String>,
    pub types: Option<Vec<String>>,
    pub attributes: Option<Vec<String>>,
    pub original_title: Option<bool>,
}

fn map_to_movieaka(tsv_line: &TsvLine) -> MovieAkas {
    // println!("mapping tsv_line {:?} to MovieAkas  ", &tsv_line);

    let title_id = get_nullable_string(&tsv_line.entries, 0).unwrap();
    let ordering = get_nullable_u32(&tsv_line.entries, 1).expect("ordering should be there");
    let title = get_nullable_string(&tsv_line.entries, 2);
    let region = get_nullable_string(&tsv_line.entries, 3);
    let language = get_nullable_string(&tsv_line.entries, 4);
    let types = get_nullable_string_list(&tsv_line.entries, 5);
    let attributes = get_nullable_string_list(&tsv_line.entries, 6);
    let original_title = get_nullable_bool(&tsv_line.entries, 7);
    let id = format!("{}_{}", title_id, ordering);

    MovieAkas {
        id,
        title_id,
        ordering,
        title,
        region,
        language,
        types,
        attributes,
        original_title,
    }
}

impl EntityConverter<MovieAkas> for TsvLines {
    fn convert(&self) -> Vec<MovieAkas> {
        self.lines.iter().map(|t| map_to_movieaka(&t)).collect()
    }
}
