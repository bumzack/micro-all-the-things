package at.bumzack.foryouandyourfakewebshop.searcharticle.model;

import java.util.List;

public class SearchArticleResponse {
    private List<ArticleSearchResult> articles;

    public SearchArticleResponse() {
    }

    public List<ArticleSearchResult> getArticles() {
        return articles;
    }

    public void setArticles(final List<ArticleSearchResult> articles) {
        this.articles = articles;
    }

    @Override
    public String toString() {
        return "SearchArticleResponse{" +
                "articles=" + articles +
                '}';
    }
}

// , IndexDocFacetDistribution facets)