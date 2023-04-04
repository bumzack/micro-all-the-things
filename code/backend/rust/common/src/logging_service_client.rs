pub mod logging_service {
    use config::Config;

    use crate::logging::AddLogEntry;

    lazy_static::lazy_static! {
        static ref CLIENT_LOG: reqwest::Client = reqwest::Client::new();
    }

    lazy_static::lazy_static! {
        static ref CONFIG_LOG :Config = Config::builder()
            .add_source(config::File::with_name("/home/bumzack/micro-all-the-things/code/backend/rust/config.toml"))
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
}
