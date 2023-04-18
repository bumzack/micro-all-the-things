use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

use crate::entity::entity::{EntityConverter, get_nullable_string, get_nullable_u32};
use crate::tsv::tsv::{TsvLine, TsvLines};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub id: String,
    pub tconst: String,
    pub parent_tconst: String,
    pub season_number: Option<u32>,
    pub episode_number: Option<u32>,
}

fn map_to_episode(tsv_line: &TsvLine) -> Episode {
    let tconst = get_nullable_string(&tsv_line.entries, 0).unwrap();
    let parent_tconst = get_nullable_string(&tsv_line.entries, 1).unwrap();
    let season_number = get_nullable_u32(&tsv_line.entries, 2);
    let episode_number = get_nullable_u32(&tsv_line.entries, 3);
    let id = format!("{}_{}", tconst, parent_tconst);

    Episode {
        id,
        tconst,
        parent_tconst,
        season_number,
        episode_number,
    }
}

impl EntityConverter<Episode> for TsvLines {
    fn convert(&self) -> Vec<Episode> {
        self.lines.iter().map(|t| map_to_episode(&t)).collect()
    }
}
