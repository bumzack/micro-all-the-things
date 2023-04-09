pub mod mod_solr {
    use std::collections::HashMap;

    use log::info;
    use reqwest::{Client, Error, Response, StatusCode, Url};

    use crate::entity::entity::Entity;
    use crate::logging::logging_service_client::logging_service::log_error;
    use crate::meili::dump_response_status;

    pub(crate) async fn solr_search(
        entity: Entity,
        filters: Option<HashMap<String, Vec<String>>>,
        facets: Option<Vec<String>>,
        search_text: Option<Vec<(String, String)>>,
        sort: Option<Vec<(String, bool)>>,
        limit: Option<u32>,
        offset: Option<u32>,
        client: &Client,
    ) -> Result<Response, Error> {
        let mut url_params: Vec<(String, String)> = vec![];

        if filters.is_some() {
            let f = filters.unwrap();
            f.iter().for_each(|f| {
                let field_name = f.0;
                let field_values = f.1;
                for filter_value in field_values {
                    let entry = format!("{field_name}:{filter_value}");
                    let entry = ("fq".to_string(), entry);
                    url_params.push(entry)
                }
            })
        };

        if search_text.is_some() {
            let search_text = search_text.unwrap();
            search_text.iter().for_each(|(field, txt)| {
                let entry = format!("{field}:{txt}");
                let entry = ("q".to_string(), entry);
                url_params.push(entry)
            })
        } else {
            url_params.push(("q".to_string(), "*:*".to_string()));
        };

        if limit.is_some() {
            let entry = ("rows".to_string(), limit.unwrap().to_string());
            url_params.push(entry)
        }

        if offset.is_some() {
            let entry = ("start".to_string(), offset.unwrap().to_string());
            url_params.push(entry)
        }

        if facets.is_some() {
            let entry = ("facet".to_string(), "true".to_string());
            url_params.push(entry);
            let facets = facets.unwrap();
            facets.iter().for_each(|f| {
                let entry = ("facet.field".to_string(), f.clone());
                url_params.push(entry);
            });
        }
        // http://solr01.bumzack.at/solr/movie/select?facet=true&indent=true&q.op=OR&q=*%3A*&facet.field=originalTitle&facet.field=primaryTitle

        if sort.is_some() {
            let sort = sort.unwrap();
            let sort: Vec<String> = sort
                .iter()
                .map(|(field, asc)| {
                    let order = match asc {
                        true => "asc",
                        false => "desc",
                    };
                    format!("{field} {order}")
                })
                .collect();
            let sort = sort.join(",");

            let entry = ("sort".to_string(), sort);
            url_params.push(entry)
        };

        let url_params = url_params.iter();

        let index = format!(
            "http://solr01.bumzack.at/solr/{}/select",
            entity.to_string()
        );
        let url = Url::parse_with_params(&index, url_params)
            .expect("should be a valid url")
            .to_string();

        info!(
            "solr_search  entity {:?},  url {}",
            entity.to_string(),
            &url
        );

        let response = client
            .get(&url)
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        dump_response_status(&response, &index, &"n/a".to_string(), "solr".to_string());

        response
    }

    pub async fn exec_solr_update(entity: &Entity, client: &Client, json: String) {
        let cmd = "/update?commitWithin=1000&overwrite=true&wt=json".to_string();
        let index = format!(
            "http://solr01.bumzack.at/solr/{}/{}",
            entity.to_string(),
            &cmd
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
                if code == StatusCode::OK
                    || code == StatusCode::CREATED
                    || code == StatusCode::ACCEPTED
                {
                    info!("solr request success");
                } else {
                    let x = res.headers().clone();
                    let b = res.text().await.unwrap();
                    info!(
                        "solr request != OK AND != CREATED  != ACCEPTED. status {:?}",
                        code
                    );
                    info!(
                        "solr request != OK AND != CREATED  != ACCEPTED. headers {:?}",
                        x
                    );
                    info!(
                        "solr request != OK AND != CREATED  != ACCEPTED. response body {:?}",
                        &b
                    );
                    let msg = format!(
                        "exec_solr_update request != OK AND != CREATED AND != ACCEPTED. entity {}, url '{}'  body: '{:?}'",
                        &entity.to_string(),
                        &index,
                        &b
                    );
                    log_error(msg).await;
                }
            }
            Err(e) => {
                info!("solr request error in request to solr {:?}", e);
                let msg = format!(
                    "exec_solr_update returned an error. inserting entity {}. error: {}",
                    &entity.to_string(),
                    e
                );
                log_error(msg).await;
            }
        }
    }
}
