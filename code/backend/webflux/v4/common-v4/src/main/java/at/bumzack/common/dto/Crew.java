package at.bumzack.common.dto;

import at.bumzack.common.search.SearchResult;
import at.bumzack.common.solr.SolrResponse;
import org.apache.commons.lang3.StringUtils;
import org.springframework.core.ParameterizedTypeReference;

import java.io.Serializable;
import java.util.List;

@SolrDocument
public class Crew extends AbstractItem implements Serializable {

    public static final ParameterizedTypeReference<SolrResponse<Crew>> TYPE_REF_CREW = new ParameterizedTypeReference<>() {
    };

    public static final ParameterizedTypeReference<SearchResult<Crew>> TYPE_REF_SEARCH_RESULT_CREW = new ParameterizedTypeReference<>() {
    };

    private static final long serialVersionUID = 3L;

    private String id;
    private String tconst;
    private List<String> directors;
    private List<String> writers;

    public Crew() {
    }

    @Override
    public String getId() {
        return id;
    }

    @Override
    public void setId(final String id) {
        this.id = id;
    }

    public String getTconst() {
        return tconst;
    }

    public void setTconst(final String tconst) {
        this.tconst = tconst;
    }

    public List<String> getDirectors() {
        return directors;
    }

    public void setDirectors(final List<String> directors) {
        this.directors = directors;
    }

    public List<String> getWriters() {
        return writers;
    }

    public void setWriters(final List<String> writers) {
        this.writers = writers;
    }

    @Override
    public String toString() {
        final var directorsStr = StringUtils.joinWith(" // ", directors);
        final var writersStr = StringUtils.joinWith(" // ", writers);
        return "Crew{" +
                "id='" + id + '\'' +
                "tconst='" + tconst + '\'' +
                ", directors=" + directorsStr +
                ", writers=" + writersStr +
                '}';
    }
}
