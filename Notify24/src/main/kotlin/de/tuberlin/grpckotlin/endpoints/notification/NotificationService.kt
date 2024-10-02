package de.tuberlin.grpckotlin.endpoints.notification


import de.tuberlin.grpckotlin.entity.ipset.repository.IpSetRepository
import de.tuberlin.grpckotlin.entity.notification.NotificationResponse
import de.tuberlin.grpckotlin.entity.notification.ReceivedNotification
import de.tuberlin.grpckotlin.entity.notification.repository.NotificationResponseRepository
import de.tuberlin.grpckotlin.entity.notification.repository.ReceivedNotificationRepository
import io.netty.channel.ChannelOption
import io.netty.channel.ConnectTimeoutException
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.http.*
import org.springframework.http.client.reactive.ReactorClientHttpConnector
import org.springframework.security.core.context.ReactiveSecurityContextHolder
import org.springframework.stereotype.Service
import org.springframework.transaction.annotation.Transactional
import org.springframework.web.reactive.function.client.WebClient
import org.springframework.web.util.DefaultUriBuilderFactory
import reactor.core.publisher.Flux
import reactor.core.publisher.Mono
import reactor.netty.http.client.HttpClient
import java.time.Duration
import java.time.LocalDateTime
import java.util.*


@Service
class NotificationService(
    private val ipSetRepository: IpSetRepository,
    private val notificationResponseRepository: NotificationResponseRepository,
    private val receivedNotificationRepository: ReceivedNotificationRepository
) {
    companion object {
        val logger: Logger = LoggerFactory.getLogger(NotificationService::class.java)
    }


    @Transactional
    fun sendNotificationsByIpSet(message: String, ipSetId: Long): Flux<NotificationResponse> {
        return ReactiveSecurityContextHolder.getContext().flatMapMany { securityContext ->
            val username = securityContext.authentication.name
            val ipSet = ipSetRepository.findById(ipSetId).switchIfEmpty(Mono.error(IllegalArgumentException("IpSet not found")))

            ipSet.flatMapMany ipSetFlatMap@{ set ->
                if (set.owner != username) {
                    return@ipSetFlatMap Flux.error<NotificationResponse>(IllegalArgumentException("User does not own this IpSet"))
                }

                Flux.fromIterable(set.ips).flatMap { ip ->
                    sendNotification(ip, message)
                }
            }
        }
    }
    private fun sendNotification(ip: String, message: String): Mono<NotificationResponse> {
        val uuid: UUID = UUID.randomUUID()
        val factory = DefaultUriBuilderFactory()
        factory.encodingMode = DefaultUriBuilderFactory.EncodingMode.VALUES_ONLY
        val url = factory.uriString(ip)
            .path("notification")
            .queryParam("message", "{message}")
            .build(message)
            .toString()
        val client: HttpClient = HttpClient.create()
            .responseTimeout(Duration.ofSeconds(1))
            .option(ChannelOption.CONNECT_TIMEOUT_MILLIS, 500);
        val webClient = WebClient.builder()
            .clientConnector(ReactorClientHttpConnector(client))
            .build()

        return webClient.get()
            .uri(url)
            .retrieve()
            .onStatus(HttpStatusCode::isError) { Mono.error(RuntimeException("Error response from server")) }
            .bodyToMono(String::class.java)
            .flatMap { status ->
                val response = NotificationResponse(
                    recipient = ip,
                    message = message,
                    uuid = uuid.toString(),
                    status = status
                )

                notificationResponseRepository.save(response)
            }
            .onErrorResume { e ->
                logger.info("Could not establish connection to $url")
                val response = NotificationResponse(
                    recipient = ip,
                    message = message,
                    uuid = uuid.toString(),
                    status = "ERROR"
                )

                notificationResponseRepository.save(response)
            }


    }

    fun receiveNotification(message: String): Mono<String> {
        val id = UUID.randomUUID().toString()
        return receivedNotificationRepository.save(
            ReceivedNotification(
                id = id,
                message = message,
                receivedAt = LocalDateTime.now()
            )
        ).map { it.id.toString() }
    }

    fun getResponseFromSentMessageByUUUD(uuid: String): Mono<NotificationResponse> {
        return notificationResponseRepository.findByUuid(uuid)
    }

    fun getReceivedNotificationsFromPastMinutes(minutes: Long): Flux<ReceivedNotification> {
        return receivedNotificationRepository.findByReceivedAtAfter(LocalDateTime.now().minusMinutes(minutes))
    }

}

