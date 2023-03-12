//package at.bumzack.pimpricedbreader;
//
//import org.springframework.context.annotation.Configuration;
//import org.springframework.web.server.ServerWebExchange;
//import org.springframework.web.server.WebFilter;
//import org.springframework.web.server.WebFilterChain;
//import reactor.core.publisher.Mono;
//import reactor.util.Logger;
//import reactor.util.Loggers;
//
//import java.nio.charset.Charset;
//
//@Configuration
//public class RequestLoggingFilter implements WebFilter {
//    private static final Logger LOGGER = Loggers.getLogger(RequestLoggingFilter.class);
//
//    @Override
//    public Mono<Void> filter(final ServerWebExchange serverWebExchange, final WebFilterChain webFilterChain) {
//        LOGGER.info(String.format("request path: '%s', method: '%s', hostname: '%s', uri.host: '%s', uri.path: '%s'     ",
//                serverWebExchange.getRequest().getPath(),
//                serverWebExchange.getRequest().getMethod().name(),
//                serverWebExchange.getRequest().getRemoteAddress().getHostName(),
//                serverWebExchange.getRequest().getURI().getHost(),
//                serverWebExchange.getRequest().getURI().getPath()));
//
//        serverWebExchange.getRequest().getHeaders().forEach((k, v) -> {
//            v.forEach(value -> LOGGER.info(String.format("request header: '%s', value: '%s' ", k, v)));
//        });
//        serverWebExchange.getRequest().getBody()
//                .map(a -> a.toString(Charset.defaultCharset()))
//                .subscribe(k -> {
//                    LOGGER.info("body {}", k);
//                });
//
//        return webFilterChain.filter(serverWebExchange);
//    }
//}