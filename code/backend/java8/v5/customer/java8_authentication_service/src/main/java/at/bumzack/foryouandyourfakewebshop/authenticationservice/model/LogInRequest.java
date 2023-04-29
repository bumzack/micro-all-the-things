package at.bumzack.foryouandyourfakewebshop.authenticationservice.model;


public final class LogInRequest {
    private String email;
    private String password;

    public LogInRequest() {
    }

    @Override
    public String toString() {
        return "LogInRequest{" +
                "email='" + email + '\'' +
                ", password='" + password + '\'' +
                '}';
    }

    public String getEmail() {
        return email;
    }

    public void setEmail(final String email) {
        this.email = email;
    }

    public String getPassword() {
        return password;
    }

    public void setPassword(final String password) {
        this.password = password;
    }
}








