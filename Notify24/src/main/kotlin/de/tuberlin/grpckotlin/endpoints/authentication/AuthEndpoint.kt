package de.tuberlin.grpckotlin.endpoints.authentication

import de.tuberlin.grpckotlin.entity.user.User
import de.tuberlin.grpckotlin.entity.user.UserRepository
import io.netty.handler.codec.http.cookie.Cookie
import jakarta.validation.Valid
import org.springframework.http.HttpStatus
import org.springframework.http.ResponseEntity
import org.springframework.http.server.reactive.ServerHttpRequest
import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RestController
import reactor.core.publisher.Mono
import java.security.Principal


data class AuthResponse(
    val email: String,
    val accessToken: String
)

data class AuthRequest(
    val email: String,
    val password: String
)

@RestController
class AuthEndpoint(
    private val jwtUtility: JwtUtility,
    private val userRepository: UserRepository
) {

    @PostMapping("/login")
    fun login(@RequestBody request: @Valid Mono<AuthRequest>, response: ServerHttpRequest): Mono<ResponseEntity<String>> {
        return request.flatMap { req ->
            userRepository.findByEmail(req.email).flatMap { foundUser ->
                if (BCryptPasswordEncoder().matches(req.password, foundUser.passwordHash)) {
                    val token = jwtUtility.generateAccessToken(req.email)
                    Mono.just(ResponseEntity.ok(token))
                } else Mono.just(ResponseEntity.status(HttpStatus.UNAUTHORIZED).body("Password incorrect!"))
            }.switchIfEmpty(Mono.just(ResponseEntity.status(HttpStatus.UNAUTHORIZED).body("User not found!")))
        }
    }

    @PostMapping("/register")
    fun register(@RequestBody request: @Valid Mono<AuthRequest>): Mono<ResponseEntity<AuthResponse>> {
        return request.flatMap { req ->
            userRepository.findByEmail(req.email).flatMap {
                Mono.just(ResponseEntity.status(HttpStatus.CONFLICT).build<AuthResponse>())
            }.switchIfEmpty(
                Mono.defer {
                    val user = User(email = req.email, passwordHash = BCryptPasswordEncoder().encode(req.password))
                    userRepository.save(user).thenReturn(
                        ResponseEntity.status(HttpStatus.CREATED).body(
                            AuthResponse(
                                email = req.email,
                                accessToken = jwtUtility.generateAccessToken(req.email)
                            )
                        )
                    )
                }
            )
        }
    }

    @GetMapping("/username")
    fun currentUserName(principal: Mono<Principal>): Mono<String> {
        return principal.map { it.name }
    }
}