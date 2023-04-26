pub mod meili_http_stuff {
    use std::collections::HashMap;

    use log::info;
    use reqwest::{Client, Error, Response, StatusCode};
    use serde_json::json;

    use crate::entity::entity::{Engine, Entity};
    use crate::helper::dump_response_status;
    use crate::logging::logging_service_client::logging_service::log_error;
    use crate::meili::meili_models::MeiliSearchRequest;

    pub(crate) async fn meili_search_http(
        entity: Entity,
        filters: Option<HashMap<String, Vec<String>>>,
        facets: Option<Vec<String>>,
        search_text: Option<Vec<(String, String)>>,
        sort: Option<Vec<(String, bool)>>,
        limit: Option<u32>,
        offset: Option<u32>,
        client: &Client,
    ) -> Result<Response, Error> {
        let filter = match filters {
            Some(f) => {
                let mut filter = vec![];
                f.iter().for_each(|f| {
                    let field_name = f.0;
                    let field_values = f.1;
                    for filter_value in field_values {
                        let entry = format!("\"{field_name}\" = \"{filter_value}\"");
                        filter.push(entry)
                    }
                });
                Some(filter)
            }
            None => None,
        };

        let mut search = vec![];
        if search_text.is_some() {
            let search_text = search_text.unwrap();
            search_text
                .iter()
                .for_each(|(_, txt)| search.push(txt.clone()))
        } else {
            search.push("*".to_string());
        };
        let search_text = search.join(" OR ");

        let sort = match sort {
            Some(s) => {
                let sort: Vec<String> = s
                    .iter()
                    .map(|(field, asc)| {
                        let order = match asc {
                            true => "asc",
                            false => "desc",
                        };
                        format!("{field}:{order}")
                    })
                    .collect();
                Some(sort)
            }
            None => None,
        };

        let req = MeiliSearchRequest {
            q: search_text,
            offset,
            limit,
            hits_per_page: None,
            page: None,
            filter,
            facets,
            attributes_to_retrieve: None,
            attributes_to_crop: None,
            crop_length: None,
            crop_marker: None,
            attributes_to_highlight: None,
            highlight_pre_tag: None,
            highlight_post_tag: None,
            show_matches_position: None,
            sort,
            matching_strategy: None,
        };

        let json = json!(req).to_string();
        let url = format!(
            "http://meilisearch01.bumzack.at/indexes/{}/search",
            &entity.to_string()
        );

        info!(
            "meili_search  entity {:?},  url {}",
            entity.to_string(),
            &url
        );

        info!("meili_search  request struct {:?}", &req);

        let response = client
            .post(&url)
            .body(json.clone())
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        dump_response_status(&response, &url, &json, Engine::Meili);

        response
    }

    pub async fn meili_read_document_http(
        entity: Entity,
        offset: u32,
        limit: u32,
        client: &Client,
    ) -> Result<Response, Error> {
        info!(
            "reading documents for entity {}.  limit {}, offset {}",
            &entity.to_string(),
            limit,
            offset,
        );

        let index = format!(
            "http://meilisearch01.bumzack.at/indexes/{}/documents?limit={}&offset={}",
            &entity.to_string(),
            limit,
            offset
        );

        info!(
            "reading documents for entity {}. url: {}",
            &entity.to_string(),
            &index
        );
        let response = client
            .get(&index)
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        dump_response_status(
            &response,
            &index,
            &"none available".to_string(),
            Engine::Meili,
        );

        response
    }

    pub async fn meili_update_http(entity: &Entity, client: &Client, json: String) {
        let index = format!(
            "http://meilisearch01.bumzack.at/indexes/{}/documents?primaryKey=id",
            &entity.to_string()
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
                if code == StatusCode::OK
                    || code == StatusCode::CREATED
                    || code == StatusCode::ACCEPTED
                {
                    info!("meilisearch request success");
                } else {
                    let x = res.headers().clone();
                    let b = res.text().await.unwrap();
                    info!(
                        "meilisearch request != OK AND != CREATED AND != ACCEPTED. status {:?}",
                        code
                    );
                    info!(
                        "meilisearch request != OK AND != CREATED AND != ACCEPTED. headers {:?}",
                        x
                    );
                    info!("meilisearch request != OK AND != CREATED AND != ACCEPTED. response body {:?}", &b);

                    let msg = format!(
                        "exec_meilisearch_update request != OK AND != CREATED AND != ACCEPTED. entity {}, url '{}'  body: '{:?}'",
                        &entity.to_string(),
                        &index,
                        &b
                    );
                    log_error(msg).await;
                }
            }
            Err(e) => {
                info!("error in request to meilisearch {:?}", e);
                let msg = format!(
                    "exec_meilisearch_update returned an error. inserting entity {}. error: {}",
                    &entity.to_string(),
                    e
                );
                log_error(msg).await;
            }
        }
    }
}
