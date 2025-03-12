// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.datatypes

@Suppress("unused")
class PacketErrorCode private constructor() {
    companion object {
        const val NOTAPPLICABLE: UByte = 0u
        const val POWERONRESET: UByte = 1u
        const val INTERNALSYSTEMRESET: UByte = 2u
        const val WATCHDOGTIMEOUT: UByte = 3u
        const val EXTERNALRESET: UByte = 4u
        const val OTHER: UByte = 5u
        val names : Array<String> = arrayOf("NOT_APPLICABLE", "POWER_ON_RESET", "INTERNAL_SYSTEM_RESET", "WATCHDOG_TIMEOUT", "EXTERNAL_RESET", "OTHER")
        @JvmStatic
        fun name(e: Int) : String = names[e]
    }
}
