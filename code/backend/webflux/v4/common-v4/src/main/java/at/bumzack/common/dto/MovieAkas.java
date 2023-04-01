package at.bumzack.common.dto;

import at.bumzack.common.search.SearchResult;
import at.bumzack.common.solr.SolrResponse;
import org.apache.commons.lang3.StringUtils;
import org.springframework.core.ParameterizedTypeReference;

import java.io.Serializable;
import java.util.List;

@SolrDocument
public class MovieAkas extends AbstractItem implements Serializable {

    public static final ParameterizedTypeReference<SolrResponse<MovieAkas>> TYPE_REF_MOVIEAKAS = new ParameterizedTypeReference<>() {
    };

    public static final ParameterizedTypeReference<SearchResult<MovieAkas>> TYPE_REF_SEARCH_RESULT_MOVIEAKAS = new ParameterizedTypeReference<>() {
    };

    private static final long serialVersionUID = 3L;

    private String titleId;
    private Integer ordering;
    private String title;
    private String region;
    private String language;
    private List<String> types;
    private List<String> attributes;
    private boolean originalTitle;

    public MovieAkas() {
    }


    public String getTitleId() {
        return titleId;
    }

    public void setTitleId(final String titleId) {
        this.titleId = titleId;
    }

    public Integer getOrdering() {
        return ordering;
    }

    public void setOrdering(final Integer ordering) {
        this.ordering = ordering;
    }

    public String getTitle() {
        return title;
    }

    public void setTitle(final String title) {
        this.title = title;
    }

    public String getRegion() {
        return region;
    }

    public void setRegion(final String region) {
        this.region = region;
    }

    public String getLanguage() {
        return language;
    }

    public void setLanguage(final String language) {
        this.language = language;
    }

    public List<String> getTypes() {
        return types;
    }

    public void setTypes(final List<String> types) {
        this.types = types;
    }

    public List<String> getAttributes() {
        return attributes;
    }

    public void setAttributes(final List<String> attributes) {
        this.attributes = attributes;
    }

    public boolean isOriginalTitle() {
        return originalTitle;
    }

    public void setOriginalTitle(final boolean originalTitle) {
        this.originalTitle = originalTitle;
    }

    @Override
    public String toString() {
        final var typesStr = StringUtils.joinWith(" // ", types);
        final var attributesStr = StringUtils.joinWith(" // ", attributes);
        return "MovieAkas{" +
                "id='" + getId() + '\'' +
                "titleId='" + titleId + '\'' +
                ", ordering=" + ordering +
                ", title='" + title + '\'' +
                ", region='" + region + '\'' +
                ", language='" + language + '\'' +
                ", types=" + typesStr +
                ", attributes=" + attributesStr +
                ", isOriginalTitle=" + originalTitle +
                '}';
    }
}
