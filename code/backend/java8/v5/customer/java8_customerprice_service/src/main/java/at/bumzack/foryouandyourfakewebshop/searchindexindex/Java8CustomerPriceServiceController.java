package at.bumzack.foryouandyourfakewebshop.searchindexindex;

import at.bumzack.foryouandyourfakewebshop.searchindexindex.model.CustomerPriceEntry;
import at.bumzack.foryouandyourfakewebshop.searchindexindex.model.CustomerPriceEntryRepository;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.enums.ParameterIn;
import io.swagger.v3.oas.annotations.media.ArraySchema;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.springdoc.core.annotations.RouterOperation;
import org.springframework.http.ResponseEntity;
import org.springframework.lang.NonNull;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.ResponseBody;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.reactive.function.client.WebClientResponseException;

import java.util.List;

import static java.util.Objects.nonNull;
import static org.springframework.web.bind.annotation.RequestMethod.GET;


@RestController("Java8CustomerPriceServiceController")
@CrossOrigin
public class Java8CustomerPriceServiceController {

    private static final Logger LOG = LogManager.getLogger(Java8CustomerPriceServiceController.class);

    private final CustomerPriceEntryRepository customerPriceEntryRepository;

    public Java8CustomerPriceServiceController(final CustomerPriceEntryRepository customerPriceEntryRepository) {
        this.customerPriceEntryRepository = customerPriceEntryRepository;
    }

    @NonNull
    @RouterOperation(path = "/api/v1/customerprices/{customerId}",
            method = GET,
            operation = @Operation(operationId = "customerPrices",
                    parameters = {@Parameter(in = ParameterIn.PATH, name = "customerId", description = "Customer ID", schema = @Schema(implementation = Long.class))
                    },
                    responses = {@ApiResponse(responseCode = "200", content = @Content(array = @ArraySchema(schema = @Schema(implementation = CustomerPriceEntry.class)))),
                    })
    )
    @GetMapping(value = "/api/v1/customerprices/{customerId}")
    @ResponseBody
    public ResponseEntity<List<CustomerPriceEntry>> customerPrices(@PathVariable final Long customerId) throws WebClientResponseException {
        LOG.info("customer prices for customerId {}", customerId);
        final List<CustomerPriceEntry> byCustomerId = customerPriceEntryRepository.findByCustomerId(customerId);
        LOG.info("got prices {}", byCustomerId);

        if (nonNull(byCustomerId)) {
            return ResponseEntity.ok(byCustomerId);
        }
        return ResponseEntity.notFound().build();

    }
}
