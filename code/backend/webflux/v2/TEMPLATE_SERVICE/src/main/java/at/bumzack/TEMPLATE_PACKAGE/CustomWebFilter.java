package at.bumzack.apigateway;

import org.springframework.stereotype.Component;
import org.springframework.web.server.ServerWebExchange;
import org.springframework.web.server.WebFilter;
import org.springframework.web.server.WebFilterChain;
import reactor.core.publisher.Mono;
import reactor.util.Logger;

@Component
public class CustomWebFilter implements WebFilter {
    private static final Logger LOG = reactor.util.Loggers.getLogger(CustomWebFilter.class);

    @Override
    public Mono<Void> filter(final ServerWebExchange serverWebExchange,
                             final WebFilterChain webFilterChain) {
        LOG.info("yoyoyoyoyyooooo");
        final BodyCaptureExchange bodyCaptureExchange = new BodyCaptureExchange(serverWebExchange);
        // LOG.info("before: Body request {}", bodyCaptureExchange.getRequest().getFullBody());

        return webFilterChain.filter(bodyCaptureExchange).doOnSuccess((se) -> {
                    LOG.info("success: Body request {}", bodyCaptureExchange.getRequest().getFullBody());
                    LOG.info("success: Body response {}", bodyCaptureExchange.getResponse().getFullBody());
                })
                .onErrorContinue((ex, obj) -> {
                    LOG.error("error in request {}", ex.getMessage());
                    LOG.error("error", ex);
                })
                .doOnError((se) -> {
                    LOG.info("doOnError: Body request {}", se.getMessage());
                    LOG.info("doOnError: Body request {}", se.getCause());

                    LOG.info("doOnError: Body request {}", bodyCaptureExchange.getRequest().getFullBody());
                    LOG.info("doOnError: Body response {}", bodyCaptureExchange.getResponse().getFullBody());
                })
                .doOnCancel(() -> {
                    LOG.info("cancel: Body request {}", bodyCaptureExchange.getRequest().getFullBody());
                    LOG.info("cancel: Body response {}", bodyCaptureExchange.getResponse().getFullBody());
                });
    }
}
