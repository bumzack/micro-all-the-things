package at.bumzack.dto;

import java.io.Serializable;

public class ArticleData implements Serializable {

    private static final long serialVersionUID = 1L;

    private Product product;
    private Image image;

    public ArticleData() {
    }

    public Product getProduct() {
        return product;
    }

    public void setProduct(final Product product) {
        this.product = product;
    }

    public Image getImage() {
        return image;
    }

    public void setImage(final Image image) {
        this.image = image;
    }

    @Override
    public String toString() {
        return "ArticleData{" +
                "product=" + product +
                ", image=" + image +
                '}';
    }
}
