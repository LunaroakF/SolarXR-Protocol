// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

/**
 * If no tracker ID is given, it's the setting for every tracker/device
 */
@Suppress("unused")
class MagToggleRequest : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : MagToggleRequest {
        __init(_i, _bb)
        return this
    }
    val trackerId : solarxr_protocol.datatypes.TrackerId? get() = trackerId(solarxr_protocol.datatypes.TrackerId())
    fun trackerId(obj: solarxr_protocol.datatypes.TrackerId) : solarxr_protocol.datatypes.TrackerId? {
        val o = __offset(4)
        return if (o != 0) {
            obj.__assign(__indirect(o + bb_pos), bb)
        } else {
            null
        }
    }
    companion object {
        @JvmStatic
        fun validateVersion() = Constants.FLATBUFFERS_22_10_26()
        @JvmStatic
        fun getRootAsMagToggleRequest(_bb: ByteBuffer): MagToggleRequest = getRootAsMagToggleRequest(_bb, MagToggleRequest())
        @JvmStatic
        fun getRootAsMagToggleRequest(_bb: ByteBuffer, obj: MagToggleRequest): MagToggleRequest {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        @JvmStatic
        fun createMagToggleRequest(builder: FlatBufferBuilder, trackerIdOffset: Int) : Int {
            builder.startTable(1)
            addTrackerId(builder, trackerIdOffset)
            return endMagToggleRequest(builder)
        }
        @JvmStatic
        fun startMagToggleRequest(builder: FlatBufferBuilder) = builder.startTable(1)
        @JvmStatic
        fun addTrackerId(builder: FlatBufferBuilder, trackerId: Int) = builder.addOffset(0, trackerId, 0)
        @JvmStatic
        fun endMagToggleRequest(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
    }
}
