package at.bumzack.foryouandyourfakewebshop.authenticationservice.model;

public final class AuthenticatedCustomer {
    private Long customerId;
    private String jwt;

    public AuthenticatedCustomer() {
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

    @Override
    public String toString() {
        return "AuthenticatedCustomer{" +
                "customerId=" + customerId +
                ", jwt='" + jwt + '\'' +
                '}';
    }
}
