package at.bumzack.foryouandyourfakewebshop.searcharticle.model;

public record SearchArticleRequest(String q, Integer offset, Integer limit, SearchCustomer customer) {
}

