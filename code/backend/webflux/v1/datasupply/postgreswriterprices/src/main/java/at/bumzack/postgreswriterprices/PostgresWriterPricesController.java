package at.bumzack.postgreswriterprices;


import at.bumzack.dto.Image;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.ArraySchema;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.parameters.RequestBody;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.context.annotation.Bean;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.reactive.function.BodyInserters;
import org.springframework.web.reactive.function.client.WebClientResponseException;
import org.springframework.web.reactive.function.server.RequestPredicates;
import org.springframework.web.reactive.function.server.RouterFunction;
import org.springframework.web.reactive.function.server.ServerRequest;
import org.springframework.web.reactive.function.server.ServerResponse;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;
import reactor.util.Logger;
import reactor.util.annotation.NonNull;

import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("SearchArticleController")
@CrossOrigin
public class PostgresWriterPricesController {
    private static final Logger LOG = reactor.util.Loggers.getLogger(PostgresWriterPricesController.class);

    @NonNull
    public Mono<ServerResponse> selectImageDesktop(final ServerRequest request) throws WebClientResponseException {
        LOG.info("ImageSelectorController got a request ");
//        request.bodyToMono(String.class)
//                .subscribe(i -> {
//                    LOG.info("images list {}", i);
//                });

//        request.bodyToFlux(Image.class)
//                .sort(this::imageSizesComparator)
//                .        last()
//                .log("imageSelector")
//                .doOnNext(i -> {
//                    LOG.info("found image found code {}, w x h:  {}x{}", i.getCode(), i.getWidth(), i.getHeight());
//                })
//                .doOnError((e) -> {
//                    LOG.error("error: {} ", e);
//                })
//                .flatMap(image -> ServerResponse.ok().body(BodyInserters.fromValue(image)))
//                .subscribe(i -> {
//                    LOG.info("server response with 1 Image hopefully statuscode {},    i.tostring {}", i.statusCode(), i.toString());
//                });


        final Flux<Image> sort = request
                .bodyToFlux(Image.class)
                .sort(this::imageSizesComparator);
        final Mono<Image> last = sort
                .last()
                .log("imageSelector")
                .doOnNext(i -> {
                    LOG.info("found image found code {}, w x h:  {}x{}", i.getCode(), i.getWidth(), i.getHeight());
                })
                .doOnError((e) -> {
                    LOG.error("error: {} ", e);
                });
        final Mono<ServerResponse> serverResponseMono = last
                .flatMap(image -> {
                    LOG.info("returning image   {}", image);
                    return ServerResponse.ok().body(BodyInserters.fromValue(image));
                })
                .log("last in controller");
        return serverResponseMono;

        // return dummyResponse();
    }

    private static Mono<ServerResponse> dummyResponse() {
        final var img = new Image();
        img.setChannel("bla");
        img.setUrl("bla");
        img.setWidth("sdfsdf");
        img.setHeight("bla");
        img.setMime("bla");
        img.setMediaContainerQualifier("qualifier");
        img.setCode("code");
        img.setId("id");

        return ServerResponse.ok().body(BodyInserters.fromValue(img));
    }

    @NonNull
    public Mono<ServerResponse> selectImageMobile(final ServerRequest request) throws WebClientResponseException {
        return request.bodyToFlux(Image.class)
                .sort(this::imageSizesComparator)
                .last()
                .flatMap(image -> ServerResponse.ok().body(BodyInserters.fromValue(image)))
                .switchIfEmpty(ServerResponse.notFound().build());
    }

    private int imageSizesComparator(final Image i1, final Image i2) {
        final Integer cntPixels1 = Integer.parseInt(i1.getWidth()) * Integer.parseInt(i1.getHeight());
        final Integer cntPixels2 = Integer.parseInt(i2.getWidth()) * Integer.parseInt(i2.getHeight());
        return cntPixels1.compareTo(cntPixels2);
    }

    @RouterOperations({
            @RouterOperation(path = "/image/desktop",
                    method = POST,
                    operation = @Operation(operationId = "selectDesktop",
                            requestBody = @RequestBody(content = @Content(array = @ArraySchema(schema = @Schema(implementation = Image.class)))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = Image.class))),
                            })
            ),
            @RouterOperation(path = "/image/desktop",
                    method = POST,
                    operation = @Operation(operationId = "selectMobile",
                            requestBody = @RequestBody(content = @Content(array = @ArraySchema(schema = @Schema(implementation = Image.class)))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = Image.class))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrRoutes() {
        return route()
                .nest(RequestPredicates.path("/image/"),
                        builder -> builder
                                .POST("desktop", this::selectImageDesktop)
                                //  .POST("desktop", this::selectImageDesktop2)
                                .POST("mobile", this::selectImageMobile)
                                .build())
                .build();
    }
}
