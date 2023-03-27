use std::fmt::{Debug, Formatter};

pub enum Entity {
    MOVIE,
    PERSON,
    CREW,
    RATING,
    EPISODE,
    PRINCIPAL,
    MOVIEAKA,
}

pub struct TsvFileImportRequest {
    pub tsv_type: Entity,
    pub start: i32,
    pub end: i32,
    pub page_size: i32,
}


pub struct TsvLine {
    pub original: String,
    pub entries: Vec<String>,
}

pub struct Rating {
    pub tconst: String,
    pub average_rating: f32,
    pub num_votes: u32,
}

pub struct Episode {
    pub tconst: String,
    pub parent_tconst: String,
    pub season_number: String,
    pub episode_number: String,

}

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

pub struct Principal {
    tconst: String,
    ordering: String,
    nconst: String,
    category: Option<String>,
    characters: Vec<String>,
}

pub struct Person {
    pub nconst: String,
    pub primary_name: String,
    pub birth_year: Option<u32>,
    pub death_year: Option<u32>,
    pub primary_profession: Vec<String>,
    pub known_for_titles: Vec<String>,
}

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