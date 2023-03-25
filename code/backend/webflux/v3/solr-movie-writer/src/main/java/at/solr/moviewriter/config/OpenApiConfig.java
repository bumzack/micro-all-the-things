package at.solr.moviewriter.config;


import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.servers.Server;
import io.swagger.v3.oas.models.OpenAPI;
import org.springframework.context.annotation.Bean;
import org.springframework.stereotype.Service;

@OpenAPIDefinition(
        info = @Info(title = "Solr Movie Writer MicroService", version = "1.0", description = "Solr Movie Writer MicroService"),
        servers = {
                @Server(
                        description = "ASolr Movie Writer MicroService",
                        url = "http://localhost:8300")
        })
@Service
public class OpenApiConfig {

    public static final String SECURTIY_SCHEME_API = "SECURED_API";

    public @Bean OpenAPI searchArticleAPI() {
        return new OpenAPI()
//                .components(new Components().
//                        addSecuritySchemes("bearerAuth",
//                                new SecurityScheme()
//                                        .type(SecurityScheme.Type.APIKEY)
//                                        .description("JWT authorization")
//                                        .scheme("bearer")
//                                        .bearerFormat("JWT")
//                                        .in(SecurityScheme.In.HEADER)
//                                        .name(SECURTIY_SCHEME_API)))
                .info(
                        new io.swagger.v3.oas.models.info.Info()
                                .title("Solr Movie Writer MicroService")
                                .description("Solr Movie Writer MicroService")
                                .version("1.0.0")

                );
    }

//    @Bean
//    public OpenApiCustomiser securityItemCustomiser() {
//        return openApi -> openApi.getPaths().values().stream()
//                .flatMap(pathItem -> pathItem.readOperations().stream())
//                .forEach(operation -> {
//                    operation.addSecurityItem(new SecurityRequirement().addList(SECURTIY_SCHEME_API));
//                });
//    }

}
