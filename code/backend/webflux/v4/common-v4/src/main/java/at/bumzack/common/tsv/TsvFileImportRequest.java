package at.bumzack.common.tsv;

public class TsvFileImportRequest {
    private TsvEnum tsvType;
    private int start;
    private int end;
    private int pageSize;

    public TsvFileImportRequest() {
    }

    public TsvEnum getTsvType() {
        return tsvType;
    }

    public void setTsvType(final TsvEnum tsvType) {
        this.tsvType = tsvType;
    }

    public int getStart() {
        return start;
    }

    public void setStart(final int start) {
        this.start = start;
    }

    public int getEnd() {
        return end;
    }

    public void setEnd(final int end) {
        this.end = end;
    }

    public int getPageSize() {
        return pageSize;
    }

    public void setPageSize(final int pageSize) {
        this.pageSize = pageSize;
    }

    @Override
    public String toString() {
        return "TsvFileImportRequest{" +
                "tsvType=" + tsvType +
                ", start=" + start +
                ", end=" + end +
                ", pageSize=" + pageSize +
                '}';
    }
}
