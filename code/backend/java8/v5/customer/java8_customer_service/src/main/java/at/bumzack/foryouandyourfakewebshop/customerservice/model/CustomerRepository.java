package at.bumzack.foryouandyourfakewebshop.customerservice.model;

import org.springframework.data.jpa.repository.JpaRepository;

// import org.springframework.data.r2dbc.repository.Query;


public interface CustomerRepository extends JpaRepository<Customer, Long> {
    Customer findByEmail(final String email);
}
