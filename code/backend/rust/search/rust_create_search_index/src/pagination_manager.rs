use std::collections::{HashMap, VecDeque};

use log::{error, info};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct PaginationData {
    pub(crate) limit: u32,
    pub(crate) offset: u32,
}

#[derive(Debug)]
pub struct WorkerData {
    pub(crate) sender: UnboundedSender<PaginationData>,
    pub(crate) id: usize,
}

#[derive(Debug)]
pub enum ManagerCommand {
    //     GetPagination(GetPaginationCmdData),
    RegisterWorker(WorkerData),
    WorkerReady(usize),
    WorkerNoMoreItemsFound(usize),
}

pub fn start_config_manager(
    init_limit: u32,
    init_offset: u32,
    mut manager_receiver: UnboundedReceiver<ManagerCommand>,
) -> JoinHandle<String> {
    let mut offset = init_offset;
    let limit = init_limit;

    let mut workers = HashMap::new();
    let mut worker_queue: VecDeque<usize> = VecDeque::new();
    let mut done = false;
    tokio::spawn(async move {
        info!("MANAGER pagination manager thread started");
        while let Some(cmd) = manager_receiver.recv().await {
            match cmd {
                ManagerCommand::WorkerReady(id) => {
                    info!("MANAGER:  worker is ready again {}", id);
                    if workers.contains_key(&id) {
                        worker_queue.push_back(id);
                    } else {
                        error!("MANAGER: unknown worker id {}", id);
                    }
                }
                ManagerCommand::WorkerNoMoreItemsFound(id) => {
                    info!("MANAGER: worker has no more items found {}", id);
                    workers.remove(&id);
                    done = true;
                }
                ManagerCommand::RegisterWorker(wd) => {
                    info!("MANAGER: got a new worker {}", wd.id);
                    let id = wd.id;
                    workers.insert(wd.id, wd);
                    worker_queue.push_back(id);
                }
            }

            while !worker_queue.is_empty() {
                match worker_queue.pop_front() {
                    Some(id) => {
                        if workers.get(&id).is_some() {
                            info!("MANAGER: ok found an active worker {}", id);
                            let wd = workers.get(&id).unwrap();
                            let new_pg_data = PaginationData { limit, offset };
                            offset += limit;
                            match wd.sender.send(new_pg_data) {
                                Ok(_) => {
                                    info!("MANAGER: ok sending data to worker id {}", id);
                                }
                                Err(e) => {
                                    error!(
                                        "MANAGER: error sending data to worker id {}. error {}",
                                        id, e
                                    );
                                }
                            }
                        }
                    }
                    None => {
                        error!("MANAGER: no worker available")
                    }
                }
            }
            if done {
                info!("we are DONE here!!");
            }
        }
        let msg = "pagination manager is done. no errors.".to_string();
        info!("{}", &msg);
        msg
    })
}
