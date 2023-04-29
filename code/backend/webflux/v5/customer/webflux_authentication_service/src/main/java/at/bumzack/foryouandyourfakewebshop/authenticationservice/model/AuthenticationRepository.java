package at.bumzack.foryouandyourfakewebshop.authenticationservice.model;


import org.springframework.data.repository.reactive.ReactiveCrudRepository;
import org.springframework.stereotype.Component;
import reactor.core.publisher.Mono;

@Component
public interface AuthenticationRepository extends ReactiveCrudRepository<AuthenticationEntry, Long> {

    Mono<AuthenticationEntry> findById(final Long id);

    Mono<AuthenticationEntry> findByCustomerId(final Long customerId);

    Mono<AuthenticationEntry> findByCustomerIdAndJwtNotNull(final Long customerId);

}

