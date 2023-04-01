use std::fmt::{Debug, Formatter};

use serde::Deserialize;
use serde::Serialize;

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
    pub id: String,
    pub tconst: String,
    pub average_rating: f32,
    pub num_votes: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Episode {
    pub id: String,
    pub tconst: String,
    pub parent_tconst: String,
    pub season_number: Option<u32>,
    pub episode_number: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Movie {
    pub id: String,
    pub tconst: String,
    pub title_type: Option<String>,
    pub primary_title: Option<String>,
    pub original_title: Option<String>,
    pub is_adult: bool,
    pub start_year: Option<u32>,
    pub end_year: Option<u32>,
    pub runtime_minutes: Option<u32>,
    pub genres: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieAkas {
    pub id: String,
    pub title_id: String,
    pub ordering: u32,
    pub title: Option<String>,
    pub region: Option<String>,
    pub language: Option<String>,
    pub types: Option<Vec<String>>,
    pub attributes: Option<Vec<String>>,
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
    pub id: String,
    pub nconst: String,
    pub primary_name: Option<String>,
    pub birth_year: Option<u32>,
    pub death_year: Option<u32>,
    pub primary_profession: Option<Vec<String>>,
    pub known_for_titles: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Crew {
    pub id: String,
    pub tconst: String,
    pub directors: Option<Vec<String>>,
    pub writers: Option<Vec<String>>,
}

impl Debug for TsvLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsvLine")
            .field("original", &self.original)
            .field("entries", &self.entries.join(" // "))
            .finish()
    }
}

pub fn get_nullable_string(input: &Vec<String>, idx: usize) -> Option<String> {
    match input.get(idx) {
        Some(s) => {
            if s.eq("\\N") {
                return None;
            }
            Some(s.to_string())
        }
        None => {
            panic!("should not happen, that a field is empty")
        }
    }
}

pub fn get_nullable_bool(input: &Vec<String>, idx: usize) -> Option<bool> {
    match input.get(idx) {
        Some(s) => {
            if s.eq("\\N") {
                return None;
            }
            let b = match s.as_str() {
                "1" => true,
                _ => false,
            };
            Some(b)
        }
        None => {
            panic!("should not happen, that a field is empty")
        }
    }
}

pub fn get_nullable_u32(input: &Vec<String>, idx: usize) -> Option<u32> {
    match input.get(idx) {
        Some(s) => {
            if s.eq("\\N") {
                return None;
            }
            Some(s.parse::<u32>().unwrap())
        }
        None => {
            panic!("should not happen, that a field is empty")
        }
    }
}


pub fn get_nullable_f32(input: &Vec<String>, idx: usize) -> Option<f32> {
    match input.get(idx) {
        Some(s) => {
            if s.eq("\\N") {
                return None;
            }
            Some(s.parse::<f32>().unwrap())
        }
        None => {
            panic!("should not happen, that a field is empty")
        }
    }
}

pub fn get_nullable_string_list(input: &Vec<String>, idx: usize) -> Option<Vec<String>> {
    match input.get(idx) {
        Some(s) => {
            if s.eq("\\N") {
                return None;
            }
            let characters = s.split(",").map(|s| s.to_string()).collect();
            Some(characters)
        }
        None => {
            panic!("should not happen, that a field is empty")
        }
    }
}

pub trait EntityConvert<T> {
    fn convert(&self) -> Vec<T>;
}

pub mod handlers_entity {
    use std::convert::Infallible;

    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::{EntityConvert, TsvLines};

    pub async fn post_entity<'a, T: Serialize + Deserialize<'a> + Send>(tsv_lines: TsvLines, entity_name: String, client: &reqwest::Client) -> Result<impl warp::Reply, Infallible>
        where TsvLines: EntityConvert<T>
    {
        println!("processing request with {} lines. {}", tsv_lines.lines.len(), &entity_name);
        let entities: Vec<T> = tsv_lines.convert();

        let json = json!(&entities).to_string();

        // let client = reqwest::Client::new();
        let index = format!("http://meilisearch01.bumzack.at/indexes/{}/documents?primaryKey=id", &entity_name);
        let response = client
            .post(&index)
            .body(json)
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        match response {
            Ok(res) => {
                let code = res.status().clone();
                // let x = res.headers().clone();
                // let b = res.text().await.unwrap();
                println!("request ok. status {:?}", code);
                // println!("request ok. headers {:?}", x);
                // println!("request ok. response body {:?}", &b);
            }
            Err(e) => println!("error in request {:?}", e),
        }
        let res = "all good".to_string();
        Ok(warp::reply::json(&res))
    }
}

fn map_to_principal(tsv_line: &TsvLine) -> Principal {
    println!("mapping tsv_line {:?} to principal", &tsv_line);

    let ordering = get_nullable_u32(&tsv_line.entries, 1).expect("ordering should be there");

    let characters = get_nullable_string_list(&tsv_line.entries, 4);
    let tconst = get_nullable_string(&tsv_line.entries, 0).unwrap();
    let nconst = get_nullable_string(&tsv_line.entries, 2).unwrap();
    let id = format!("{}_{}_{}", tconst, ordering, nconst);
    let category = get_nullable_string(&tsv_line.entries, 3).unwrap();

    Principal {
        id,
        tconst,
        ordering,
        nconst,
        category,
        characters,
    }
}


fn map_to_person(tsv_line: &TsvLine) -> Person {
    println!("mapping tsv_line {:?} to person  ", &tsv_line);

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


fn map_to_episode(tsv_line: &TsvLine) -> Episode {
    println!("mapping tsv_line {:?} to Episode  ", &tsv_line);

    let tconst = get_nullable_string(&tsv_line.entries, 0).unwrap();
    let parent_tconst = get_nullable_string(&tsv_line.entries, 1).unwrap();
    let season_number = get_nullable_u32(&tsv_line.entries, 2);
    let episode_number = get_nullable_u32(&tsv_line.entries, 3);
    let id = format!("{}_{}", tconst, parent_tconst);

    Episode {
        id,
        tconst,
        parent_tconst,
        season_number,
        episode_number,
    }
}


fn map_to_movie(tsv_line: &TsvLine) -> Movie {
    println!("mapping tsv_line {:?} to Movie  ", &tsv_line);

    let tconst = get_nullable_string(&tsv_line.entries, 0).unwrap();
    let title_type = get_nullable_string(&tsv_line.entries, 1);
    let primary_title = get_nullable_string(&tsv_line.entries, 2);
    let original_title = get_nullable_string(&tsv_line.entries, 3);
    let is_adult = get_nullable_bool(&tsv_line.entries, 4).unwrap();
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
        is_adult,
        start_year,
        end_year,
        runtime_minutes,
        genres,
    }
}


fn map_to_crew(tsv_line: &TsvLine) -> Crew {
    println!("mapping tsv_line {:?} to crew  ", &tsv_line);

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


fn map_to_movieaka(tsv_line: &TsvLine) -> MovieAkas {
    println!("mapping tsv_line {:?} to MovieAkas  ", &tsv_line);

    let title_id = get_nullable_string(&tsv_line.entries, 0).unwrap();
    let ordering = get_nullable_u32(&tsv_line.entries, 1).expect("ordering should be there");
    let title = get_nullable_string(&tsv_line.entries, 2);
    let region = get_nullable_string(&tsv_line.entries, 3);
    let language = get_nullable_string(&tsv_line.entries, 4);
    let types = get_nullable_string_list(&tsv_line.entries, 5);
    let attributes = get_nullable_string_list(&tsv_line.entries, 6);
    let original_title = get_nullable_bool(&tsv_line.entries, 7).unwrap();
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


fn map_to_rating(tsv_line: &TsvLine) -> Rating {
    println!("mapping tsv_line {:?} to Rating  ", &tsv_line);

    let tconst = get_nullable_string(&tsv_line.entries, 0).unwrap();
    let average_rating = get_nullable_f32(&tsv_line.entries, 1).unwrap();
    let num_votes = get_nullable_u32(&tsv_line.entries, 2).unwrap();
    let id = tconst.clone();

    Rating {
        id,
        tconst,
        average_rating,
        num_votes,
    }
}


impl EntityConvert<Principal> for TsvLines {
    fn convert(&self) -> Vec<Principal> {
        let v: Vec<Principal> = self.lines
            .iter()
            .map(|t| map_to_principal(&t))
            .collect();
        v
    }
}


impl EntityConvert<Person> for TsvLines {
    fn convert(&self) -> Vec<Person> {
        let v: Vec<Person> = self.lines
            .iter()
            .map(|t| map_to_person(&t))
            .collect();
        v
    }
}


impl EntityConvert<Crew> for TsvLines {
    fn convert(&self) -> Vec<Crew> {
        let v: Vec<Crew> = self.lines
            .iter()
            .map(|t| map_to_crew(&t))
            .collect();
        v
    }
}


impl EntityConvert<Episode> for TsvLines {
    fn convert(&self) -> Vec<Episode> {
        let v: Vec<Episode> = self.lines
            .iter()
            .map(|t| map_to_episode(&t))
            .collect();
        v
    }
}


impl EntityConvert<Movie> for TsvLines {
    fn convert(&self) -> Vec<Movie> {
        let v: Vec<Movie> = self.lines
            .iter()
            .map(|t| map_to_movie(&t))
            .collect();
        v
    }
}


impl EntityConvert<MovieAkas> for TsvLines {
    fn convert(&self) -> Vec<MovieAkas> {
        let v: Vec<MovieAkas> = self.lines
            .iter()
            .map(|t| map_to_movieaka(&t))
            .collect();
        v
    }
}

impl EntityConvert<Rating> for TsvLines {
    fn convert(&self) -> Vec<Rating> {
        let v: Vec<Rating> = self.lines
            .iter()
            .map(|t| map_to_rating(&t))
            .collect();
        v
    }
}



