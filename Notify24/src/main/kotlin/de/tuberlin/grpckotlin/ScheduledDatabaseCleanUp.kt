package de.tuberlin.grpckotlin

import de.tuberlin.grpckotlin.entity.ipset.repository.IpSetRepository
import de.tuberlin.grpckotlin.entity.notification.repository.NotificationResponseRepository
import de.tuberlin.grpckotlin.entity.notification.repository.ReceivedNotificationRepository
import de.tuberlin.grpckotlin.entity.user.UserRepository
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.scheduling.annotation.Scheduled
import org.springframework.stereotype.Component
import java.time.LocalDateTime


@Component
class ScheduledDatabaseCleanUp(
    private val notificationResponseRepository: NotificationResponseRepository,
    private val receivedNotificationRepository: ReceivedNotificationRepository,
    private val ipSetRepository: IpSetRepository,
    private val userRepository: UserRepository
) {

    companion object {
        val logger: Logger = LoggerFactory.getLogger(ScheduledDatabaseCleanUp::class.java)
    }

    @Scheduled(fixedRate = 120_000)
    fun deleteOldNotifications() {
        logger.info("Deleting old notifications")
        val timestamp = LocalDateTime.now().minusMinutes(15)
        notificationResponseRepository.deleteAllByCreatedAtBefore(timestamp)
            .then(receivedNotificationRepository.deleteAllByCreatedAtBefore(timestamp))
            .then(ipSetRepository.deleteAllByCreatedAtBefore(timestamp))
            .then(userRepository.deleteAllByCreatedAtBefore(timestamp))
            .subscribe()
    }
}