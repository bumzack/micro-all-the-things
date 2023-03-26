package at.bumzack.solr;


import at.bumzack.common.microthingisregistry.RegisterMicroService;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.ComponentScan;

@SpringBootApplication
@ComponentScan(basePackages = {"at.bumzack.common.microthingisregistry", "at.bumzack.solr"})
public class SolrEpisodeWriterApplication {

    private final RegisterMicroService registerMicroService;

    public SolrEpisodeWriterApplication(final RegisterMicroService registerMicroService) {
        this.registerMicroService = registerMicroService;
    }

    public static void main(String[] args) {
        try {
            SpringApplication.run(SolrEpisodeWriterApplication.class, args);
        } catch (final Exception e) {
            System.out.println("error " + e.getMessage());
            System.out.println(e);
        }
    }
}
