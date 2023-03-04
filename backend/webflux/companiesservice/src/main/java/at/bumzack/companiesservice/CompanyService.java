package at.bumzack.app.service;


import at.bumzack.app.db.UsersRepository;
import at.bumzack.app.db.models.UserModel;
import at.bumzack.app.dto.UserDTO;
import io.swagger.v3.oas.annotations.tags.Tag;
import org.springframework.stereotype.Service;
import reactor.core.publisher.Mono;

import javax.validation.constraints.NotNull;
import java.util.function.Function;

@Service
@Tag(name = "UsersService")
public class UsersService {

    private final UsersRepository usersRepository;

    public UsersService(final UsersRepository usersRepository) {
        this.usersRepository = usersRepository;
    }

    public Mono<UserDTO> findByUsername(final String userName) {
        return usersRepository.findByUsername(userName)
                .map(mapToDTO());
    }

    public Mono<UserDTO> save(final UserModel u) {
        return usersRepository.save(u)
                .map(mapToDTO());
    }

    public Mono<UserDTO> findById(final Long userId) {
        return usersRepository.findById(userId)
                .map(mapToDTO());
    }

    @NotNull
    private Function<UserModel, UserDTO> mapToDTO() {
        return u -> new UserDTO(u.getId(), u.getRole(), u.getUsername(), u.getLanguageIsoCode(), u.getFirstName(), u.getLastName(), u.getPassword());
    }
}
