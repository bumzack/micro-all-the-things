package at.bumzack.foryouandyourfakewebshop.customerservice.model;

import org.springframework.data.relational.core.mapping.Table;

import java.time.LocalDateTime;

@Table("customer")
public record Customer(Long id, String firstName, String lastName, String email, String password, LocalDateTime created) {
}
