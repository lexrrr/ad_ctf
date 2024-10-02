package de.tuberlin.grpckotlin.entity.notification

import org.springframework.data.annotation.Id
import java.time.LocalDateTime
import java.util.*

class ReceivedNotification(
    val id: String? = null,
    val message: String,
    val receivedAt: LocalDateTime,
    val createdAt: LocalDateTime = LocalDateTime.now()
)