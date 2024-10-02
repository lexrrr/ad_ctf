package de.tuberlin.grpckotlin.endpoints.notification

import de.tuberlin.grpckotlin.entity.notification.NotificationResponse
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.web.bind.annotation.*
import reactor.core.publisher.Flux
import reactor.core.publisher.Mono
import java.net.URLDecoder
import java.nio.charset.StandardCharsets


@RestController
@RequestMapping("notification")
class NotificationController(
    private val notificationService: NotificationService
) {

    companion object {
        val logger: Logger = LoggerFactory.getLogger(NotificationController::class.java)
    }

    @GetMapping
    fun receiveNotification(@RequestParam("message") message: String): Mono<String> {
        val decodedMessage = URLDecoder.decode(message, StandardCharsets.UTF_8.toString())
        return notificationService.receiveNotification(decodedMessage)
            .doOnNext { logger.info("Received notification: $decodedMessage") }
            .thenReturn("Notification received")
    }

    @GetMapping("/send")
    fun sendNotifications(
        @RequestParam("notification") notification: String,
        @RequestParam("ipSetId") ipSetId: Long
    ): Flux<NotificationResponse> {
        return notificationService.sendNotificationsByIpSet(notification, ipSetId)
    }

    @GetMapping("/all")
    fun getAllNotifications() = notificationService.getReceivedNotificationsFromPastMinutes(14400)
}