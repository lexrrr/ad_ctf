package de.tuberlin.grpckotlin.entity.notification.repository

import de.tuberlin.grpckotlin.entity.notification.ReceivedNotification
import org.springframework.data.r2dbc.repository.Modifying
import org.springframework.data.r2dbc.repository.Query
import org.springframework.data.r2dbc.repository.R2dbcRepository
import org.springframework.data.repository.CrudRepository
import org.springframework.data.repository.reactive.ReactiveCrudRepository
import org.springframework.transaction.annotation.Transactional
import reactor.core.publisher.Flux
import reactor.core.publisher.Mono
import java.time.LocalDateTime
import java.util.*


interface ReceivedNotificationRepository : ReactiveCrudRepository<ReceivedNotification, UUID> {

    fun findByReceivedAtAfter(dateTime: LocalDateTime) : Flux<ReceivedNotification>

    @Modifying
    @Transactional
    @Query("delete from `notify24`.received_notification n where n.created_at < :timestamp")
    fun deleteAllByCreatedAtBefore(timestamp: LocalDateTime): Mono<Void>
}