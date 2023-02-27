package at.bumzack.dto;

import at.bumzack.solr.SolrResponse;
import org.springframework.core.ParameterizedTypeReference;

import java.io.Serializable;

@SolrDocument
public class Image extends AbstractItem implements Serializable {
    public static final ParameterizedTypeReference<SolrResponse<Image>> TYPE_REF_IMAGE = new ParameterizedTypeReference<SolrResponse<Image>>() {
    };

    private static final long serialVersionUID = 2L;

    private String mediaContainerQualifier;
    private String url;
    private String width;
    private String height;
    private String channel;
    private Boolean valid;
    private String mediaFormat;
    private String mime;


    public Image() {
    }

    public String getMediaContainerQualifier() {
        return mediaContainerQualifier;
    }

    public void setMediaContainerQualifier(final String mediaContainerQualifier) {
        this.mediaContainerQualifier = mediaContainerQualifier;
    }

    public String getUrl() {
        return url;
    }

    public void setUrl(final String url) {
        this.url = url;
    }

    public String getWidth() {
        return width;
    }

    public void setWidth(final String width) {
        this.width = width;
    }

    public String getHeight() {
        return height;
    }

    public void setHeight(final String height) {
        this.height = height;
    }

    public String getChannel() {
        return channel;
    }

    public void setChannel(final String channel) {
        this.channel = channel;
    }

    public Boolean getValid() {
        return valid;
    }

    public void setValid(final Boolean valid) {
        this.valid = valid;
    }

    public String getMediaFormat() {
        return mediaFormat;
    }

    public void setMediaFormat(final String mediaFormat) {
        this.mediaFormat = mediaFormat;
    }

    public String getMime() {
        return mime;
    }

    public void setMime(final String mime) {
        this.mime = mime;
    }
}
