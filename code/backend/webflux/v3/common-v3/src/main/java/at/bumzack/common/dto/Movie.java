package at.bumzack.common.dto;

import at.bumzack.common.search.SearchResult;
import at.bumzack.common.solr.SolrResponse;
import org.apache.commons.lang3.StringUtils;
import org.springframework.core.ParameterizedTypeReference;

import java.io.Serializable;
import java.util.List;

@SolrDocument
public class Movie extends AbstractItem implements Serializable {

    public static final ParameterizedTypeReference<SolrResponse<Movie>> TYPE_REF_MOVIE = new ParameterizedTypeReference<>() {
    };

    public static final ParameterizedTypeReference<SearchResult<Movie>> TYPE_REF_SEARCH_RESULT_MOVIE = new ParameterizedTypeReference<>() {
    };

    private static final long serialVersionUID = 3L;

    private String tconst;
    private String titleType;
    private String primaryTitle;
    private String originalTitle;
    private boolean isAdult;
    private String startYear;
    private String endYear;
    private String runtimeMinutes;
    private List<String> genres;

    public Movie() {
    }

    @Override
    public String toString() {
        final var genreStr = StringUtils.joinWith(" // ", genres);
        return "Movie{" +
                "id='" + getId() + '\'' +
                ", tconst='" + tconst + '\'' +
                ", titleType='" + titleType + '\'' +
                ", primaryTitle='" + primaryTitle + '\'' +
                ", originalTitle='" + originalTitle + '\'' +
                ", isAdult=" + isAdult +
                ", startYear=" + startYear +
                ", endYear=" + endYear +
                ", runtimeMinutes=" + runtimeMinutes +
                ", genres=" + genreStr +
                '}';
    }

    public String getTconst() {
        return tconst;
    }

    public void setTconst(final String tconst) {
        this.tconst = tconst;
    }

    public String getTitleType() {
        return titleType;
    }

    public void setTitleType(final String titleType) {
        this.titleType = titleType;
    }

    public String getPrimaryTitle() {
        return primaryTitle;
    }

    public void setPrimaryTitle(final String primaryTitle) {
        this.primaryTitle = primaryTitle;
    }

    public String getOriginalTitle() {
        return originalTitle;
    }

    public void setOriginalTitle(final String originalTitle) {
        this.originalTitle = originalTitle;
    }

    public boolean isAdult() {
        return isAdult;
    }

    public void setAdult(final boolean adult) {
        isAdult = adult;
    }

    public String getStartYear() {
        return startYear;
    }

    public void setStartYear(final String startYear) {
        this.startYear = startYear;
    }

    public String getEndYear() {
        return endYear;
    }

    public void setEndYear(final String endYear) {
        this.endYear = endYear;
    }

    public String getRuntimeMinutes() {
        return runtimeMinutes;
    }

    public void setRuntimeMinutes(final String runtimeMinutes) {
        this.runtimeMinutes = runtimeMinutes;
    }

    public List<String> getGenres() {
        return genres;
    }

    public void setGenres(final List<String> genres) {
        this.genres = genres;
    }

}