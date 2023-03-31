package at.bumzack.common.dto;

import at.bumzack.common.search.SearchResult;
import at.bumzack.common.solr.SolrResponse;
import org.apache.commons.lang3.StringUtils;
import org.springframework.core.ParameterizedTypeReference;

import java.io.Serializable;
import java.util.List;

@SolrDocument
public class Principal extends AbstractItem implements Serializable {

    public static final ParameterizedTypeReference<SolrResponse<Principal>> TYPE_REF_PRINCIPAL = new ParameterizedTypeReference<>() {
    };

    public static final ParameterizedTypeReference<SearchResult<Principal>> TYPE_REF_SEARCH_RESULT_PRINCIPAL = new ParameterizedTypeReference<>() {
    };

    private String tconst;
    private String ordering;
    private String nconst;
    private String category;
    private List<String> characters;

    public Principal() {
    }

    public String getTconst() {
        return tconst;
    }

    public void setTconst(final String tconst) {
        this.tconst = tconst;
    }

    public String getOrdering() {
        return ordering;
    }

    public void setOrdering(final String ordering) {
        this.ordering = ordering;
    }

    public String getNconst() {
        return nconst;
    }

    public void setNconst(final String nconst) {
        this.nconst = nconst;
    }

    public String getCategory() {
        return category;
    }

    public void setCategory(final String category) {
        this.category = category;
    }

    public List<String> getCharacters() {
        return characters;
    }

    public void setCharacters(final List<String> characters) {
        this.characters = characters;
    }

    @Override
    public String toString() {
        final var charactersStr = StringUtils.joinWith(" // ", characters);
        return "Person{" +
                "nconst='" + nconst + '\'' +
                ", id='" + getId() + '\'' +
                ", ordering='" + ordering + '\'' +
                ", nconst='" + nconst + '\'' +
                ", category='" + category + '\'' +
                ", characters=" + charactersStr +
                '}';
    }
}
