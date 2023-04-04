use std::convert::Infallible;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;

use common::logging_service_client::logging_service;

use crate::pagination_manager::ManagerCommand::{WorkerNoMoreItemsFound, WorkerReady};
use crate::pagination_manager::{start_config_manager, ManagerCommand, WorkerData};

pub async fn build_index_v2(multiplier: u32) -> Result<impl warp::Reply, Infallible> {
    let offset = 0;
    let limit = 100;

    let (manager_sender, manager_receiver) = mpsc::unbounded_channel();

    let handle_config_manager = start_config_manager(limit, offset, manager_receiver);
    let num_cpus = num_cpus::get();
    let num_tasks = num_cpus * multiplier as usize;

    log_build_stats(num_cpus, multiplier, num_tasks).await;

    let tasks = start_tasks(num_tasks, manager_sender);

    log_start(offset, limit).await;

    // while cnt_movies < total_cnt_movies {
    //     let movies = search_movies(limit, offset).await;
    //     offset += limit;
    //
    //     let mut docs = vec![];
    //     convert_to_meilisearch_doc(total_cnt_movies, &mut cnt_movies, movies, &mut docs).await;
    //
    //     let docs_json = json!(&docs).to_string();
    //
    //     let message = format!(
    //         "sending a list of docs to the search index.  {} docs. movies processed {} / {}",
    //         docs.len(),
    //         cnt_movies,
    //         total_cnt_movies
    //     );
    //     println!("{}", &message);
    //
    //     logging_service::log_entry(
    //         "rust_create_search_index".to_string(),
    //         "INFO".to_string(),
    //         &message,
    //     )
    //     .await;
    //
    //     println!("starting update request for  {} docs", docs.len());
    //     exec_meilisearch_update(&"searchindex".to_string(), &CLIENT, docs_json).await;
    //     println!(
    //         "finished update request for  {} docs.  . movies processed {} / {} ",
    //         docs.len(),
    //         cnt_movies,
    //         total_cnt_movies
    //     );
    // }

    let mut total_movies_processed = 0;
    for t in tasks {
        match t.await {
            Ok((id, cnt)) => {
                total_movies_processed += cnt;
                info!(
                    "worker {} ended successfully and processed {} items",
                    id, cnt
                );
                log_task_end("worker".to_string(), id as i32, cnt as i32).await;
            }
            Err(e) => {
                error!("worker  ended with an error  {:?}", e);
                log_task_error("worker".to_string(), e.to_string()).await;
            }
        }
    }

    match handle_config_manager.await {
        Ok(msg) => {
            info!("pagination manager says.  '{}'", msg);
            log_task_end("pagination manager".to_string(), -1, -1).await;
        }
        Err(e) => {
            error!("pagination manager   ended with an error  {:?}", e);
            log_task_error("pagination manager".to_string(), e.to_string()).await;
        }
    }

    let message = log_end(total_movies_processed).await;
    info!("done {}", &message);
    Ok(warp::reply::json(&message))
}

fn start_tasks(
    tasks: usize,
    manager_sender: UnboundedSender<ManagerCommand>,
) -> Vec<JoinHandle<(usize, usize)>> {
    let mut t = vec![];
    for i in 1..=tasks {
        let sender = manager_sender.clone();
        let id = i;
        let t1 = tokio::spawn(async move {
            let (tx, mut rx) = mpsc::unbounded_channel();
            let wd = WorkerData { sender: tx, id: id };
            let mc = ManagerCommand::RegisterWorker(wd);

            let mut cnt = 0;
            sender
                .send(mc)
                .expect("    TASK: registering a worker should work");

            while let Some(pg) = rx.recv().await {
                info!(
                    "  TASK: worker {} got new pagination data {:?}.  limit {}, offset {}",
                    id, pg, pg.limit, pg.offset
                );

                // do stuff
                tokio::time::sleep(Duration::from_millis(100 * id as u64)).await;

                cnt += 1;
                let wd = WorkerReady(id);
                sender
                    .send(wd)
                    .expect("    TASK: send manager a 'ready' message");
                cnt += 1;

                // random abort criteria
                if cnt > id {
                    let wd = WorkerNoMoreItemsFound(id);
                    sender
                        .send(wd)
                        .expect("    TASK: send manager a 'finished' message");
                }
            }
            info!("worker {} closing business", id);
            // ¯\_(ツ)_/¯
            (id, cnt)
        });
        t.push(t1);
    }
    t
}

async fn log_end(total_movies_processed: usize) -> String {
    let message = format!(
        "build_index_v2.finished. processed {} movies ",
        total_movies_processed
    );
    info!("res {}", &message);
    logging_service::log_entry(
        "rust_create_search_index".to_string(),
        "INFO".to_string(),
        &message,
    )
    .await;
    message
}

async fn log_start(offset: u32, limit: u32) {
    let msg = format!(
        "build_index_v2.start. start offset {}, start limit {}",
        offset, limit
    );
    logging_service::log_entry(
        "rust_create_search_index".to_string(),
        "INFO".to_string(),
        &msg,
    )
    .await;
}

async fn log_build_stats(num_cpus: usize, multiplier: u32, num_tasks: usize) {
    let msg = format!(
        "build_index_v2.stats. cpu has {} cores. multiplier {}. total_tasks created {}",
        num_cpus, multiplier, num_tasks
    );
    info!("{}", msg);
    logging_service::log_entry(
        "rust_create_search_index".to_string(),
        "INFO".to_string(),
        &msg,
    )
    .await;
}

async fn log_task_error(name: String, e: String) {
    let msg = format!(
        "build_index_v2.worker_error. worker {} crashed with error {}",
        name, e
    );
    error!("{}", msg);
    logging_service::log_entry(
        "rust_create_search_index".to_string(),
        "ERROR".to_string(),
        &msg,
    )
    .await;
}

async fn log_task_end(name: String, id: i32, cnt_movies: i32) -> String {
    let message = format!(
        "build_index_v2()  finished task {} with {} and processed {} movies ",
        name, id, cnt_movies
    );
    info!("res {}", &message);
    logging_service::log_entry(
        "rust_create_search_index".to_string(),
        "INFO".to_string(),
        &message,
    )
    .await;
    message
}
