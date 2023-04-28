use std::convert::Infallible;

use log::{error, info};
use serde_json::json;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;
use warp::http::HeaderMap;

use common::entity::entity::{Engine, Entity};
use common::meili::meili_http::meili_http_stuff::meili_update_http;
use common::solr::solr_http::mod_solr_http::solr_update_http;

use crate::build_search_common::{convert_to_search_index_doc, search_movies};
use crate::pagination_manager::ManagerCommand::{WorkerNoMoreItemsFound, WorkerReady};
use crate::pagination_manager::{start_config_manager, ManagerCommand, WorkerData};
use crate::CLIENT;

pub async fn build_index_v3(
    engine: Engine,
    offset: u32,
    limit: u32,
    tasks: u32,
    _headers: HeaderMap,
) -> Result<impl warp::Reply, Infallible> {
    let (manager_sender, manager_receiver) = mpsc::unbounded_channel();

    let handle_config_manager = start_config_manager(limit, offset, manager_receiver);
    let num_tasks = tasks as usize;

    let tasks = start_tasks(num_tasks, manager_sender, engine.clone());

    let mut total_movies_processed = 0;
    for t in tasks {
        match t.await {
            Ok((id, cnt)) => {
                total_movies_processed += cnt;
                info!(
                    "worker {} ended successfully and processed {} items",
                    id, cnt
                );
            }
            Err(e) => {
                error!("worker  ended with an error  {:?}", e);
            }
        }
    }

    match handle_config_manager.await {
        Ok(msg) => {
            info!("pagination manager says.  '{}'", msg);
        }
        Err(e) => {
            error!("pagination manager   ended with an error  {:?}", e);
        }
    }

    let message = format!("movies proceesses {}", total_movies_processed);
    info!("done {}", &message);
    Ok(warp::reply::json(&message))
}

async fn search_and_write_to_index(offset: u32, limit: u32, engine: Engine) -> usize {
    let movies = search_movies(limit, offset, engine.clone()).await;

    if movies.is_empty() {
        return 0;
    }
    let cnt = movies.len();
    let mut docs = vec![];
    convert_to_search_index_doc(movies, &mut docs, engine.clone()).await;

    let docs_json = json!(&docs).to_string();

    info!(
        "starting update request for  {} docs. offset {}, limit {}",
        docs.len(),
        offset,
        limit
    );
    let entity = Entity::SEARCHINDEX;
    meili_update_http(&entity, &CLIENT, docs_json.clone()).await;
    solr_update_http(&entity, &CLIENT, docs_json).await;
    info!(
        "finished update request for  {} docs.  offset {}, limit {}",
        docs.len(),
        offset,
        limit
    );

    cnt
}

fn start_tasks(
    tasks: usize,
    manager_sender: UnboundedSender<ManagerCommand>,
    engine: Engine,
) -> Vec<JoinHandle<(usize, usize)>> {
    let mut t = vec![];
    for i in 1..=tasks {
        let sender = manager_sender.clone();
        let id = i;
        let engine = engine.to_owned();
        let t1 = tokio::spawn(async move {
            let (tx, mut rx) = mpsc::unbounded_channel();
            let wd = WorkerData { sender: tx, id };
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
                let limit = pg.limit;
                let offset = pg.offset;

                // do stuff
                let cnt_movies = search_and_write_to_index(offset, limit, engine.clone()).await;
                cnt += cnt_movies;
                if cnt_movies == 0 {
                    let wd = WorkerNoMoreItemsFound(id);
                    sender
                        .send(wd)
                        .expect("    TASK: send manager a 'finished' message");
                } else {
                    let wd = WorkerReady(id);
                    sender
                        .send(wd)
                        .expect("    TASK: send manager a 'ready' message");
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
