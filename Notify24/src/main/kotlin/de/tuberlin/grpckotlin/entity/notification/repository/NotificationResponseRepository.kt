package de.tuberlin.grpckotlin.entity.notification.repository

import de.tuberlin.grpckotlin.entity.notification.NotificationResponse
import org.springframework.data.r2dbc.repository.Modifying
import org.springframework.data.r2dbc.repository.Query
import org.springframework.data.r2dbc.repository.R2dbcRepository
import org.springframework.data.repository.CrudRepository
import org.springframework.data.repository.reactive.ReactiveCrudRepository
import org.springframework.transaction.annotation.Transactional
import reactor.core.publisher.Mono
import java.time.LocalDateTime

interface NotificationResponseRepository : ReactiveCrudRepository<NotificationResponse, Long> {
    fun findByUuid(uuid: String): Mono<NotificationResponse>

    @Modifying
    @Transactional
    @Query("delete from `notify24`.notification_response n where n.created_at < :timestamp")
    fun deleteAllByCreatedAtBefore(timestamp: LocalDateTime): Mono<Void>
}