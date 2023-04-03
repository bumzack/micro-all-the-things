pub mod logging_service {

    pub async fn log_entry(service_id: String, logtype: String, msg: String) -> Result<STRING> {
        // let response = meili_filter(entity, filter, client);
        //
        // let response2 = response.await.unwrap();
        // let result = response2
        //     .json::<MeiliSearchResult<Person>>()
        //     .await
        //     .expect("expected a MeiliSearchResult<Person>");
        //
        // let persons = result.hits;
        //
        // let p = serde_json::to_string_pretty(&persons).expect("expected a list of persons");
        // // println!("filter_personse returned {}", p);
        //
        // Ok(warp::reply::json(&persons))
    }
}
