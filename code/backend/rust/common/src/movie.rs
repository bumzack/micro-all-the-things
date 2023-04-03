use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

use crate::entity::{EntityConverter, get_nullable_bool, get_nullable_string, get_nullable_string_list, get_nullable_u32};
use crate::tsv::{TsvLine, TsvLines};

#[derive(Debug, Deserialize, Serialize)]
pub struct Movie {
    pub id: String,
    pub tconst: String,
    #[serde(rename = "titleType")]
    pub title_type: Option<String>,
    #[serde(rename = "primaryTitle")]
    pub primary_title: Option<String>,
    #[serde(rename = "originalTitle")]
    pub original_title: Option<String>,
    #[serde(rename = "adult")]
    pub adult: Option<bool>,
    #[serde(rename = "startYear")]
    pub start_year: Option<u32>,
    #[serde(rename = "endYear")]
    pub end_year: Option<u32>,
    #[serde(rename = "runtimeMinutes")]
    pub runtime_minutes: Option<u32>,
    #[serde(rename = "genres")]
    pub genres: Option<Vec<String>>,
}

fn map_to_movie(tsv_line: &TsvLine) -> Movie {
    // println!("mapping tsv_line {:?} to Movie  ", &tsv_line);

    let tconst = get_nullable_string(&tsv_line.entries, 0).unwrap();
    let title_type = get_nullable_string(&tsv_line.entries, 1);
    let primary_title = get_nullable_string(&tsv_line.entries, 2);
    let original_title = get_nullable_string(&tsv_line.entries, 3);
    let adult = get_nullable_bool(&tsv_line.entries, 4);
    let start_year = get_nullable_u32(&tsv_line.entries, 5);
    let end_year = get_nullable_u32(&tsv_line.entries, 6);
    let runtime_minutes = get_nullable_u32(&tsv_line.entries, 7);
    let genres = get_nullable_string_list(&tsv_line.entries, 8);

    let id = tconst.clone();
    Movie {
        id,
        tconst,
        title_type,
        primary_title,
        original_title,
        adult,
        start_year,
        end_year,
        runtime_minutes,
        genres,
    }
}

impl EntityConverter<Movie> for TsvLines {
    fn convert(&self) -> Vec<Movie> {
        self.lines.iter().map(|t| map_to_movie(&t)).collect()
    }
}
