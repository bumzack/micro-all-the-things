package at.bumzack.foryouandyourfakewebshop.authenticationservice;


import at.bumzack.customerservice.ApiClient;
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
import io.swagger.v3.oas.annotations.parameters.RequestBody;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.http.HttpStatus;
import org.springframework.lang.NonNull;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.reactive.function.BodyInserters;
import org.springframework.web.reactive.function.client.WebClientResponseException;
import org.springframework.web.reactive.function.server.RequestPredicates;
import org.springframework.web.reactive.function.server.RouterFunction;
import org.springframework.web.reactive.function.server.ServerRequest;
import org.springframework.web.reactive.function.server.ServerResponse;
import reactor.core.publisher.Mono;
import reactor.util.Logger;
import reactor.util.Loggers;

import java.time.LocalDateTime;
import java.util.Date;
import java.util.HashMap;

import static java.util.Objects.nonNull;
import static org.springframework.web.bind.annotation.RequestMethod.GET;
import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;


@Controller("AuthenticationServiceController")
@CrossOrigin
public class AuthenticationServiceController {

    private static final Logger LOG = Loggers.getLogger(AuthenticationServiceController.class);
    private final AuthenticationRepository authenticationRepository;

    @Value("${jwt.secret}")
    private String secret;

    @Value("${jwt.expiration}")
    private String expirationTime;

    public AuthenticationServiceController(final AuthenticationRepository authenticationRepository) {
        this.authenticationRepository = authenticationRepository;
    }

    @NonNull
    public Mono<ServerResponse> loggedIn(final ServerRequest request) throws WebClientResponseException {
        final var customerId = Long.parseLong(request.pathVariable("customerid"));
        return authenticationRepository.findByCustomerIdAndJwtNotNull(customerId)
                .flatMap(authenticationEntry -> {
                    LOG.info("found a authentication_entry with customerid {}.   {} ", customerId, authenticationEntry);
                    return ServerResponse.ok().body(BodyInserters.fromValue(authenticationEntry));
                })
                .switchIfEmpty(ServerResponse.notFound().build());
    }

    @NonNull
    public Mono<ServerResponse> login(final ServerRequest request) throws WebClientResponseException {
        final var apiClient = new ApiClient();
        final var customerServiceApi = new DefaultApi();

        customerServiceApi.setApiClient(apiClient);
        final Mono<ServerResponse> res = request.bodyToMono(LogInRequest.class)
                .flatMap(req -> customerServiceApi.getCustomer(req.email()).log("getCustomer")
                        .onErrorResume(WebClientResponseException.NotFound.class, (e) -> {
                            LOG.info("NotFound Exception in WebClient!. customer not found with email  " + req.email());
                            return Mono.empty();
                        })
                        .log("after onErrorResume")
                        .flatMap(customer -> {
                            LOG.info("found a customer in DB  id  {}", customer.getId());
                            final Mono<ServerResponse> serverResponseMono = authenticationRepository.findByCustomerId(customer.getId())
                                    .log("authenticationRepository.findByCustomerId()")
                                    .flatMap(authenticationEntry -> {
                                        LOG.info("returning an  authenticationEntry   for customer id {},  authENtry {} " + customer.getId(), authenticationEntry);
                                        //  authenticationEntry.setJwt("hahahahaa");
                                        return ServerResponse.status(HttpStatus.OK).body(BodyInserters.fromValue(authenticationEntry));
                                    })
                                    .log("serverrepsonse 200 because there exists already an authenticationEntry")
                                    .switchIfEmpty(noAuthENtryFound(customer, req))
                                    .log("after switchIfEmpty(noAuthENtryFound(customer, req))");
//                                    .flatMap(authenticationEntry -> {
//                                        LOG.info("customer already has an  authenticationEntry   {}", authenticationEntry);
//                                        return checkAuth(authenticationEntry);
//                                    });
                            // return ServerResponse.status(HttpStatus.OK).body(BodyInserters.fromValue("customer found " + customer));
                            return serverResponseMono;
                        }))
                .switchIfEmpty(stillNothingFound());
        return res;
    }

    private Mono<ServerResponse> stillNothingFound() {
        LOG.info("still nothing found");
        return ServerResponse.status(HttpStatus.NOT_FOUND).body(BodyInserters.fromValue("customerxxxx NOT found with email"));
    }

