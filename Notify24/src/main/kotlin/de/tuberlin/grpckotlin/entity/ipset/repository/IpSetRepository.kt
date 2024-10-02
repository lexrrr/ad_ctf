package de.tuberlin.grpckotlin.entity.ipset.repository

import de.tuberlin.grpckotlin.entity.ipset.IpSet
import org.springframework.data.r2dbc.repository.Modifying
import org.springframework.data.r2dbc.repository.Query
import org.springframework.data.repository.CrudRepository
import org.springframework.data.repository.reactive.ReactiveCrudRepository
import org.springframework.transaction.annotation.Transactional
import reactor.core.publisher.Flux
import reactor.core.publisher.Mono
import java.time.LocalDateTime

interface IpSetRepository : ReactiveCrudRepository<IpSet, Long> {
    fun findByOwner(name: String): Flux<IpSet>
    fun findByIdAndOwner(id: Long, name: String): Mono<IpSet>

    @Modifying
    @Transactional
    @Query("delete from `notify24`.ip_set n where n.created_at < :timestamp")
    fun deleteAllByCreatedAtBefore(timestamp: LocalDateTime): Mono<Void>

}