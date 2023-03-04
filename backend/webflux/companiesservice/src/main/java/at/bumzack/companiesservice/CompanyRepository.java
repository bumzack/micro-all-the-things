package at.bumzack.app.db;


import at.bumzack.app.db.models.UserModel;
import org.springframework.data.repository.reactive.ReactiveCrudRepository;
import reactor.core.publisher.Mono;

public interface UsersRepository extends ReactiveCrudRepository<UserModel, Long> {

    Mono<UserModel> findByUsername(final String username);

    Mono<UserModel> findByIdAndPassword(Long userId, String pw);

}
