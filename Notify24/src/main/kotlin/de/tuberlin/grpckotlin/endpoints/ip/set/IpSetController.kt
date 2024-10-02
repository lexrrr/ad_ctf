package de.tuberlin.grpckotlin.endpoints.ip.set

import de.tuberlin.grpckotlin.entity.ipset.IpSet
import de.tuberlin.grpckotlin.entity.ipset.repository.IpSetRepository
import org.springframework.security.core.context.ReactiveSecurityContextHolder
import org.springframework.security.core.context.SecurityContextHolder
import org.springframework.web.bind.annotation.*
import reactor.core.publisher.Flux
import reactor.core.publisher.Mono


data class IpSetCreateRequest(
    val name: String,
    val description: String,
    val ips: List<String>
)

@RestController
@RequestMapping("ip-set")
class IpSetController(private val ipSetRepository: IpSetRepository) {

    @PostMapping
    fun createIpSet(@RequestBody ipSetRequest: Mono<IpSetCreateRequest>): Mono<IpSet> {
        return ipSetRequest.flatMap { request ->
            ReactiveSecurityContextHolder.getContext()
                .map { it.authentication.name }
                .flatMap { username ->
                    val validHttpIps = request.ips.map { "http://$it" }
                    val ipSet = IpSet(name = request.name, description = request.description, owner = username)
                    ipSet.ips = validHttpIps
                    ipSetRepository.save(ipSet)
                }
        }
    }

    @GetMapping("/{id}")
    fun getIpSetById(@PathVariable id: Long): Mono<IpSet> {
        return ReactiveSecurityContextHolder.getContext()
            .map { it.authentication.name }
            .flatMap { username ->
                ipSetRepository.findByIdAndOwner(id, username)
                    .switchIfEmpty(Mono.error(IllegalArgumentException("IpSet not found")))
            }
    }

    @GetMapping
    fun getAllIpSets(): Flux<IpSet> {
        return ReactiveSecurityContextHolder.getContext()
            .map { it.authentication.name }
            .flatMapMany { username ->
                ipSetRepository.findByOwner(username)
            }
    }
}