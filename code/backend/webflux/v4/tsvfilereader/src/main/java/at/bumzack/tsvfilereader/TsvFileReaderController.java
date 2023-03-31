package at.bumzack.tsvfilereader;


import at.bumzack.common.dto.TsvLine;
import at.bumzack.common.dto.TsvLines;
import at.bumzack.common.tsv.TsvEnum;
import at.bumzack.common.tsv.TsvFileImportRequest;
import at.bumzack.common.webflux.WebClientFactory;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.parameters.RequestBody;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.apache.commons.lang3.StringUtils;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.context.annotation.Bean;
import org.springframework.core.env.Environment;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.reactive.function.BodyInserters;
import org.springframework.web.reactive.function.client.WebClient;
import org.springframework.web.reactive.function.client.WebClientResponseException;
import org.springframework.web.reactive.function.server.RequestPredicates;
import org.springframework.web.reactive.function.server.RouterFunction;
import org.springframework.web.reactive.function.server.ServerRequest;
import org.springframework.web.reactive.function.server.ServerResponse;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;
import reactor.core.scheduler.Schedulers;
import reactor.util.Logger;
import reactor.util.Loggers;
import reactor.util.annotation.NonNull;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.time.Duration;
import java.util.stream.Stream;

import static at.bumzack.common.tsv.TsvUtils.SPLITTER_TSV;
import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("TsvFileReaderController")
@CrossOrigin
public class TsvFileReaderController {
    private static final Logger LOG = Loggers.getLogger(TsvFileReaderController.class);

    private final Environment env;

    public TsvFileReaderController(final Environment env) {
        this.env = env;
    }

    @NonNull
    public Mono<ServerResponse> status(final ServerRequest request) {
        return ServerResponse.ok().body(BodyInserters.fromValue("all good here"));
    }

    @NonNull
    public Mono<ServerResponse> readFile(final ServerRequest request) throws WebClientResponseException {
        return request.bodyToMono(TsvFileImportRequest.class)
                .publishOn(Schedulers.boundedElastic())
                .flatMap(req -> {
                    final var type = req.getTsvType();
                    final var webClient = WebClientFactory.getClient();
                    final var url = getSolrWriterServiceUrl(type);
                    final var tsvFilename = getTsvFilename(type);
                    final var path = Paths.get(tsvFilename);
                    final var start = req.getStart();
                    final var pageSize = req.getPageSize();
                    LOG.info("processing request {}", req);

                    final Stream<String> lines;
                    try {
                        lines = Files.lines(path);
                    } catch (final IOException e) {
                        LOG.error("error reading file " + tsvFilename + ". ex: ", e.getMessage());
                        LOG.error(e.toString());
                        final var msg = "TSV reader error " + e.getMessage() + " processing file  " + tsvFilename + ". target URL " + url;
                        return ServerResponse.status(HttpStatus.INTERNAL_SERVER_ERROR).body(BodyInserters.fromValue(msg));
                    }

                    return getServerResponseMono(webClient, url, start, pageSize, lines);
                });
    }

    private Mono<ServerResponse> getServerResponseMono(WebClient webClient, String url, int start, int pageSize, Stream<String> lines) {
        return execRequests(webClient, url, lines, start, pageSize)
                .doOnError(e -> {
                    LOG.error("an error occurred while requesting a POST to ratings {}", e);
                })
                .flatMap(c -> {
                    final var msg = "TSV reader processed " + c + " items. items sent to " + url;
                    return ServerResponse.ok().body(BodyInserters.fromValue(msg));
                });
    }

    private Mono<Long> execRequests(final WebClient webClient, final String url, final Stream<String> lines, final int start, final int pageSize) {
        return Flux.fromStream(lines)
                .skip(start)
                .buffer(pageSize)
                .delayElements(Duration.ofMillis(10000))
                .collectList()
                .flatMapMany(l -> {
                    final Stream<Mono<String>> monoStream = l.stream().map(list -> {
                        final var tmp = list.stream()
                                .map(this::mapToTsvLine)
                                .toList();
                        final var tsvLines = new TsvLines();
                        tsvLines.setLines(tmp);
                        return execRequest(webClient, url, tsvLines);
                    });
                    return Flux.fromStream(monoStream);
                })
                .delayElements(Duration.ofMillis(1000L))
                .doOnNext(e -> LOG.info("processing next batch "))
                .count();
    }

    private Mono<String> execRequest(final WebClient webClient, final String url, final TsvLines tsvLine) {
        return webClient.post()
                .uri(url)
                .contentType(MediaType.APPLICATION_JSON)
                .accept()
                .body(BodyInserters.fromValue(tsvLine))
                .retrieve()
                .bodyToMono(String.class)
                //      .doOnNext(res -> LOG.info("tsv reader POST  to {} returned success  {}", url, res))
                .doOnError(e -> LOG.error("tsv reader POST to {} returned an error  '{}'", url, e.getMessage()));
    }

    private TsvLine mapToTsvLine(final String l) {
        final var entries = SPLITTER_TSV.splitToList(l);
        final TsvLine tsvLine = new TsvLine();
        tsvLine.setOriginal(l);
        tsvLine.setEntries(entries);
        return tsvLine;
    }

    private String getTsvFilename(final TsvEnum type) {
        final var templateFilename = "datasource.TYPE.filename";

        final var folderKey = "datasource.folder";
        final var filenameKey = templateFilename.replace("TYPE", type.toString());
        final var folder = env.getProperty(folderKey);
        final var filename = env.getProperty(filenameKey);

        return StringUtils.join(folder, "/", filename);
    }

    private String getSolrWriterServiceUrl(final TsvEnum type) {
        final var templateHost = "microservice.TYPE.host";
        final var templatePort = "microservice.TYPE.port";
        final var templateURL = "microservice.TYPE.url";

        final var hostKey = templateHost.replace("TYPE", type.toString());
        final var portKey = templatePort.replace("TYPE", type.toString());
        final var urlKey = templateURL.replace("TYPE", type.toString());
        final var host = env.getProperty(hostKey, String.class);
        final var port = env.getProperty(portKey, String.class);
        final var url = env.getProperty(urlKey, String.class);

        return StringUtils.join(host, ":", port, url);
    }

    @RouterOperations({
            @RouterOperation(path = "v2/api/tsv/read",
                    method = POST,
                    operation = @Operation(operationId = "readFile",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = TsvFileImportRequest.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = TsvLines.class))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrRoutes() {
        return route()
                .nest(RequestPredicates.path("v2/api/tsv/"),
                        builder -> builder
                                .POST("read", this::readFile)
                                .GET("status", this::status)
                                .build())
                .build();
    }
}
