package at.bumzack.foryouandyourfakewebshop.searcharticle.model;

public class SearchArticleRequest {
    private String q;
    private Integer offset;
    private Integer limit;
    private SearchCustomer customer;

    public SearchArticleRequest() {
    }

    public String getQ() {
        return q;
    }

    public void setQ(final String q) {
        this.q = q;
    }

    public Integer getOffset() {
        return offset;
    }

    public void setOffset(final Integer offset) {
        this.offset = offset;
    }

    public Integer getLimit() {
        return limit;
    }

    public void setLimit(final Integer limit) {
        this.limit = limit;
    }

    public SearchCustomer getCustomer() {
        return customer;
    }

    public void setCustomer(final SearchCustomer customer) {
        this.customer = customer;
    }

    @Override
    public String toString() {
        return "SearchArticleRequest{" +
                "q='" + q + '\'' +
                ", offset=" + offset +
                ", limit=" + limit +
                ", customer=" + customer +
                '}';
    }
}

