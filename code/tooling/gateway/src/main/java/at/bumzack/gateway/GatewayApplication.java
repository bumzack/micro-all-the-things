package at.bumzack.gateway;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.cloud.gateway.route.RouteLocator;
import org.springframework.cloud.gateway.route.builder.RouteLocatorBuilder;
import org.springframework.context.annotation.Bean;

@SpringBootApplication
public class GatewayApplication {

	public static void main(String[] args) {
		SpringApplication.run(GatewayApplication.class, args);
	}

	@Bean
	public RouteLocator myRoutes(final RouteLocatorBuilder builder) {

		// TODO: https://github.com/spring-cloud/spring-cloud-gateway/issues/276
		return builder.routes()
				.route(p -> p
						.path("/api/person")
						.filters(f -> f.addRequestHeader("Hello", "World"))
						.uri( "http://localhost:3040/api/person"))
				.build();
	}
}
