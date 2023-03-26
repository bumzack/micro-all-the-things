package at.bumzack.companiesservice;


import at.bumzack.dto.Company;
import org.springframework.data.repository.reactive.ReactiveCrudRepository;
import reactor.core.publisher.Mono;

public interface CompanyRepository extends ReactiveCrudRepository<Company, Long> {

    Mono<Company> findByCode(final String code);

}
