use std::fmt::Debug;

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

pub fn get_nullable_string_list_of_string_array(input: &Vec<String>, idx: usize) -> Option<Vec<String>> {
    match input.get(idx) {
        Some(s) => {
            if s.eq(N_A) {
                return None;
            }
            let result = serde_json::from_str::<Vec<String>>(s);
            if result.is_err() {
                println!("serializing the line did not work:  '{}'     input:   '{:?}'    ", &s, &input);
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

    use reqwest::{Client, StatusCode};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::entity::EntityConverter;
    use crate::tsv::TsvLines;

    pub async fn post_entity<'a, T: Serialize + Deserialize<'a> + Send>(
        tsv_lines: TsvLines,
        entity_name: String,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible>
        where
            TsvLines: EntityConverter<T>,
    {
        println!(
            "processing request with {} lines. {}",
            tsv_lines.lines.len(),
            &entity_name
        );
        let entities: Vec<T> = tsv_lines.convert();

        let json = json!(&entities).to_string();

        exec_meilisearch_update(&entity_name, client, json.clone()).await;
        exec_solr_update(&entity_name, client, json).await;

        let res = "all good".to_string();
        Ok(warp::reply::json(&res))
    }

    pub async fn exec_meilisearch_update(entity_name: &String, client: &Client, json: String) {
        let index = format!(
            "http://meilisearch01.bumzack.at/indexes/{}/documents?primaryKey=id",
            &entity_name
        );
        let response = client
            .post(&index)
            .body(json)
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        match response {
            Ok(res) => {
                let code = res.status();
                if code == StatusCode::OK || code == StatusCode::CREATED || code == StatusCode::ACCEPTED {
                    println!("meilisearch request success");
                } else {
                    let x = res.headers().clone();
                    let b = res.text().await.unwrap();
                    println!("meilisearch request != OK AND != CREATED AND != ACCEPTED. status {:?}", code);
                    println!("meilisearch request != OK AND != CREATED AND != ACCEPTED. headers {:?}", x);
                    println!("meilisearch request != OK AND != CREATED AND != ACCEPTED. response body {:?}", &b);
                }
            }
            Err(e) => println!("error in request to meilisearch {:?}", e),
        }
    }

    async fn exec_solr_update(entity_name: &String, client: &Client, json: String) {
        let cmd = "/update?commitWithin=1000&overwrite=true&wt=json".to_string();
        let index = format!(
            "http://solr01.bumzack.at/solr/{}/{}",
            &entity_name, &cmd
        );

        let response = client
            .post(&index)
            .body(json)
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        match response {
            Ok(res) => {
                let code = res.status();
                if code == StatusCode::OK || code == StatusCode::CREATED || code == StatusCode::ACCEPTED {
                    println!("solr request success");
                } else {
                    let x = res.headers().clone();
                    let b = res.text().await.unwrap();
                    println!("solr request != OK AND != CREATED  != ACCEPTED. status {:?}", code);
                    println!("solr request != OK AND != CREATED  != ACCEPTED. headers {:?}", x);
                    println!("solr request != OK AND != CREATED  != ACCEPTED. response body {:?}", &b);
                }
            }
            Err(e) => println!("solr request error in request to solr {:?}", e),
        }
    }
}

