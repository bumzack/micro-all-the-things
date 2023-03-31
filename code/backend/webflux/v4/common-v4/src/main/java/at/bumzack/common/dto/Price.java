package at.bumzack.common.dto;

import at.bumzack.common.solr.SolrResponse;
import org.springframework.core.ParameterizedTypeReference;

import java.math.BigDecimal;

@SolrDocument
public class Price {
    public static final ParameterizedTypeReference<SolrResponse<Price>> TYPE_REF_PRICE = new ParameterizedTypeReference<>() {
    };

    private String articleCode;
    private BigDecimal price;


    public Price() {
    }

    @Override
    public String toString() {
        return "Price{" +
                "articleCode='" + articleCode + '\'' +
                ", price=" + price +
                '}';
    }

    public String getArticleCode() {
        return articleCode;
    }

    public void setArticleCode(final String articleCode) {
        this.articleCode = articleCode;
    }

    public BigDecimal getPrice() {
        return price;
    }

    public void setPrice(final BigDecimal price) {
        this.price = price;
    }
}
