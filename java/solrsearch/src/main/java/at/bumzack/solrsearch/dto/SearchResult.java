package at.bumzack.solrsearch.dto;

import java.io.Serializable;
import java.util.List;
import java.util.stream.Collectors;


public class SearchResult implements Serializable {

    private static final long serialVersionUID = 1L;

    private List<B2BArticleChannelInfoData> articles;

    public SearchResult() {
    }

    public List<B2BArticleChannelInfoData> getArticles() {
        return articles;
    }

    public void setArticles(final List<B2BArticleChannelInfoData> articles) {
        this.articles = articles;
    }

    @Override
    public String toString() {
        return "SearchResult{" +
                "articles=" + articles.stream().map(B2BArticleChannelInfoData::getCode).collect(Collectors.joining(" / ")) +
                '}';
    }
}
