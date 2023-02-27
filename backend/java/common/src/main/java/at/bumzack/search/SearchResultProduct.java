package at.bumzack.search;


import at.bumzack.dto.Product;

import java.io.Serializable;
import java.util.List;
import java.util.stream.Collectors;

public class SearchResultProduct implements Serializable {

    private static final long serialVersionUID = 1L;

    private List<Product> products;

    public SearchResultProduct() {
    }

    public List<Product> getProducts() {
        return products;
    }

    public void setProducts(final List<Product> products) {
        this.products = products;
    }
}
