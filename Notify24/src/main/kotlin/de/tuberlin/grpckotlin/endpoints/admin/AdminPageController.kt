package de.tuberlin.grpckotlin.endpoints.admin

import de.tuberlin.grpckotlin.endpoints.notification.NotificationService
import de.tuberlin.grpckotlin.entity.notification.NotificationResponse
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RequestParam
import org.springframework.web.bind.annotation.RestController
import reactor.core.publisher.Mono

@RestController
@RequestMapping("admin")
class AdminPageController(
    private val notificationService: NotificationService
) {
    @GetMapping("/notification")
    fun getNotificationFromResponse(@RequestParam("uuid") uuid: String): Mono<NotificationResponse> {
        return notificationService.getResponseFromSentMessageByUUUD(uuid)
    }
}