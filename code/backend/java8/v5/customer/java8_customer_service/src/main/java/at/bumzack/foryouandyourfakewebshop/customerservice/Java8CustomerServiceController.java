package at.bumzack.foryouandyourfakewebshop.customerservice;

import at.bumzack.foryouandyourfakewebshop.customerservice.model.AddCustomerRequest;
import at.bumzack.foryouandyourfakewebshop.customerservice.model.Customer;
import at.bumzack.foryouandyourfakewebshop.customerservice.model.CustomerRepository;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.enums.ParameterIn;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.springdoc.core.annotations.RouterOperation;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.lang.NonNull;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.ResponseBody;
import org.springframework.web.reactive.function.client.WebClientResponseException;

import static java.util.Objects.nonNull;
import static org.springframework.web.bind.annotation.RequestMethod.GET;
import static org.springframework.web.bind.annotation.RequestMethod.POST;


@Controller("Java8CustomerServiceController")
@CrossOrigin
public class Java8CustomerServiceController {

    private static final Logger LOG = LogManager.getLogger(Java8CustomerServiceController.class);

    private final CustomerRepository customerRepository;

    public Java8CustomerServiceController(final CustomerRepository customerRepository) {
        this.customerRepository = customerRepository;
    }

    @NonNull
    @RouterOperation(path = "/api/v1/customer/{email}",
            method = GET,
            operation = @Operation(operationId = "getCustomer",
                    parameters = {@Parameter(in = ParameterIn.PATH, name = "email", description = "Email address of customer", schema = @Schema(implementation = String.class))
                    },
                    responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = Customer.class))),
                    })
    )
    @ResponseBody
    @GetMapping(value = "/api/v1/customer/{email}")
    public ResponseEntity<Customer> getCustomer(@PathVariable final String email) {
        final Customer customer = customerRepository.findByEmail(email);
        if (nonNull(customer)) {
            return ResponseEntity.ok(customer);
        }
        return ResponseEntity.notFound().build();

    }

    @NonNull
    @RouterOperation(path = "/api/v1/customer",
            method = POST,
            operation = @Operation(operationId = "addCustomer",
                    requestBody = @io.swagger.v3.oas.annotations.parameters.RequestBody(content = @Content(schema = @Schema(implementation = AddCustomerRequest.class))),
                    responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = Customer.class))),
                    })
    )
    @ResponseBody
    @PostMapping(value = "/api/v1/customer", produces = MediaType.APPLICATION_JSON_VALUE)
    public ResponseEntity<Customer> addCustomer(@RequestBody final AddCustomerRequest request) throws WebClientResponseException {
        final var c = new Customer();
        c.setEmail(request.getEmail());
        c.setFirstName(request.getFirstName());
        c.setLastName(request.getLastName());
        c.setPassword(request.getPassword());

        if (nonNull(c)) {
            return ResponseEntity.ok(c);
        }
        return ResponseEntity.notFound().build();
    }
}
