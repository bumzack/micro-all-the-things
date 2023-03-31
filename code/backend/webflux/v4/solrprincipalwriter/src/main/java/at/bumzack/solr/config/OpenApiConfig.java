package at.bumzack.solr.config;


import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.servers.Server;
import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.info.License;
import org.springframework.context.annotation.Bean;
import org.springframework.stereotype.Service;

@OpenAPIDefinition(
        info = @Info(title = "Article Search Application", version = "1.0", description = "Article Search Application"),
        servers = {
                @Server(
                        description = "ATS Client Backend",
                        url = "http://localhost:8100")
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
                                .title("Article Search Application")
                                .description("Article Search Application")
                                .version("1.0.0")
                                .license(
                                        new License().name("MIT").url("https://opensource.org/licenses/MIT")
                                )
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
