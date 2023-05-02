package at.bumzack.foryouandyourfakewebshop.searchindexindex.model;

import org.springframework.data.jpa.repository.JpaRepository;

import java.util.List;

public interface CustomerPriceEntryRepository extends JpaRepository<CustomerPriceEntry, Long> {
    List<CustomerPriceEntry> findByCustomerId(Long customerId);
}
