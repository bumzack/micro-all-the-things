use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

use crate::entity::entity::{
    get_nullable_string, get_nullable_string_list, get_nullable_u32, EntityConverter,
};
use crate::tsv::tsv::{TsvLine, TsvLines};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchPersonList {
    pub nconsts: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchPrincipalList {
    pub tconsts: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: String,
    pub nconst: String,
    pub primary_name: Option<String>,
    pub birth_year: Option<u32>,
    pub death_year: Option<u32>,
    pub primary_profession: Option<Vec<String>>,
    pub known_for_titles: Option<Vec<String>>,
}

fn map_to_person(tsv_line: &TsvLine) -> Person {
    // info!("mapping tsv_line {:?} to person  ", &tsv_line);

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
