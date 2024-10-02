package de.tuberlin.grpckotlin.entity.ipset

import org.springframework.data.annotation.Id
import org.springframework.data.relational.core.mapping.Column
import java.time.LocalDateTime

class IpSet(
    @Id
    val id: Long? = null,
    val name: String,
    val description: String,
    val owner: String,
    @Column("ips")
    private var _ips: String? = null,
    val createdAt: LocalDateTime = LocalDateTime.now()
) {
    var ips: List<String>
        get() = _ips?.split(",") ?: emptyList()
        set(value) {
            _ips = value.joinToString(",")
        }
}
