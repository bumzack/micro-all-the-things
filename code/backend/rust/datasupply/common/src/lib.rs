use serde::Deserialize;
use serde::Serialize;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Deserialize, Serialize)]
pub enum Entity {
    MOVIE,
    PERSON,
    CREW,
    RATING,
    EPISODE,
    PRINCIPAL,
    MOVIEAKA,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TsvFileImportRequest {
    #[serde(rename = "tsvType")]
    pub tsv_type: Entity,
    pub start: i32,
    pub end: i32,
    #[serde(rename = "pageSize")]
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


#[derive(Debug, Deserialize, Serialize)]
pub struct Rating {
    pub tconst: String,
    pub average_rating: f32,
    pub num_votes: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Episode {
    pub tconst: String,
    pub parent_tconst: String,
    pub season_number: Option<u32>,
    pub episode_number: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Movie {
    pub tconst: String,
    pub title_type: String,
    pub primary_title: String,
    pub original_title: String,
    pub is_adult: bool,
    pub start_year: Option<u32>,
    pub end_year: Option<u32>,
    pub runtime_minutes: Option<u32>,
    pub genres: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieAkas {
    pub title_id: String,
    pub ordering: u32,
    pub title: String,
    pub region: String,
    pub language: String,
    pub types: Vec<String>,
    pub attributes: Vec<String>,
    pub original_title: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Principal {
    pub id: String,
    pub tconst: String,
    pub ordering: u32,
    pub nconst: String,
    pub category: String,
    pub characters: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    pub nconst: String,
    pub primary_name: String,
    pub birth_year: Option<u32>,
    pub death_year: Option<u32>,
    pub primary_profession: Vec<String>,
    pub known_for_titles: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Crew {
    pub id: String,
    pub tconst: String,
    pub directors: Vec<String>,
    pub writers: Vec<String>,
}

impl Debug for TsvLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsvLine")
            .field("original", &self.original)
            .field("entries", &self.entries.join(" // "))
            .finish()
    }
}
