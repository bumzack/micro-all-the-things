package at.bumzack.solrsearch;

import at.bumzack.microthingisregistry.NewBackend;
import at.bumzack.microthingisregistry.Technology;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.CommandLineRunner;
import org.springframework.context.annotation.Bean;
import org.springframework.http.MediaType;
import org.springframework.stereotype.Service;
import org.springframework.web.reactive.function.BodyInserters;
import org.springframework.web.reactive.function.client.WebClient;
import reactor.core.publisher.Mono;
import reactor.util.Logger;

import static at.bumzack.microthingisregistry.MicrothingisRegistryConst.URL_BACKEND_POST;
import static at.bumzack.microthingisregistry.MicrothingisRegistryConst.URL_TECHNOLOGY_BY_NAME;

@Service
public class SearchArticleRegister {
    private static final Logger LOG = reactor.util.Loggers.getLogger(SearchArticleRegister.class);


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

    @Bean
    public CommandLineRunner readFromApiServer() {
        System.out.println("yoyoyoooyooo");
        return new CommandLineRunner() {
            @Override
            public void run(String... args) throws Exception {
                final var url = URL_TECHNOLOGY_BY_NAME + "/" + "webflux";
                final WebClient webClient = WebClient.create(url);
                final Mono<Technology> technologyMono = webClient
                        .get()
                        .accept(MediaType.APPLICATION_STREAM_JSON)
                        .exchange()
                        .flatMap(response -> response.bodyToMono(Technology.class));

                technologyMono.subscribe(t -> {
                    System.out.println("webflux found?   tech.id = " + t.getId() + ", tech.name = '" + " " + t.getName() + "'");

                    final var be = new NewBackend();
                    be.setLocalRepoPath(openApiPath);
                    be.setMicroserviceId(micrsoserviceId);
                    be.setOpenapiUrl(openApiPath);
                    be.setServiceUri("TODO repo path");
                    be.setPublishAsFrontendPackage(publishAsFrontendPackage);
                    be.setTechnologyId(t.getId());

                    // register microservice

                    final WebClient webClient2 = WebClient.create(URL_BACKEND_POST);
                    System.out.println("new backend " + be);
                    final var res = webClient2
                            .post()
                            .contentType(MediaType.APPLICATION_JSON)
                            .body(BodyInserters.fromValue(be))
                            .retrieve()
                            .bodyToMono(String.class)
                            .doOnError(e -> LOG.error("error calling microservicesthings  {}", e.getMessage()));

                    res.subscribe(r -> {
                        System.out.println("got a result from microservicesthings " + r);
                    });
                });
                // do something with the result?
            }
        };
    }
}
