package at.bumzack.foryouandyourfakewebshop.searcharticle.model;

import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;

import java.util.Collection;
import java.util.List;

public interface PriceEntryRepository extends JpaRepository<PriceEntry, Long> {
    @Query(nativeQuery = true, value = "SELECT * FROM price WHERE movie_tconst = :movieTconst")
    PriceEntry findByMovieTconst(@Param("movieTconst") String movieTconst);

    // https://javadeveloperzone.com/spring/spring-jpa-query-in-clause-example/
    @Query(nativeQuery = true, value = "SELECT * FROM price WHERE movie_tconst IN (:movieTconst)")
    List<PriceEntry> findPrices(@Param("movieTconst") Collection<String> movieTconst);

}
