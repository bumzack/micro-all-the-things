package at.bumzack.solrsearch.dto;

import java.io.Serializable;
import java.util.List;
import java.util.stream.Collectors;


public class SearchResult implements Serializable {

    private static final long serialVersionUID = 1L;

    private List<ArticleData> articles;

    public SearchResult() {
    }

    public List<ArticleData> getArticles() {
        return articles;
    }

    public void setArticles(final List<ArticleData> articles) {
        this.articles = articles;
    }
}
