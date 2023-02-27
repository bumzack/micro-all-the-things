package at.bumzack.dto;

import java.io.Serializable;
import java.util.List;

public class ArticleData implements Serializable {

    private static final long serialVersionUID = 1L;

    private Product product;
    private List<Image> images;

    public ArticleData() {
    }

    public Product getProduct() {
        return product;
    }

    public void setProduct(final Product product) {
        this.product = product;
    }

    public List<Image> getImages() {
        return images;
    }

    public void setImages(final List<Image> images) {
        this.images = images;
    }
}
