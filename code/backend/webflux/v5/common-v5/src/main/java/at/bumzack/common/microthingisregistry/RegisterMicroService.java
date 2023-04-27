package at.bumzack.common.microthingisregistry;

import org.apache.commons.lang3.StringUtils;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.CommandLineRunner;
import org.springframework.context.annotation.Bean;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.stereotype.Service;
import org.springframework.web.reactive.function.BodyInserters;
import org.springframework.web.reactive.function.client.WebClient;
import reactor.core.publisher.Mono;
import reactor.util.Logger;
import reactor.util.Loggers;

import java.nio.file.Path;
import java.nio.file.Paths;

import static at.bumzack.common.microthingisregistry.MicrothingisRegistryConst.*;


@Service
public class RegisterMicroService {
    private static final Logger LOG = Loggers.getLogger(RegisterMicroService.class);

    @Value("${server.port}")
    private String serverPort;

    @Value("${server.host}")
    private String serverHost;

    @Value("${server.openapipath}")
    private String openApiPath;

    @Value("${micrsoservice.id}")
    private String micrsoserviceId;

    @Value("${publish_as_frontend_package}")
    private boolean publishAsFrontendPackage;

    @Value("${apiclient.prefix}")
    private String apiClientPrefix;

    @Value("${technology}")
    private String technology;

    @Value("${local.repo.path}")
    private String localRepoPath;

    @Value("${apiclient.package}")
    private String apiClientPackage;

    @Bean
    public CommandLineRunner readFromApiServer() {
        LOG.info("readFromApiServer");
        final Path currentRelativePath = Paths.get("");
        final String blupp = currentRelativePath.toAbsolutePath().toString();
        LOG.info("Current absolute path is: " + blupp);

        return new CommandLineRunner() {
            @Override
            public void run(String... args) throws Exception {
                final var url = URL_BACKEND_FIND_BY_NAME + "/" + micrsoserviceId;
                LOG.info("calling URL {}", url);
                final WebClient webClient = WebClient.create(url);
                webClient
                        .get()
                        .accept(MediaType.APPLICATION_JSON)
                        .exchangeToMono(clientResponse -> {
                            if (clientResponse.statusCode().equals(HttpStatus.NOT_FOUND)) {
                                LOG.info("registering new backend");
                                return registerMicroService();
                            } else {
                                return clientResponse.bodyToMono(String.class);
                            }
                        })
                        //.flatMap(be -> updateOpenApiClient())
                        .subscribe(s -> {
                            LOG.info("run ended with {}", s);
                        });
            }

            private Mono<String> updateOpenApiClient() {
                return Mono.just("updating sometime later");
            }

            private Mono<String> registerMicroService() {
                LOG.info("registerMicroService");
                final var url = URL_TECHNOLOGY_BY_NAME + "/" + technology;
                LOG.info("requesting URL {}}", url);
                final WebClient webClient = WebClient.create(url);
                return webClient
                        .get()
                        .accept(MediaType.APPLICATION_JSON)
                        .exchange()
                        .flatMap(response -> response.bodyToMono(Technology.class))
                        .flatMap(t -> {
                            LOG.info("webflux found?   tech.id = " + t.getId() + ", tech.name = '" + " " + t.getName() + "'");
                            final var be = getNewBackend(t);
                            // register microservice
                            final WebClient webClient2 = WebClient.create(URL_BACKEND_POST);
                            LOG.info("new backend " + be);
                            LOG.info("new backend post URL " + URL_BACKEND_POST);
                            return webClient2
                                    .post()
                                    .contentType(MediaType.APPLICATION_JSON)
                                    .body(BodyInserters.fromValue(be))
                                    .retrieve()
                                    .bodyToMono(String.class)
                                    .doOnError(e -> LOG.error("error calling microservicesthings  {}", e.getMessage()));
                        })
                        .doOnError(e -> {
                            LOG.error("an error occurred '{}'", e.getMessage(), e);
                        });
            }

            private NewBackend getNewBackend(final Technology technology) {
                final var be = new NewBackend();
                be.setLocalRepoPath(localRepoPath);
                be.setMicroserviceId(micrsoserviceId);
                final var openapi = StringUtils.join("http://", serverHost, ":", serverPort, "/", openApiPath);
                LOG.info("made openapimatch " + openapi);
                be.setOpenapiUrl(openapi);
                final var serviceUri = StringUtils.join("http://", serverHost, ":", serverPort);
                be.setServiceUri(serviceUri);
                be.setPublishAsFrontendPackage(publishAsFrontendPackage);
                be.setTechnologyId(technology.getId());
                be.setApiClientPrefix(apiClientPrefix);
                be.setApiClientPackage(apiClientPackage);
                return be;
            }
        };
    }
}
