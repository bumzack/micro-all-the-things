pub mod logging_service {
    use config::Config;
    use log::{error, info};
    use reqwest::{Error, Response, StatusCode};
    use crate::entity::entity::Engine;

    use crate::logging::logging::AddLogEntry;

    lazy_static::lazy_static! {
        static ref CLIENT_LOG: reqwest::Client = reqwest::Client::new();
    }

    lazy_static::lazy_static! {
        static ref CONFIG_LOG :Config = Config::builder()
            .add_source(config::File::with_name("/Users/bumzack/stoff/micro-all-the-things/code/backend/rust/config.toml"))
            .build()
            .unwrap();
    }

    lazy_static::lazy_static! {
        static ref LOG_SERVICE_URL: String ={
             let host: String = CONFIG_LOG
                .get("loggingservice_service_host")
                .expect("expected loggingservice_service_host variable");

            let port: u16 = CONFIG_LOG
                .get("loggingservice_service_port")
                .expect("expected loggingservice_service_port variable");

            let host = format!("http://{host}:{port}/api/log/entry");

             host
        };
    }

    pub async fn log_entry(service_id: String, log_type: String, message: &String) {
        let add_log_entry = AddLogEntry {
            service_id: service_id.clone(),
            log_type: log_type.clone(),
            message: message.clone(),
            logtime: chrono::offset::Utc::now(),
        };

        info!(
            "LOGGING_entry service_id {}, log_type {}, message {}",
            &service_id, &log_type, &message
        );
        let url: &String = &LOG_SERVICE_URL;
        let response = CLIENT_LOG.post(url).json(&add_log_entry).send().await;

        // dump_response_status(&response);

        match response {
            Ok(_) => {}
            Err(e) => {
                error!("error sending add log entry request {}", e);
            }
        };
    }

    pub async fn log_error(msg: String) {
        let msg = format!(
            "build_index_v3.exec_meilisearch_update. request not successful {}",
            msg,
        );
        error!("{}", msg);
        log_entry(
            "rust_create_search_index".to_string(),
            "ERROR".to_string(),
            &msg,
        )
            .await;
    }

    pub async fn log_docs_processed(num_docs: usize, offset: u32, limit: u32) {
        let message = format!(
            "sending a list of docs to the search index.  {} docs. offset {}, limit {}",
            num_docs, offset, limit
        );
        info!("{}", &message);

        log_entry(
            "rust_create_search_index".to_string(),
            "INFO".to_string(),
            &message,
        )
            .await;
    }

    pub async fn log_end(total_movies_processed: usize) -> String {
        let message = format!(
            "build_index_v3.finished. processed {} movies ",
            total_movies_processed
        );
        info!("res {}", &message);
        log_entry(
            "rust_create_search_index".to_string(),
            "INFO".to_string(),
            &message,
        )
            .await;
        message
    }

    pub async fn log_start(offset: u32, limit: u32) {
        let msg = format!(
            "build_index_v3.start. start offset {}, start limit {}",
            offset, limit
        );
        log_entry(
            "rust_create_search_index".to_string(),
            "INFO".to_string(),
            &msg,
        )
            .await;
    }

    pub async fn log_build_stats(engine: Engine, num_tasks: usize) {
        let msg = format!(
            "build_index_v3.stats. engine  {:?}.  total_tasks created {}",
            engine, num_tasks
        );
        info!("{}", msg);
        log_entry(
            "rust_create_search_index".to_string(),
            "INFO".to_string(),
            &msg,
        )
            .await;
    }

    pub async fn log_task_error(name: String, e: String) {
        let msg = format!(
            "build_index_v3.worker_error. worker {} crashed with error {}",
            name, e
        );
        error!("{}", msg);
        log_entry(
            "rust_create_search_index".to_string(),
            "ERROR".to_string(),
            &msg,
        )
            .await;
    }

    pub async fn log_task_end(name: String, id: i32, cnt_movies: i32) -> String {
        let message = format!(
            "build_index_v3()  finished task {} with {} and processed {} movies ",
            name, id, cnt_movies
        );
        info!("res {}", &message);
        log_entry(
            "rust_create_search_index".to_string(),
            "INFO".to_string(),
            &message,
        )
            .await;
        message
    }

    pub async fn log_external_service_error(
        msg1: &String,
        message: &String,
        response: &Result<Response, Error>,
    ) {
        match &response {
            Ok(res) => {
                let code = res.status();
                if code != StatusCode::OK {
                    let x = res.headers().clone();
                    // let b = res.text().await.unwrap();
                    info!("{} != OK.  returned HTTP code {} ", msg1, code);
                    info!(
                        "{} != OK.  returned HTTP code {} headers {:?}",
                        msg1, code, x
                    );

                    info!("message {}", &message);
                    log_entry(
                        "rust_create_search_index".to_string(),
                        "ERROR".to_string(),
                        message,
                    )
                        .await;
                }
            }
            Err(e) => error!("error in request to meilisearch {:?}", e),
        };
    }
}
