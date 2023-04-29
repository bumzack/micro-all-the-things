package at.bumzack.foryouandyourfakewebshop.customerservice.model;

import org.springframework.data.repository.reactive.ReactiveCrudRepository;
import reactor.core.publisher.Mono;

// import org.springframework.data.r2dbc.repository.Query;


public interface CustomerRepository extends ReactiveCrudRepository<Customer, Long> {
    Mono<Customer> findByEmail(final String email);
}
