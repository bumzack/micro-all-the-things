use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

use crate::entity::{EntityConverter, get_nullable_string, get_nullable_u32};
use crate::tsv::{TsvLine, TsvLines};

#[derive(Debug, Deserialize, Serialize)]
pub struct Episode {
    pub id: String,
    pub tconst: String,
    #[serde(rename = "parentTconst")]
    pub parent_tconst: String,
    #[serde(rename = "seasonNumber")]
    pub season_number: Option<u32>,
    #[serde(rename = "episodeNumber")]
    pub episode_number: Option<u32>,
}

fn map_to_episode(tsv_line: &TsvLine) -> Episode {
    // println!("mapping tsv_line {:?} to Episode  ", &tsv_line);

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
