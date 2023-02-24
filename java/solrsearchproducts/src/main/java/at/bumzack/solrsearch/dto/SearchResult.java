package at.bumzack.solrsearch.dto;

import java.io.Serializable;
import java.util.List;
import java.util.stream.Collectors;


public class SearchResult implements Serializable {

    private static final long serialVersionUID = 1L;

    private List<B2BArticleChannelInfoData> products;

    public SearchResult() {
    }

    public List<B2BArticleChannelInfoData> getProducts() {
        return products;
    }

    public void setProducts(final List<B2BArticleChannelInfoData> products) {
        this.products = products;
    }

    @Override
    public String toString() {
        return "SearchResult{" +
                "products=" + products.stream().map(B2BArticleChannelInfoData::getCode).collect(Collectors.joining(" / ")) +
                '}';
    }
}
