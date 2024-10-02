package de.tuberlin.grpckotlin.entity.user

import org.springframework.data.annotation.Id
import org.springframework.security.core.userdetails.UserDetails
import java.time.LocalDateTime


class User(
    @Id
    val id: Long? = null,
    val email: String,
    val passwordHash: String,
    val createdAt: LocalDateTime = LocalDateTime.now()
) : UserDetails {
    override fun getAuthorities() = null
    override fun getPassword() = passwordHash
    override fun getUsername() = email
    override fun isAccountNonExpired() = true
    override fun isAccountNonLocked() = true
    override fun isCredentialsNonExpired() = true
    override fun isEnabled() = true
}