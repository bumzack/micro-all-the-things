package at.bumzack.foryouandyourfakewebshop.authenticationservice.model;

import org.springframework.data.annotation.Id;
import org.springframework.data.relational.core.mapping.Table;

import java.time.LocalDateTime;

@Table("authentication")
public class AuthenticationEntry {
    @Id
    private Long id;

    private Long customerId;
    private String jwt;
    private LocalDateTime loggedIn;
    private LocalDateTime loggedOut;
    private LocalDateTime created;

    public AuthenticationEntry() {
    }

    @Override
    public String toString() {
        return "AuthenticationEntry{" +
                "id=" + id +
                ", customerId=" + customerId +
                ", jwt='" + jwt + '\'' +
                ", loggedIn=" + loggedIn +
                ", loggedOut=" + loggedOut +
                ", created=" + created +
                '}';
    }

    public Long getId() {
        return id;
    }

    public void setId(final Long id) {
        this.id = id;
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

    public LocalDateTime getLoggedIn() {
        return loggedIn;
    }

    public void setLoggedIn(final LocalDateTime loggedIn) {
        this.loggedIn = loggedIn;
    }

    public LocalDateTime getLoggedOut() {
        return loggedOut;
    }

    public void setLoggedOut(final LocalDateTime loggedOut) {
        this.loggedOut = loggedOut;
    }

    public LocalDateTime getCreated() {
        return created;
    }

    public void setCreated(final LocalDateTime created) {
        this.created = created;
    }
}

