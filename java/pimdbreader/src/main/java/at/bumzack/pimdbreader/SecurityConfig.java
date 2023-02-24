//package at.bumzack.solrreader;
//
//
//import org.springframework.context.annotation.Bean;
//import org.springframework.http.HttpMethod;
//import org.springframework.security.config.annotation.method.configuration.EnableReactiveMethodSecurity;
//import org.springframework.security.config.annotation.web.reactive.EnableWebFluxSecurity;
//import org.springframework.security.config.web.server.ServerHttpSecurity;
//import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;
//import org.springframework.security.web.server.SecurityWebFilterChain;
//import org.springframework.web.cors.CorsConfiguration;
//import org.springframework.web.cors.reactive.CorsConfigurationSource;
//import org.springframework.web.cors.reactive.UrlBasedCorsConfigurationSource;
//import reactor.util.Logger;
//
//import java.util.Arrays;
//
//@EnableReactiveMethodSecurity
//@EnableWebFluxSecurity
//public class SecurityConfig {
//    private static final Logger LOG = reactor.util.Loggers.getLogger(SecurityConfig.class);
//
//    private static final String LOCALHOST = "http://localhost:3000";
//
//    // private final SecurityContextRepository securityContextRepository;
//
////    public SecurityConfig( final SecurityContextRepository securityContextRepository) {
////        this.securityContextRepository = securityContextRepository;
////    }
//
//    @Bean
//    CorsConfigurationSource corsConfiguration() {
//        final CorsConfiguration corsConfig = new CorsConfiguration();
//        corsConfig.applyPermitDefaultValues();
//        corsConfig.addAllowedMethod(HttpMethod.PUT);
//        corsConfig.addAllowedMethod(HttpMethod.DELETE);
//        corsConfig.addAllowedMethod(HttpMethod.OPTIONS);
//        corsConfig.addAllowedMethod(HttpMethod.GET);
//        corsConfig.addAllowedMethod(HttpMethod.POST);
//        corsConfig.setAllowedOrigins(Arrays.asList(LOCALHOST));
//
//        final UrlBasedCorsConfigurationSource source =
//                new UrlBasedCorsConfigurationSource();
//        source.registerCorsConfiguration("/**", corsConfig);
//        return source;
//    }
//
//    @Bean
//    SecurityWebFilterChain springWebFilterChain(final ServerHttpSecurity http) {
//        LOG.info("hallo");
//        final String[] patternsPublic = new String[]{"/api" + "/**", "/v3/**", "/v3/api-docs/", "swagger-ui.html", "/webjars/swagger-ui/**"};
//
//        return http.cors()
//                .and()
//                .exceptionHandling()
////                .authenticationEntryPoint((swe, e) -> Mono.fromRunnable(() -> {
////                    swe.getResponse().setStatusCode(HttpStatus.UNAUTHORIZED);
////                })).accessDeniedHandler((swe, e) -> Mono.fromRunnable(() -> {
////                    swe.getResponse().setStatusCode(HttpStatus.FORBIDDEN);
////                }))
//                .and()
//
//                .csrf().disable()
//                .formLogin().disable()
//                .httpBasic().disable()
////                .httpBasic().authenticationEntryPoint(new HttpStatusServerEntryPoint(HttpStatus.UNAUTHORIZED))
//                //             .securityContextRepository(securityContextRepository)
//                .authorizeExchange()
//                .pathMatchers(HttpMethod.OPTIONS).permitAll()
//                .pathMatchers(patternsPublic).permitAll()
//                .anyExchange().permitAll()
//                .and()
//                .build();
//    }
//
//    @Bean
//    public BCryptPasswordEncoder passwordEncoder() {
//        return new BCryptPasswordEncoder();
//    }
//}
