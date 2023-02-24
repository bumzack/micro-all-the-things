package at.bumzack.solrsearch.dto;

import java.io.Serializable;


public class SearchRequestData implements Serializable {

    private static final long serialVersionUID = 1L;

    private String code;
    private String text;
        private int pageSize;
    private int start;

    public SearchRequestData() {
    }

    public String getCode() {
        return code;
    }

    public void setCode(final String code) {
        this.code = code;
    }

    public String getText() {
        return text;
    }

    public void setText(final String text) {
        this.text = text;
    }


    public int getPageSize() {
        return pageSize;
    }

    public void setPageSize(final int pageSize) {
        this.pageSize = pageSize;
    }

    public int getStart() {
        return start;
    }

    public void setStart(final int start) {
        this.start = start;
    }

    @Override
    public String toString() {
        return "SearchRequestData{" +
                "code='" + code + '\'' +
                ", text='" + text + '\'' +
                ", pageSize=" + pageSize +
                ", start=" + start +
                '}';
    }
}
