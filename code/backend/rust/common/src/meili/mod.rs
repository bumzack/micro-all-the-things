use log::{error, info};
use reqwest::{Error, Response, StatusCode};

use crate::entity::entity::Engine;

pub mod meili_entity;
pub mod meili_http;
pub mod meili_models;
