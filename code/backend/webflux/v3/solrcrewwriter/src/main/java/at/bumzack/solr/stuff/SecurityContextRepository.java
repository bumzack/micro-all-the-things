package at.bumzack.solr.stuff;


import org.springframework.http.HttpHeaders;
import org.springframework.security.authentication.ReactiveAuthenticationManager;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.context.SecurityContext;
import org.springframework.security.core.context.SecurityContextImpl;
import org.springframework.security.web.server.context.ServerSecurityContextRepository;
import org.springframework.stereotype.Component;
import org.springframework.web.server.ServerWebExchange;
import reactor.core.publisher.Mono;
import reactor.util.Logger;

@Component
public class SecurityContextRepository implements ServerSecurityContextRepository {

    private static final Logger LOGGER = reactor.util.Loggers.getLogger(SecurityContextRepository.class);

    private static final String TOKEN_PREFIX = "Bearer ";

    private final ReactiveAuthenticationManager authenticationManager;

    public SecurityContextRepository(final ReactiveAuthenticationManager authenticationManager) {
        this.authenticationManager = authenticationManager;
    }


    public Mono save(final ServerWebExchange swe, final SecurityContext sc) {
        throw new UnsupportedOperationException("Not supported yet.");
    }

    @Override
    public Mono load(final ServerWebExchange swe) {
        final var request = swe.getRequest();
        final var authHeader = request
                .getHeaders()
                .getFirst(HttpHeaders.AUTHORIZATION);

        String authToken = null;

        if (authHeader != null && authHeader.startsWith(TOKEN_PREFIX)) {
            authToken = authHeader.replace(TOKEN_PREFIX, "");
        }
        if (authToken != null) {
            final UsernamePasswordAuthenticationToken auth = new UsernamePasswordAuthenticationToken(authToken, authToken);
            return this.authenticationManager
                    .authenticate(auth)
                    .map(SecurityContextImpl::new);
        } else {
            return Mono.empty();
        }
    }
}
