package at.bumzack.foryouandyourfakewebshop.searcharticle.model;

import java.util.List;

public record IndexDocFacetDistribution(List<String> actors) {
}


//    pub struct IndexDocFacetDistribution {
//    pub actors: Option<HashMap<String, HashMap<String, usize>>>,
//    pub directors: Option<HashMap<String, HashMap<String, usize>>>,
//    pub genres: Option<HashMap<String, HashMap<String, usize>>>,
//    pub titles: Option<HashMap<String, HashMap<String, usize>>>,
//    pub characters: Option<HashMap<String, HashMap<String, usize>>>,
//}
