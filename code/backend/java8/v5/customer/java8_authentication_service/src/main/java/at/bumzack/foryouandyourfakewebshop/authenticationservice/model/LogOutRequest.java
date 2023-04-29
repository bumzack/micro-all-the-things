package at.bumzack.foryouandyourfakewebshop.authenticationservice.model;

public final class LogOutRequest {
    private Long customerId;

    public LogOutRequest() {
    }

    @Override
    public String toString() {
        return "LogOutRequest{" +
                "customerId=" + customerId +
                '}';
    }

    public Long getCustomerId() {
        return customerId;
    }

    public void setCustomerId(final Long customerId) {
        this.customerId = customerId;
    }
}