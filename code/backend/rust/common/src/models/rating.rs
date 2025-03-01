use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

use crate::entity::entity::{
    EntityConverter, get_nullable_f32, get_nullable_string, get_nullable_u32,
};
use crate::tsv::tsv::{TsvLine, TsvLines};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    pub id: String,
    pub tconst: String,
    pub average_rating: f32,
    pub num_votes: u32,
}

fn map_to_rating(tsv_line: &TsvLine) -> Rating {
    // info!("mapping tsv_line {:?} to Rating  ", &tsv_line);

    let tconst = get_nullable_string(&tsv_line.entries, 0).unwrap();
    let average_rating = get_nullable_f32(&tsv_line.entries, 1).unwrap();
    let num_votes = get_nullable_u32(&tsv_line.entries, 2).unwrap();
    let id = tconst.clone();

    Rating {
        id,
        tconst,
        average_rating,
        num_votes,
    }
}

impl EntityConverter<Rating> for TsvLines {
    fn convert(&self) -> Vec<Rating> {
        self.lines.iter().map(|t| map_to_rating(&t)).collect()
    }
}
