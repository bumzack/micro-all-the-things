package at.bumzack.common.dto;

import at.bumzack.common.search.SearchResult;
import at.bumzack.common.solr.SolrResponse;
import org.springframework.core.ParameterizedTypeReference;

import java.io.Serializable;

@SolrDocument
public class Rating extends AbstractItem implements Serializable {

    public static final ParameterizedTypeReference<SolrResponse<Rating>> TYPE_REF_RATING = new ParameterizedTypeReference<>() {
    };

    public static final ParameterizedTypeReference<SearchResult<Rating>> TYPE_REF_SEARCH_RESULT_RATING = new ParameterizedTypeReference<>() {
    };

    private String tconst;
    private Double averageRating;
    private Integer numVotes;

    public Rating() {
    }

    public String getTconst() {
        return tconst;
    }

    public void setTconst(final String tconst) {
        this.tconst = tconst;
    }

    public Double getAverageRating() {
        return averageRating;
    }

    public void setAverageRating(final Double averageRating) {
        this.averageRating = averageRating;
    }

    public Integer getNumVotes() {
        return numVotes;
    }

    public void setNumVotes(final Integer numVotes) {
        this.numVotes = numVotes;
    }
}
