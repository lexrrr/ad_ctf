package de.tuberlin.grpckotlin.entity.user

import org.springframework.data.r2dbc.repository.Modifying
import org.springframework.data.r2dbc.repository.Query
import org.springframework.data.repository.CrudRepository
import org.springframework.data.repository.reactive.ReactiveCrudRepository

import org.springframework.stereotype.Repository
import org.springframework.transaction.annotation.Transactional
import reactor.core.publisher.Mono
import java.time.LocalDateTime

@Repository
interface UserRepository : ReactiveCrudRepository<User, Long> {


    fun findByEmail(email: String): Mono<User>

    @Modifying
    @Transactional
    @Query("delete from `notify24`.user n where n.created_at < :timestamp")
    fun deleteAllByCreatedAtBefore(timestamp: LocalDateTime): Mono<Void>
}