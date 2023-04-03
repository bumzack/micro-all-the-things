use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

use crate::entity::{EntityConverter, get_nullable_string, get_nullable_string_list, get_nullable_u32};
use crate::tsv::{TsvLine, TsvLines};

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    pub id: String,
    pub nconst: String,
    #[serde(rename = "primaryName")]
    pub primary_name: Option<String>,
    #[serde(rename = "birthYear")]
    pub birth_year: Option<u32>,
    #[serde(rename = "deathYear")]
    pub death_year: Option<u32>,
    #[serde(rename = "primaryProfession")]
    pub primary_profession: Option<Vec<String>>,
    #[serde(rename = "knownForTitles")]
    pub known_for_titles: Option<Vec<String>>,
}

fn map_to_person(tsv_line: &TsvLine) -> Person {
    // println!("mapping tsv_line {:?} to person  ", &tsv_line);

    let nconst = get_nullable_string(&tsv_line.entries, 0).unwrap();
    let primary_name = get_nullable_string(&tsv_line.entries, 1);
    let birth_year = get_nullable_u32(&tsv_line.entries, 2);
    let death_year = get_nullable_u32(&tsv_line.entries, 3);
    let primary_profession = get_nullable_string_list(&tsv_line.entries, 4);
    let known_for_titles = get_nullable_string_list(&tsv_line.entries, 5);

    let id = nconst.clone();

    Person {
        id,
        nconst,
        primary_name,
        birth_year,
        death_year,
        primary_profession,
        known_for_titles,
    }
}

impl EntityConverter<Person> for TsvLines {
    fn convert(&self) -> Vec<Person> {
        self.lines.iter().map(|t| map_to_person(&t)).collect()
    }
}
