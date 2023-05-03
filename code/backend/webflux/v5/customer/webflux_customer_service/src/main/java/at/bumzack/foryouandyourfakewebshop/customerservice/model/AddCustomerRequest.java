package at.bumzack.foryouandyourfakewebshop.customerservice.model;

public record AddCustomerRequest(String firstName, String lastName, String email, String password) {
}

