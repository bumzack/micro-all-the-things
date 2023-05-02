package at.bumzack.foryouandyourfakewebshop.searchindexindex.model;

import org.springframework.data.repository.reactive.ReactiveCrudRepository;
import reactor.core.publisher.Flux;

public interface CustomerPriceEntryRepository extends ReactiveCrudRepository<CustomerPriceEntry, Long> {
    Flux<CustomerPriceEntry> findByCustomerId(String customerId);
}
