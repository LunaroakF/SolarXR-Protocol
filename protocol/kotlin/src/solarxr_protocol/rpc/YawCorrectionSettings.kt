// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

@Suppress("unused")
class YawCorrectionSettings : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : YawCorrectionSettings {
        __init(_i, _bb)
        return this
    }
    val enabled : Boolean
        get() {
            val o = __offset(4)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else false
        }
    val amountInDegPerSec : Float
        get() {
            val o = __offset(6)
            return if(o != 0) bb.getFloat(o + bb_pos) else 0.0f
        }
    companion object {
        @JvmStatic
        fun validateVersion() = Constants.FLATBUFFERS_22_10_26()
        @JvmStatic
        fun getRootAsYawCorrectionSettings(_bb: ByteBuffer): YawCorrectionSettings = getRootAsYawCorrectionSettings(_bb, YawCorrectionSettings())
        @JvmStatic
        fun getRootAsYawCorrectionSettings(_bb: ByteBuffer, obj: YawCorrectionSettings): YawCorrectionSettings {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        @JvmStatic
        fun createYawCorrectionSettings(builder: FlatBufferBuilder, enabled: Boolean, amountInDegPerSec: Float) : Int {
            builder.startTable(2)
            addAmountInDegPerSec(builder, amountInDegPerSec)
            addEnabled(builder, enabled)
            return endYawCorrectionSettings(builder)
        }
        @JvmStatic
        fun startYawCorrectionSettings(builder: FlatBufferBuilder) = builder.startTable(2)
        @JvmStatic
        fun addEnabled(builder: FlatBufferBuilder, enabled: Boolean) = builder.addBoolean(0, enabled, false)
        @JvmStatic
        fun addAmountInDegPerSec(builder: FlatBufferBuilder, amountInDegPerSec: Float) = builder.addFloat(1, amountInDegPerSec, 0.0)
        @JvmStatic
        fun endYawCorrectionSettings(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
    }
}
