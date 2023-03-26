package at.bumzack.common.dto;

import at.bumzack.common.search.SearchResult;
import at.bumzack.common.solr.SolrResponse;
import org.springframework.core.ParameterizedTypeReference;

import java.io.Serializable;
import java.util.List;

@SolrDocument
public class Category extends AbstractItem implements Serializable {
    public static final ParameterizedTypeReference<SolrResponse<Category>> TYPE_REF_CATEGORY = new ParameterizedTypeReference<>() {
    };

    public static final ParameterizedTypeReference<SearchResult<Category>> TYPE_REF_SEARCH_RESULT_CATEGORY = new ParameterizedTypeReference<>() {
    };

    private static final long serialVersionUID = 1L;

    private String name;
    private List<String> superCategories;
    private List<String> allSuperCategories;

    public Category() {
    }


    public String getName() {
        return name;
    }

    public void setName(final String name) {
        this.name = name;
    }

    public List<String> getSuperCategories() {
        return superCategories;
    }

    public void setSuperCategories(final List<String> superCategories) {
        this.superCategories = superCategories;
    }

    public List<String> getAllSuperCategories() {
        return allSuperCategories;
    }

    public void setAllSuperCategories(final List<String> allSuperCategories) {
        this.allSuperCategories = allSuperCategories;
    }

    @Override
    public String toString() {
        return "Category{" +
                "name='" + name + '\'' +
                ", superCategories=" + superCategories +
                ", allSuperCategories=" + allSuperCategories +
                '}';
    }
}
