package de.tuberlin.grpckotlin.endpoints.authentication

import io.jsonwebtoken.Jwts
import org.springframework.stereotype.Component
import java.util.*
import javax.crypto.SecretKey


@Component
object JwtUtility {

    private var signingKey: SecretKey? = null
    fun generateAccessToken(email: String): String {

        return Jwts.builder()
            .subject(email)
            .issuer("Notify24")
            .issuedAt(Date())
            .claims()
            .add("scope", "DEFAULT_USER")
            .and()
            .expiration(Date(Date().time + (24 * 60 * 60 * 1000).toLong()))
            .signWith(getSigningKey())
            .compact()
    }

    fun getSigningKey(): SecretKey {
        if (signingKey == null) {
            val key = Jwts.SIG.HS256.key().build()
            signingKey = key
            return key
        }
        return signingKey!!
    }
}