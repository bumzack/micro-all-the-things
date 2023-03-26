package at.bumzack.tsvfilereader.stuff;

import org.springframework.security.authentication.ReactiveAuthenticationManager;
import org.springframework.security.core.Authentication;
import org.springframework.stereotype.Component;
import reactor.core.publisher.Mono;
import reactor.util.Logger;

@Component
public class AuthenticationManager implements ReactiveAuthenticationManager {

    private static final Logger LOGGER = reactor.util.Loggers.getLogger(AuthenticationManager.class);

    @Override
    @SuppressWarnings("unchecked")
    public Mono<Authentication> authenticate(Authentication authentication) {
        // dont care
        return Mono.empty();
    }
}
