package at.bumzack.foryouandyourfakewebshop.searcharticle.model;

import at.bumzack.searchindexservice.model.SearchDoc;

public record ArticleSearchResult(SearchDoc article, Double price, Double customerPrice) {
}


