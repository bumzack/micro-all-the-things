package at.bumzack.foryouandyourfakewebshop.searcharticle;

import at.bumzack.foryouandyourfakewebshop.searcharticle.model.PriceEntry;
import at.bumzack.foryouandyourfakewebshop.searcharticle.model.PriceEntryRepository;
import at.bumzack.foryouandyourfakewebshop.searcharticle.model.SearchPricesRequest;
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
import org.springframework.http.MediaType;
import org.springframework.lang.NonNull;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.ResponseBody;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

import static java.util.Objects.nonNull;
import static org.springframework.web.bind.annotation.RequestMethod.GET;
import static org.springframework.web.bind.annotation.RequestMethod.POST;


@RestController("Java8PriceServiceController")
public class Java8PriceServiceController {

    private static final Logger LOG = LogManager.getLogger(Java8PriceServiceController.class);
    private final PriceEntryRepository priceEntryRepository;

    public Java8PriceServiceController(final PriceEntryRepository priceEntryRepository) {
        this.priceEntryRepository = priceEntryRepository;
    }

    @NonNull
    @GetMapping("/api/v1/price/{tconst}")
    @ResponseBody
    @RouterOperation(path = "/api/v1/price/{tconst}",
            method = GET,
            operation = @Operation(operationId = "getPriceForMovie",
                    parameters = {@Parameter(in = ParameterIn.PATH, name = "tconst", description = "Movie tconst ID", schema = @Schema(implementation = String.class))
                    }, responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = PriceEntry.class))),
            })
    )
    public PriceEntry getPriceForMovie(@PathVariable final String tconst) {
        LOG.info("yo   getPriceForMovie   tconst '{}'", tconst);
        final PriceEntry byMovieTconst = priceEntryRepository.findByMovieTconst(tconst);
        if (nonNull(byMovieTconst)) {
            LOG.info("found a price entry {}", byMovieTconst);
        } else {
            LOG.info("did not find a price entry for tconst '{}", tconst);
        }
        return byMovieTconst;
    }

    @NonNull
    @PostMapping(value = "/api/v2/prices", produces = MediaType.APPLICATION_JSON_VALUE)
    @ResponseBody
    @RouterOperation(path = "/api/v2/prices",
            method = POST,
            operation = @Operation(operationId = "getPricesForMovies",
                    requestBody = @io.swagger.v3.oas.annotations.parameters.RequestBody(content = @Content(schema = @Schema(implementation = SearchPricesRequest.class))),
                    responses = {@ApiResponse(responseCode = "200", content = @Content(array = @ArraySchema(schema = @Schema(implementation = PriceEntry.class)))),
                    })
    )
    public List<PriceEntry> getPricesForMovies(@RequestBody final SearchPricesRequest request) {
        LOG.info("yoyo getPricseForMovies.  request {}", request);
        final List<PriceEntry> prices = priceEntryRepository.findPrices(request.getMovieTconst());

        if (nonNull(prices)) {
            LOG.info("found  price entries {}", prices);
        } else {
            LOG.info("did not find a price entry for request '{}", request);
        }
        return prices;

    }
}
