package at.bumzack.foryouandyourfakewebshop.searchindexindex.model;

import at.bumzack.common.annotations.SolrDocument;
import at.bumzack.common.search.AbstractItem;
import at.bumzack.common.search.SearchResult;
import at.bumzack.common.solr.SolrResponse;
import org.springframework.core.ParameterizedTypeReference;

import java.util.List;


@SolrDocument
public class SearchDoc extends AbstractItem {

    public static final ParameterizedTypeReference<SolrResponse<SearchDoc>> TYPE_REF_SEARCH_DOC = new ParameterizedTypeReference<>() {
    };

    public static final ParameterizedTypeReference<SearchResult<SearchDoc>> TYPE_REF_SEARCH_RESULT_SEARCH_DOC = new ParameterizedTypeReference<>() {
    };

    private String tconst;
    private List<String> titles;
    private List<String> actors;
    private List<String> directors;
    private List<String> genres;
    private List<String> characters;
    private Integer runtimeMinutes;
    private Integer year;
    private Boolean adult;
    private String titleType;


    public SearchDoc() {
    }

    public String getTconst() {
        return tconst;
    }

    public void setTconst(final String tconst) {
        this.tconst = tconst;
    }

    public List<String> getTitles() {
        return titles;
    }

    public void setTitles(final List<String> titles) {
        this.titles = titles;
    }

    public List<String> getActors() {
        return actors;
    }

    public void setActors(final List<String> actors) {
        this.actors = actors;
    }

    public List<String> getDirectors() {
        return directors;
    }

    public void setDirectors(final List<String> directors) {
        this.directors = directors;
    }

    public List<String> getGenres() {
        return genres;
    }

    public void setGenres(final List<String> genres) {
        this.genres = genres;
    }

    public List<String> getCharacters() {
        return characters;
    }

    public void setCharacters(final List<String> characters) {
        this.characters = characters;
    }

    public Integer getRuntimeMinutes() {
        return runtimeMinutes;
    }

    public void setRuntimeMinutes(final Integer runtimeMinutes) {
        this.runtimeMinutes = runtimeMinutes;
    }

    public Integer getYear() {
        return year;
    }

    public void setYear(final Integer year) {
        this.year = year;
    }

    public Boolean getAdult() {
        return adult;
    }

    public void setAdult(final Boolean adult) {
        this.adult = adult;
    }

    public String getTitleType() {
        return titleType;
    }

    public void setTitleType(final String titleType) {
        this.titleType = titleType;
    }

    @Override
    public String toString() {
        return "SearchDoc{" +
                "tconst='" + tconst + '\'' +
                ", titles=" + titles +
                ", actors=" + actors +
                ", directors=" + directors +
                ", genres=" + genres +
                ", characters=" + characters +
                ", runtimeMinutes=" + runtimeMinutes +
                ", year=" + year +
                ", adult=" + adult +
                ", titleType='" + titleType + '\'' +
                '}';
    }
}
