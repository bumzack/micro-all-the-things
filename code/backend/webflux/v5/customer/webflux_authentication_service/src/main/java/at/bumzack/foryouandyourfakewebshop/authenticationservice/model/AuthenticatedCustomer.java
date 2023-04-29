package at.bumzack.foryouandyourfakewebshop.authenticationservice.model;

public record AuthenticatedCustomer(Long customerId, String jwt) {
}
