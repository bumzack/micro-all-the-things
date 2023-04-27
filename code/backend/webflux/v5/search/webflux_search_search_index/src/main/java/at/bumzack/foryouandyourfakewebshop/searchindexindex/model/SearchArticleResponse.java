package at.bumzack.foryouandyourfakewebshop.searchindexindex.model;

import java.util.List;

public record SearchArticleResponse(List<SearchDoc> movies) {
}

// , IndexDocFacetDistribution facets)