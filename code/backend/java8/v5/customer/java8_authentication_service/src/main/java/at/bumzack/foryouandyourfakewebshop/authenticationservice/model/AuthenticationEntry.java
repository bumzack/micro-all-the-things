package at.bumzack.foryouandyourfakewebshop.authenticationservice.model;

import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Table;

import java.time.LocalDateTime;

@Table(name = "authentication")
@Entity
public class AuthenticationEntry {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    private Long customerId;
    private String jwt;
    private LocalDateTime loggedIn;
    private LocalDateTime loggedOut;
    private LocalDateTime created;

    public AuthenticationEntry() {
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
}

