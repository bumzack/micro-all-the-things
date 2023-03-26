package at.bumzack.solr;


import at.bumzack.common.microthingisregistry.RegisterMicroService;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.ComponentScan;

@SpringBootApplication
@ComponentScan(basePackages = {"at.bumzack.common.microthingisregistry", "at.bumzack.solr"})
public class SolrPersonWriterApplication {

    private final RegisterMicroService registerMicroService;

    public SolrPersonWriterApplication(final RegisterMicroService registerMicroService) {
        this.registerMicroService = registerMicroService;
    }

    public static void main(String[] args) {
        try {
            SpringApplication.run(SolrPersonWriterApplication.class, args);
        } catch (final Exception e) {
            System.out.println("error " + e.getMessage());
            System.out.println(e);
        }
    }
}
