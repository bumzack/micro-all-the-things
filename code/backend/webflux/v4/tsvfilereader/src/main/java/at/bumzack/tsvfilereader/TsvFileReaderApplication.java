package at.bumzack.tsvfilereader;


import at.bumzack.common.microthingisregistry.RegisterMicroService;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.ComponentScan;

@SpringBootApplication
@ComponentScan(basePackages = {"at.bumzack.common.microthingisregistry", "at.bumzack.tsvfilereader"})
public class TsvFileReaderApplication {

    private final RegisterMicroService registerMicroService;

    public TsvFileReaderApplication(final RegisterMicroService registerMicroService) {
        this.registerMicroService = registerMicroService;
    }

    public static void main(String[] args) {
        try {
            SpringApplication.run(TsvFileReaderApplication.class, args);
        } catch (final Exception e) {
            System.out.println("error " + e.getMessage());
            System.out.println(e);
        }
    }
}
