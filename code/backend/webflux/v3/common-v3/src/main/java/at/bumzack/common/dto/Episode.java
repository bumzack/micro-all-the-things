package at.bumzack.common.dto;

import at.bumzack.common.search.SearchResult;
import at.bumzack.common.solr.SolrResponse;
import org.springframework.core.ParameterizedTypeReference;

import java.io.Serializable;

@SolrDocument
public class Episode extends AbstractItem implements Serializable {

    public static final ParameterizedTypeReference<SolrResponse<Episode>> TYPE_REF_EPISODE = new ParameterizedTypeReference<>() {
    };

    public static final ParameterizedTypeReference<SearchResult<Episode>> TYPE_REF_SEARCH_RESULT_EPISODE = new ParameterizedTypeReference<>() {
    };

    private String tconst;
    private String parentTconst;
    private String seasonNumber;
    private String episodeNumber;

    public Episode() {
    }


    public String getTconst() {
        return tconst;
    }

    public void setTconst(final String tconst) {
        this.tconst = tconst;
    }

    public String getParentTconst() {
        return parentTconst;
    }

    public void setParentTconst(final String parentTconst) {
        this.parentTconst = parentTconst;
    }

    public String getSeasonNumber() {
        return seasonNumber;
    }

    public void setSeasonNumber(final String seasonNumber) {
        this.seasonNumber = seasonNumber;
    }

    public String getEpisodeNumber() {
        return episodeNumber;
    }

    public void setEpisodeNumber(final String episodeNumber) {
        this.episodeNumber = episodeNumber;
    }

    @Override
    public String toString() {
        return "Episode{" +
                "id='" + getId() + '\'' +
                ", tconst='" + tconst + '\'' +
                ", parentTconst='" + parentTconst + '\'' +
                ", seasonNumber='" + seasonNumber + '\'' +
                ", episodeNumber='" + episodeNumber + '\'' +
                '}';
    }
}