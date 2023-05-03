package at.bumzack.foryouandyourfakewebshop.searchindexindex.model;

import java.util.List;

public class MovieSearchResult {
    private List<SearchDoc> movies;

    public MovieSearchResult() {
    }

    public List<SearchDoc> getMovies() {
        return movies;
    }

    public void setMovies(final List<SearchDoc> movies) {
        this.movies = movies;
    }

    @Override
    public String toString() {
        return "MovieSearchResult{" +
                "movies=" + movies +
                '}';
    }
}

//     pub facets: Option<IndexDocFacetDistribution>,


