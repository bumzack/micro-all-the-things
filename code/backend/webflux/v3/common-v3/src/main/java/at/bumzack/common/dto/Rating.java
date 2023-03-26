package at.bumzack.common.dto;

import at.bumzack.common.search.SearchResult;
import at.bumzack.common.solr.SolrResponse;
import org.apache.commons.lang3.StringUtils;
import org.springframework.core.ParameterizedTypeReference;

import java.io.Serializable;
import java.util.List;

@SolrDocument
public class Rating extends AbstractItem implements Serializable {

    public static final ParameterizedTypeReference<SolrResponse<Rating>> TYPE_REF_RATING = new ParameterizedTypeReference<>() {
    };

    public static final ParameterizedTypeReference<SearchResult<Rating>> TYPE_REF_SEARCH_RESULT_RATING = new ParameterizedTypeReference<>() {
    };

    private String tconst;
    private String averageRating;
    private String numVotes;

    public Rating() {
    }

    public String getTconst() {
        return tconst;
    }

    public void setTconst(final String tconst) {
        this.tconst = tconst;
    }

    public String getAverageRating() {
        return averageRating;
    }

    public void setAverageRating(final String averageRating) {
        this.averageRating = averageRating;
    }

    public String getNumVotes() {
        return numVotes;
    }

    public void setNumVotes(final String numVotes) {
        this.numVotes = numVotes;
    }
}
