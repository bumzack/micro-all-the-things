package at.bumzack.foryouandyourfakewebshop.authenticationservice.model;


import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Component;

@Component
public interface AuthenticationRepository extends JpaRepository<AuthenticationEntry, Long> {

    AuthenticationEntry findByCustomerId(final Long customerId);

    AuthenticationEntry findByCustomerIdAndJwtNotNull(final Long customerId);

}

