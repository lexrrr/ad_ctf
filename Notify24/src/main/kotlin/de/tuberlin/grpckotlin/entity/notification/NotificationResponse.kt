package de.tuberlin.grpckotlin.entity.notification

import org.springframework.data.annotation.Id
import java.time.LocalDateTime

class NotificationResponse(
    @Id
    val id: Long? = null,
    var recipient: String,
    var message: String,
    var sendAt: LocalDateTime? = null,
    val status: String,
    val uuid: String?,
    val createdAt: LocalDateTime = LocalDateTime.now()
)