    private Mono<ServerResponse> noAuthENtryFound(final Customer customer, final LogInRequest req) {
        LOG.info(" no auth entry in DB found. so we will try and login the user");
        if (req.email().equals(customer.getEmail()) && req.password().equals(customer.getPassword())) {
            final var jwt = getToken(customer.getEmail());
            final var entry = new AuthenticationEntry();
            entry.setLoggedIn(LocalDateTime.now());
            entry.setJwt(jwt);
            //  entry.setJwt("new entry");
            entry.setCustomerId(customer.getId());
            LOG.info("customer email and password are matching. logging user in {}", entry);
            return authenticationRepository.save(entry)
                    .flatMap(e -> {
                        LOG.info("saved new authEntry for customer_id {},   entry {}", customer.getId(), e);
                        return ServerResponse.status(HttpStatus.OK).body(BodyInserters.fromValue(e));
                    }).log("authenticationRepository.save 1")
                    .switchIfEmpty(retEmptyAuth(customer)).log("switchIfEmpty(retEmptyAuth(customer)) 1");
        }
        return Mono.empty();
    }

//    private Mono<ServerResponse> createAuthEntry(final LogInRequest req, final Customer customer) {
//        LOG.info("customer has no auth entry  -> try logging the user in and create auth entry");
//        if (req.email().equals(customer.getEmail()) && req.password().equals(customer.getPassword())) {
//            final var jwt = getToken(customer.getEmail());
//            final var entry = new AuthenticationEntry();
//            entry.setLoggedIn(LocalDateTime.now());
//            entry.setJwt(jwt);
//            entry.setCustomerId(customer.getId());
//            LOG.info("customer email and password are matching. logging user in {}", entry);
//            return authenticationRepository.save(entry)
//                    .flatMap(e -> {
//                        LOG.info("saved new authEntry for customer_id {},   entry {}", customer.getId(), e);
//                        return authenticationRepository.findByCustomerId(customer.getId());
//                    })
//                    .switchIfEmpty(retEmptyAuth(customer));
//
//        }
//        return Mono.empty();
//    }

    private Mono<ServerResponse> retEmptyAuth(final Customer customer) {
        LOG.info("save Entry crashed for customer id {}", customer.getId());
        return ServerResponse.status(HttpStatus.INTERNAL_SERVER_ERROR).body(BodyInserters.fromValue("could not save auth entry"));
    }

    private Mono<AuthenticationEntry> checkAuth(final AuthenticationEntry authenticationEntry) {
        LOG.info("check if jwt is set. i   jwt {}", authenticationEntry.getJwt());
        if (nonNull(authenticationEntry.getJwt())) {
            LOG.info("customer is loggedin. jwt is {}", authenticationEntry.getJwt());
            return Mono.just(authenticationEntry);
        }
        LOG.info("check if jwt is set. no it is not -> return 404. user is not logged in {}", authenticationEntry.getJwt());
        return Mono.empty();
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
    public Mono<ServerResponse> logout(final ServerRequest request) throws WebClientResponseException {
        return request.bodyToMono(LogOutRequest.class)
                .flatMap(req -> authenticationRepository.findByCustomerIdAndJwtNotNull(req.customerId())
                        .flatMap(authenticationEntry -> {
                            authenticationEntry.setJwt(null);
                            authenticationEntry.setLoggedOut(LocalDateTime.now());
                            return authenticationRepository.save(authenticationEntry)
                                    .flatMap(entry -> ServerResponse.ok().body(BodyInserters.fromValue(entry)))
                                    .switchIfEmpty(ServerResponse.status(HttpStatus.INTERNAL_SERVER_ERROR).body(BodyInserters.fromValue("cant save logged out authentication entry")));
                        })
                        .switchIfEmpty(ServerResponse.notFound().build()));
    }

    @RouterOperations({
            @RouterOperation(path = "/api/v1/authenticated/{customerId}",
                    method = GET,
                    operation = @Operation(operationId = "loggedin",
                            parameters = {@Parameter(in = ParameterIn.PATH, name = "customerId", description = "Customer ID", schema = @Schema(implementation = Long.class))
                            },
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = AuthenticationEntry.class))),
                            })
            ),
            @RouterOperation(path = "/api/v1/authentication/login",
                    method = POST,
                    operation = @Operation(operationId = "login",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = LogInRequest.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = AuthenticationEntry.class))),
                            })
            ),
            @RouterOperation(path = "/api/v1/authentication/logout",
                    method = POST,
                    operation = @Operation(operationId = "logout",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = LogOutRequest.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = AuthenticationEntry.class))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> authenticationRoutes() {
        return route()
                .nest(RequestPredicates.path("/api/v1"),
                        builder -> builder
                                .GET("/authenticated/{customerid}", this::loggedIn)
                                .POST("/authentication/login", this::login)
                                .POST("/authentication/logout", this::logout)
                                .build())
                .build();
    }

}
