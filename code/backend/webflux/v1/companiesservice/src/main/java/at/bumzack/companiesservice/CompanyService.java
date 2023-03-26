package at.bumzack.companiesservice;


import at.bumzack.dto.Company;
import io.swagger.v3.oas.annotations.tags.Tag;
import org.springframework.stereotype.Service;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;


@Service
@Tag(name = "UsersService")
public class CompanyService {

    private final CompanyRepository companyRepository;

    public CompanyService(final CompanyRepository companyRepository) {
        this.companyRepository = companyRepository;
    }

    public Mono<Company> findByCOde(final String code) {
        return companyRepository.findByCode(code);
    }

    public Flux<Company> findAll() {
        return companyRepository.findAll();
    }
}
