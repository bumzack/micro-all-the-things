package at.bumzack.foryouandyourfakewebshop.searcharticle.model;

public class SearchCustomer {
    private Long customerId;
    private String jwt;

    public SearchCustomer() {
    }

    @Override
    public String toString() {
        return "SearchCustomer{" +
                "customerId=" + customerId +
                ", jwt='" + jwt + '\'' +
                '}';
    }

    public Long getCustomerId() {
        return customerId;
    }

    public void setCustomerId(final Long customerId) {
        this.customerId = customerId;
    }

    public String getJwt() {
        return jwt;
    }

    public void setJwt(final String jwt) {
        this.jwt = jwt;
    }
}
