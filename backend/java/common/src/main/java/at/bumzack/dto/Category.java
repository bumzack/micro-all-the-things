package at.bumzack.dto;

import at.bumzack.solr.SolrResponse;
import org.springframework.core.ParameterizedTypeReference;

import java.io.Serializable;
import java.util.List;

@SolrDocument
public class Category extends AbstractItem implements Serializable {
    public static final ParameterizedTypeReference<SolrResponse<Category>> TYPE_REF_CATERORY = new ParameterizedTypeReference<>() {
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
}
