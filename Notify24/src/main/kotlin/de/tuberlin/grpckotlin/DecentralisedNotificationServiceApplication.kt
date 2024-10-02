package de.tuberlin.grpckotlin

import com.nimbusds.jwt.SignedJWT
import org.flywaydb.core.Flyway
import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.autoconfigure.flyway.FlywayProperties
import org.springframework.boot.autoconfigure.r2dbc.R2dbcProperties
import org.springframework.boot.context.properties.EnableConfigurationProperties
import org.springframework.boot.runApplication
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.scheduling.annotation.EnableScheduling
import org.springframework.security.config.annotation.web.reactive.EnableWebFluxSecurity
import org.springframework.security.config.web.server.ServerHttpSecurity
import org.springframework.security.oauth2.jwt.Jwt
import org.springframework.security.oauth2.jwt.ReactiveJwtDecoder
import org.springframework.security.web.server.SecurityWebFilterChain
import reactor.core.publisher.Mono
import java.time.Instant


@SpringBootApplication
@EnableScheduling
@EnableWebFluxSecurity
class GrpcKotlinApplication {

    @Bean
    fun filterChainJwtSecure(http: ServerHttpSecurity): SecurityWebFilterChain {
        http
            .csrf { it.disable() }
            .authorizeExchange { authorizeExchange ->
                authorizeExchange.pathMatchers("/login/**", "/register/**", "/notification", "/notification/all", "/admin/**", "/actuator/**").permitAll()
                authorizeExchange.anyExchange().authenticated()
            }
            .oauth2ResourceServer { oauth2ResourceServer ->
                oauth2ResourceServer
                    .jwt { jwt ->
                        jwt
                            .jwtDecoder(jwtDecoder())
                    }
            }
        return http.build()
    }
    @Bean
    fun jwtDecoder(): ReactiveJwtDecoder {
        return ReactiveJwtDecoder { token: String ->
            val signedJWT = SignedJWT.parse(token)
            val header = signedJWT.header.toJSONObject()
            val claims = signedJWT.jwtClaimsSet.toJSONObject()
            claims["exp"] = Instant.ofEpochSecond((claims["exp"] as Long))
            claims["iat"] = Instant.ofEpochSecond((claims["iat"] as Long))
            val sub = signedJWT.jwtClaimsSet.subject
            claims["sub"] = sub
            Mono.just(Jwt.withTokenValue(token)
                .headers { headers -> headers.putAll(header) }
                .claims { claimSet -> claimSet.putAll(claims) }
                .build()
            )
        }
    }
}

@Configuration
@EnableConfigurationProperties(R2dbcProperties::class, FlywayProperties::class)
internal class DatabaseConfig {
    @Bean(initMethod = "migrate")
    fun flyway(flywayProperties: FlywayProperties, r2dbcProperties: R2dbcProperties): Flyway {
        return Flyway.configure()
            .dataSource(
                flywayProperties.url,
                r2dbcProperties.username,
                r2dbcProperties.password
            )
            .baselineOnMigrate(true)
            .load()
    }
}

fun main(args: Array<String>) {
    runApplication<GrpcKotlinApplication>(*args)
}
