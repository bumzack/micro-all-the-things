package at.bumzack.foryouandyourfakewebshop.searcharticle.model;

import org.springframework.data.annotation.Id;
import org.springframework.data.relational.core.mapping.Table;

import java.time.LocalDateTime;


@Table("price")
public record PriceEntry(@Id Long id, String movieTconst, Double amount, LocalDateTime created) {
}


