package at.bumzack.solrsearch.dto;

import java.io.Serializable;
import java.util.List;
import java.util.stream.Collectors;


public class SearchResult implements Serializable {

    private static final long serialVersionUID = 1L;

    private List<XinetData> categories;

    public SearchResult() {
    }

    public List<XinetData> getCategories() {
        return categories;
    }

    public void setCategories(final List<XinetData> categories) {
        this.categories = categories;
    }

    @Override
    public String toString() {
        return "SearchResult{" +
                "categories=" + categories.stream().map(XinetData::getCode).collect(Collectors.joining(" / ")) +
                '}';
    }
}
