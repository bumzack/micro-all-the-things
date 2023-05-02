package at.bumzack.foryouandyourfakewebshop.searcharticle.model;

import org.springframework.data.r2dbc.repository.Query;
import org.springframework.data.repository.reactive.ReactiveCrudRepository;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import java.util.Collection;

public interface PriceEntryRepository extends ReactiveCrudRepository<PriceEntry, Long> {
    Mono<PriceEntry> findByMovieTconst(String movieTconst);

    @Query("SELECT * FROM price WHERE movie_tconst IN (:movieTconst)")
    Flux<PriceEntry> findMovies(Collection<String> movieTconst);

}
