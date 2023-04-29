package at.bumzack.foryouandyourfakewebshop.authenticationservice;


import at.bumzack.customerservice.ApiClient;
import at.bumzack.customerservice.ApiException;
import at.bumzack.customerservice.api.DefaultApi;
import at.bumzack.customerservice.model.Customer;
import at.bumzack.foryouandyourfakewebshop.authenticationservice.model.AuthenticationEntry;
import at.bumzack.foryouandyourfakewebshop.authenticationservice.model.AuthenticationRepository;
import at.bumzack.foryouandyourfakewebshop.authenticationservice.model.LogInRequest;
import at.bumzack.foryouandyourfakewebshop.authenticationservice.model.LogOutRequest;
import io.jsonwebtoken.Jwts;
import io.jsonwebtoken.SignatureAlgorithm;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.enums.ParameterIn;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.springdoc.core.annotations.RouterOperation;
import org.springframework.beans.factory.annotation.Value;
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

import java.time.LocalDateTime;
import java.util.Date;
import java.util.HashMap;

import static java.util.Objects.isNull;
import static java.util.Objects.nonNull;
import static org.springframework.web.bind.annotation.RequestMethod.GET;
import static org.springframework.web.bind.annotation.RequestMethod.POST;


@Controller("Java8AuthenticationServiceController")
@CrossOrigin
public class Java8AuthenticationServiceController {

    private static final Logger LOG = LogManager.getLogger(Java8AuthenticationServiceController.class);
    private final AuthenticationRepository authenticationRepository;

    @Value("${jwt.secret}")
    private String secret;

    @Value("${jwt.expiration}")
    private String expirationTime;

    public Java8AuthenticationServiceController(final AuthenticationRepository authenticationRepository) {
        this.authenticationRepository = authenticationRepository;
    }

    @NonNull
    @RouterOperation(path = "/api/v1/authenticated/{customerId}",
            method = GET,
            operation = @Operation(operationId = "loggedin",
                    parameters = {@Parameter(in = ParameterIn.PATH, name = "customerId", description = "Customer ID", schema = @Schema(implementation = Long.class))
                    },
                    responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = AuthenticationEntry.class))),
                    })
    )
    @ResponseBody
    @GetMapping(value = "/api/v1/authenticated/{customerId}", produces = MediaType.APPLICATION_JSON_VALUE)
    public ResponseEntity<AuthenticationEntry> loggedIn(@PathVariable final Long customerId) {
        final AuthenticationEntry authenticationEntry = authenticationRepository.findByCustomerIdAndJwtNotNull(customerId);
        LOG.info("found or did not find a auth entry for customerId {},  auth {}", customerId, authenticationEntry);
        if (nonNull(authenticationEntry)) {
            return ResponseEntity.ok(authenticationEntry);
        }
        return ResponseEntity.notFound().build();
    }

    @NonNull
    @RouterOperation(path = "/api/v1/authentication/login",
            method = POST,
            operation = @Operation(operationId = "login",
                    requestBody = @io.swagger.v3.oas.annotations.parameters.RequestBody(content = @Content(schema = @Schema(implementation = LogInRequest.class))),
                    responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = AuthenticationEntry.class))),
                    })
    )
    @ResponseBody
    @PostMapping(value = "/api/v1/authentication/login", produces = MediaType.APPLICATION_JSON_VALUE)
    public ResponseEntity<AuthenticationEntry> login(@RequestBody final LogInRequest request) throws WebClientResponseException, ApiException {
        LOG.info("login. request  {}", request);

        final ApiClient apiClient = new ApiClient();
        apiClient.setPort(28980);
        final DefaultApi api = new DefaultApi(apiClient);

        final var customer = api.getCustomer(request.getEmail());

        if (isNull(customer)) {
            LOG.info("login. customer not found with email  {}", request.getEmail());
            return ResponseEntity.notFound().build();
        }

        final AuthenticationEntry authenticationEntry = authenticationRepository.findByCustomerId(customer.getId());
        if (nonNull(authenticationEntry) && nonNull(authenticationEntry.getJwt())) {
            LOG.info("login. customer already logged in  {}", authenticationEntry);
            return ResponseEntity.ok(authenticationEntry);
        }

        if (request.getEmail().equals(customer.getEmail()) && (request.getPassword().equals(customer.getPassword()))) {
            return login(authenticationEntry, customer);
        }

        LOG.info("login. customer NOT logged in with email/pw  {}", request);
        return ResponseEntity.notFound().build();
    }

    private ResponseEntity<AuthenticationEntry> login(   AuthenticationEntry    auth,final Customer customer) {
        if (isNull(auth)) {
            final var jwt = getToken(customer.getEmail());
            final var newAuth = new AuthenticationEntry();

           newAuth.setJwt(jwt);
           newAuth.setCustomerId(customer.getId());
           newAuth.setLoggedIn(LocalDateTime.now());

            final var loggedIn = authenticationRepository.save(newAuth);
            LOG.info("login. customer successfully    logged in with email/pw  {}", loggedIn);
            return ResponseEntity.ok(loggedIn);
        }
        final var jwt = getToken(customer.getEmail());

        auth.setJwt(jwt);
        auth.setLoggedIn(LocalDateTime.now());

        final var loggedIn = authenticationRepository.save(auth);
        LOG.info("login. customer successfully  logged in with email/pw  {}", loggedIn);
        return ResponseEntity.ok(loggedIn);
    }


    private String getToken(final String email) {
        final var now = new Date();
        final var expirationDate = new Date(now.getTime() + Long.parseLong(expirationTime) * 1000);

        final HashMap<String, Object> claims = new HashMap<>();

        return Jwts.builder()
                .setClaims(claims)
                .setSubject(email)
                .setIssuedAt(now)
                .setExpiration(expirationDate)
                .signWith(SignatureAlgorithm.HS256, secret)
                .compact();
    }

    @NonNull
    @RouterOperation(path = "/api/v1/authentication/logout",
            method = POST,
            operation = @Operation(operationId = "logout",
                    requestBody = @io.swagger.v3.oas.annotations.parameters.RequestBody(content = @Content(schema = @Schema(implementation = LogOutRequest.class))),
                    responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = AuthenticationEntry.class))),
                    })
    )
    @ResponseBody
    @PostMapping(value = "/api/v1/authentication/logout", produces = MediaType.APPLICATION_JSON_VALUE)
    public ResponseEntity<AuthenticationEntry> logout(@RequestBody final LogOutRequest request) throws WebClientResponseException {
        LOG.info("logout  for customerId {} ", request.getCustomerId());
        final AuthenticationEntry authenticationEntry = authenticationRepository.findByCustomerIdAndJwtNotNull(request.getCustomerId());
        if (nonNull(authenticationEntry)) {
            LOG.info("logout. auth entry found  {}", authenticationEntry);
            authenticationEntry.setJwt(null);
            authenticationEntry.setLoggedOut(LocalDateTime.now());
            final var loggedOut = authenticationRepository.save(authenticationEntry);
            LOG.info("logout  loggedOUt  {} ", loggedOut);
            return ResponseEntity.ok(loggedOut);
        }
        return ResponseEntity.notFound().build();
    }
}
