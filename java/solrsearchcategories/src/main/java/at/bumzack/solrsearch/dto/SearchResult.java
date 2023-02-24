package at.bumzack.solrsearch.dto;

import java.io.Serializable;
import java.util.List;
import java.util.stream.Collectors;


public class SearchResult implements Serializable {

    private static final long serialVersionUID = 1L;

    private List<CategoryData> categories;

    public SearchResult() {
    }

    public List<CategoryData> getCategories() {
        return categories;
    }

    public void setCategories(final List<CategoryData> categories) {
        this.categories = categories;
    }

    @Override
    public String toString() {
        return "SearchResult{" +
                "articles=" + categories.stream().map(CategoryData::getCode).collect(Collectors.joining(" / ")) +
                '}';
    }
}
