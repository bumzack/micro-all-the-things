package at.bumzack.solrsearch.dto;

import java.io.Serializable;
import java.util.List;

public class CategoryData implements Serializable {

    private static final long serialVersionUID = 1L;

    private String id;
    private String code;
    private String name;
    private List<String> superCategories;
    private List<String> allSuperCategories;

    public CategoryData() {
    }

    public String getCode() {
        return code;
    }

    public void setCode(final String code) {
        this.code = code;
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

    public String getId() {
        return id;
    }

    public void setId(final String id) {
        this.id = id;
    }
}
