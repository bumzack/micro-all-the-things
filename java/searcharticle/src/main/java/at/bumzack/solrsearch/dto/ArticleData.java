package at.bumzack.solrsearch.dto;

import java.io.Serializable;
import java.util.List;


public class ArticleData implements Serializable {

    private static final long serialVersionUID = 1L;


    private B2BArticleChannelInfoData article;
    private List<XinetData> images;

    public ArticleData() {
    }

    public B2BArticleChannelInfoData getArticle() {
        return article;
    }

    public void setArticle(final B2BArticleChannelInfoData article) {
        this.article = article;
    }

    public List<XinetData> getImages() {
        return images;
    }

    public void setImages(final List<XinetData> images) {
        this.images = images;
    }
}
