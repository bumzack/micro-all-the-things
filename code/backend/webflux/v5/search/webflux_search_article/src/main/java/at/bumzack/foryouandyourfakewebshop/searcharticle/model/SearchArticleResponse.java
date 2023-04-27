package at.bumzack.foryouandyourfakewebshop.searcharticle.model;

import java.util.List;

public record SearchArticleResponse(List<ArticleSearchResult> articles) {
}

// , IndexDocFacetDistribution facets)