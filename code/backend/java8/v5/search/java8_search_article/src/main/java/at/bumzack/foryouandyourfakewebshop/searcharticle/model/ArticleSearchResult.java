package at.bumzack.foryouandyourfakewebshop.searcharticle.model;

import at.bumzack.searchindexservice.model.SearchDoc;

public class ArticleSearchResult {
    private SearchDoc article;
    private Double price;
    private Double customerPrice;

    public ArticleSearchResult() {
    }

    public SearchDoc getArticle() {
        return article;
    }

    public void setArticle(final SearchDoc article) {
        this.article = article;
    }

    public Double getPrice() {
        return price;
    }

    public void setPrice(final Double price) {
        this.price = price;
    }

    public Double getCustomerPrice() {
        return customerPrice;
    }

    public void setCustomerPrice(final Double customerPrice) {
        this.customerPrice = customerPrice;
    }

    @Override
    public String toString() {
        return "ArticleSearchResult{" +
                "article=" + article +
                ", price=" + price +
                ", customerPrice=" + customerPrice +
                '}';
    }
}


