package at.bumzack.foryouandyourfakewebshop.searchindexindex.model;

import org.springframework.data.annotation.Id;
import org.springframework.data.relational.core.mapping.Table;

import java.time.LocalDateTime;


@Table("customer_price")
public record CustomerPriceEntry(@Id Long id, Long customerId, Double discount, Integer startYear, Integer endYear, LocalDateTime created) {
}


