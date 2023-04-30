package at.bumzack.foryouandyourfakewebshop.searcharticle.model;

import java.util.List;

public class IndexDocFacetDistribution {
    private List<String> actors;

    public IndexDocFacetDistribution() {
    }

    @Override
    public String toString() {
        return "IndexDocFacetDistribution{" +
                "actors=" + actors +
                '}';
    }

    public List<String> getActors() {
        return actors;
    }

    public void setActors(final List<String> actors) {
        this.actors = actors;
    }
}


//    pub struct IndexDocFacetDistribution {
//    pub actors: Option<HashMap<String, HashMap<String, usize>>>,
//    pub directors: Option<HashMap<String, HashMap<String, usize>>>,
//    pub genres: Option<HashMap<String, HashMap<String, usize>>>,
//    pub titles: Option<HashMap<String, HashMap<String, usize>>>,
//    pub characters: Option<HashMap<String, HashMap<String, usize>>>,
//}
