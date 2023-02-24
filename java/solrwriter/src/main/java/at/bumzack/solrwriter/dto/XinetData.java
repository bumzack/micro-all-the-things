package at.bumzack.solrwriter.dto;

import java.io.Serializable;

public class XinetData implements Serializable {

    private static final long serialVersionUID = 1L;


    private String id;
    private String code;
    private String mediaContainerQualifier;
    private String URL;
    private String width;
    private String height;
    private String channel;
    private Boolean valid;
    private String mediaFormat;
    private String mime;


    public XinetData() {
    }

    public String getCode() {
        return code;
    }

    public void setCode(final String code) {
        this.code = code;
    }

    public String getMediaContainerQualifier() {
        return mediaContainerQualifier;
    }

    public void setMediaContainerQualifier(final String mediaContainerQualifier) {
        this.mediaContainerQualifier = mediaContainerQualifier;
    }

    public String getURL() {
        return URL;
    }

    public void setURL(final String URL) {
        this.URL = URL;
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

    public String getId() {
        return id;
    }

    public void setId(final String id) {
        this.id = id;
    }
}
