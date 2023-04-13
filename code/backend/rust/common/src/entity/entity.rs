use std::fmt::Debug;

use log::info;
use serde::Deserialize;
use serde::Serialize;

use crate::N_A;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Entity {
    MOVIE,
    PERSON,
    CREW,
    RATING,
    EPISODE,
    PRINCIPAL,
    MOVIEAKA,
    SEARCHINDEX,
}

pub fn get_nullable_string(input: &Vec<String>, idx: usize) -> Option<String> {
    match input.get(idx) {
        Some(s) => {
            if s.eq(N_A) {
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
            if s.eq(N_A) {
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
            if s.eq(N_A) {
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
            if s.eq(N_A) {
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
            if s.eq(N_A) {
                return None;
            }
            let characters = s
                .split(',')
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty())
                .collect();
            Some(characters)
        }
        None => {
            panic!("should not happen, that a field is empty")
        }
    }
}

pub fn get_nullable_string_list_of_string_array(
    input: &Vec<String>,
    idx: usize,
) -> Option<Vec<String>> {
    match input.get(idx) {
        Some(s) => {
            if s.eq(N_A) {
                return None;
            }
            let result = serde_json::from_str::<Vec<String>>(s);
            if result.is_err() {
                info!(
                    "serializing the line did not work:  '{}'     input:   '{:?}'    ",
                    &s, &input
                );
            }
            let res = result.unwrap();
            Some(res)
        }
        None => {
            panic!("should not happen, that a field is empty")
        }
    }
}

pub trait EntityConverter<T> {
    fn convert(&self) -> Vec<T>;
}

pub mod handlers_entity {
    use std::convert::Infallible;

    use log::info;
    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::entity::entity::{Entity, EntityConverter};
    use crate::meili::meili_http::meili_http_stuff::meili_update_http;
    use crate::solr::solr_http::mod_solr_http::solr_update_http;
    use crate::tsv::tsv::TsvLines;

    pub async fn post_entity<'a, T: Serialize + Deserialize<'a> + Send>(
        tsv_lines: TsvLines,
        entity: Entity,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible>
    where
        TsvLines: EntityConverter<T>,
    {
        info!(
            "processing request with {} lines. {}",
            tsv_lines.lines.len(),
            &entity.to_string()
        );
        let entities: Vec<T> = tsv_lines.convert();

        let json = json!(&entities).to_string();

        meili_update_http(&entity, client, json.clone()).await;
        solr_update_http(&entity, client, json).await;

        let res = "all good".to_string();
        Ok(warp::reply::json(&res))
    }
}

impl ToString for Entity {
    fn to_string(&self) -> String {
        match self {
            Entity::MOVIE => "movie".to_string(),
            Entity::PERSON => "person".to_string(),
            Entity::CREW => "crew".to_string(),
            Entity::RATING => "rating".to_string(),
            Entity::EPISODE => "episode".to_string(),
            Entity::PRINCIPAL => "principal".to_string(),
            Entity::MOVIEAKA => "movieaka".to_string(),
            Entity::SEARCHINDEX => "searchindex".to_string(),
        }
    }
}
