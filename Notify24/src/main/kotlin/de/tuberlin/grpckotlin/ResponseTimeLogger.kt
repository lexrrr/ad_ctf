package de.tuberlin.grpckotlin

import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.stereotype.Component
import org.springframework.web.reactive.function.client.ClientRequest
import org.springframework.web.reactive.function.client.ClientResponse
import org.springframework.web.reactive.function.client.ExchangeFilterFunction
import org.springframework.web.reactive.function.client.ExchangeFunction
import reactor.core.publisher.Mono


@Component
class ResponseTimeLogger : ExchangeFilterFunction {
    override fun filter(request: ClientRequest, next: ExchangeFunction): Mono<ClientResponse> {
        return next.exchange(request).doOnEach { signal ->
            if (!signal.isOnComplete()) {
                val startTime: Long = signal.getContextView()
                    .get(METRICS_WEBCLIENT_START_TIME)
                val duration = System.currentTimeMillis() - startTime
                logger.info("Downstream called taken {}ms", duration)
            }
        }.contextWrite { ctx ->
            ctx.put(
                METRICS_WEBCLIENT_START_TIME,
                System.currentTimeMillis()
            )
        }
    }

    companion object {
        val logger: Logger = LoggerFactory.getLogger(ResponseTimeLogger::class.java)
        private val METRICS_WEBCLIENT_START_TIME = ResponseTimeLogger::class.java.name + ".START_TIME"
    }

}