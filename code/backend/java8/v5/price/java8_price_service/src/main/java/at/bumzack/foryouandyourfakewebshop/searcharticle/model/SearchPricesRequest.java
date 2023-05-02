package at.bumzack.foryouandyourfakewebshop.searcharticle.model;

import com.fasterxml.jackson.annotation.JsonProperty;

import java.util.List;


public class SearchPricesRequest {

    @JsonProperty("movieTconst")
    private List<String> movieTconst;

    public SearchPricesRequest() {
    }

    public List<String> getMovieTconst() {
        return movieTconst;
    }

    public void setMovieTconst(final List<String> movieTconst) {
        this.movieTconst = movieTconst;
    }

    @Override
    public String toString() {
        return "SearchPricesRequest{" +
                "movieTconst=" + movieTconst +
                '}';
    }
}


