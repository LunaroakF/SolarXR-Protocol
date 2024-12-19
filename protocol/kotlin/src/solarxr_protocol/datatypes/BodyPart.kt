// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.datatypes

/**
 * Different parts of the body. Maps to each possible non-tracker bone in the skeleton.
 * These are *NOT* the trackers.
 */
@Suppress("unused")
class BodyPart private constructor() {
    companion object {
        const val NONE: UByte = 0u
        const val HEAD: UByte = 1u
        const val NECK: UByte = 2u
        const val CHEST: UByte = 3u
        const val WAIST: UByte = 4u
        const val HIP: UByte = 5u
        const val LEFTUPPERLEG: UByte = 6u
        const val RIGHTUPPERLEG: UByte = 7u
        const val LEFTLOWERLEG: UByte = 8u
        const val RIGHTLOWERLEG: UByte = 9u
        const val LEFTFOOT: UByte = 10u
        const val RIGHTFOOT: UByte = 11u
        const val LEFTLOWERARM: UByte = 14u
        const val RIGHTLOWERARM: UByte = 15u
        const val LEFTUPPERARM: UByte = 16u
        const val RIGHTUPPERARM: UByte = 17u
        const val LEFTHAND: UByte = 18u
        const val RIGHTHAND: UByte = 19u
        const val LEFTSHOULDER: UByte = 20u
        const val RIGHTSHOULDER: UByte = 21u
        const val UPPERCHEST: UByte = 22u
        const val LEFTHIP: UByte = 23u
        const val RIGHTHIP: UByte = 24u
        const val LEFTTHUMBMETACARPAL: UByte = 25u
        const val LEFTTHUMBPROXIMAL: UByte = 26u
        const val LEFTTHUMBDISTAL: UByte = 27u
        const val LEFTINDEXPROXIMAL: UByte = 28u
        const val LEFTINDEXINTERMEDIATE: UByte = 29u
        const val LEFTINDEXDISTAL: UByte = 30u
        const val LEFTMIDDLEPROXIMAL: UByte = 31u
        const val LEFTMIDDLEINTERMEDIATE: UByte = 32u
        const val LEFTMIDDLEDISTAL: UByte = 33u
        const val LEFTRINGPROXIMAL: UByte = 34u
        const val LEFTRINGINTERMEDIATE: UByte = 35u
        const val LEFTRINGDISTAL: UByte = 36u
        const val LEFTLITTLEPROXIMAL: UByte = 37u
        const val LEFTLITTLEINTERMEDIATE: UByte = 38u
        const val LEFTLITTLEDISTAL: UByte = 39u
        const val RIGHTTHUMBMETACARPAL: UByte = 40u
        const val RIGHTTHUMBPROXIMAL: UByte = 41u
        const val RIGHTTHUMBDISTAL: UByte = 42u
        const val RIGHTINDEXPROXIMAL: UByte = 43u
        const val RIGHTINDEXINTERMEDIATE: UByte = 44u
        const val RIGHTINDEXDISTAL: UByte = 45u
        const val RIGHTMIDDLEPROXIMAL: UByte = 46u
        const val RIGHTMIDDLEINTERMEDIATE: UByte = 47u
        const val RIGHTMIDDLEDISTAL: UByte = 48u
        const val RIGHTRINGPROXIMAL: UByte = 49u
        const val RIGHTRINGINTERMEDIATE: UByte = 50u
        const val RIGHTRINGDISTAL: UByte = 51u
        const val RIGHTLITTLEPROXIMAL: UByte = 52u
        const val RIGHTLITTLEINTERMEDIATE: UByte = 53u
        const val RIGHTLITTLEDISTAL: UByte = 54u
        val names : Array<String> = arrayOf("NONE", "HEAD", "NECK", "CHEST", "WAIST", "HIP", "LEFT_UPPER_LEG", "RIGHT_UPPER_LEG", "LEFT_LOWER_LEG", "RIGHT_LOWER_LEG", "LEFT_FOOT", "RIGHT_FOOT", "", "", "LEFT_LOWER_ARM", "RIGHT_LOWER_ARM", "LEFT_UPPER_ARM", "RIGHT_UPPER_ARM", "LEFT_HAND", "RIGHT_HAND", "LEFT_SHOULDER", "RIGHT_SHOULDER", "UPPER_CHEST", "LEFT_HIP", "RIGHT_HIP", "LEFT_THUMB_METACARPAL", "LEFT_THUMB_PROXIMAL", "LEFT_THUMB_DISTAL", "LEFT_INDEX_PROXIMAL", "LEFT_INDEX_INTERMEDIATE", "LEFT_INDEX_DISTAL", "LEFT_MIDDLE_PROXIMAL", "LEFT_MIDDLE_INTERMEDIATE", "LEFT_MIDDLE_DISTAL", "LEFT_RING_PROXIMAL", "LEFT_RING_INTERMEDIATE", "LEFT_RING_DISTAL", "LEFT_LITTLE_PROXIMAL", "LEFT_LITTLE_INTERMEDIATE", "LEFT_LITTLE_DISTAL", "RIGHT_THUMB_METACARPAL", "RIGHT_THUMB_PROXIMAL", "RIGHT_THUMB_DISTAL", "RIGHT_INDEX_PROXIMAL", "RIGHT_INDEX_INTERMEDIATE", "RIGHT_INDEX_DISTAL", "RIGHT_MIDDLE_PROXIMAL", "RIGHT_MIDDLE_INTERMEDIATE", "RIGHT_MIDDLE_DISTAL", "RIGHT_RING_PROXIMAL", "RIGHT_RING_INTERMEDIATE", "RIGHT_RING_DISTAL", "RIGHT_LITTLE_PROXIMAL", "RIGHT_LITTLE_INTERMEDIATE", "RIGHT_LITTLE_DISTAL")
        @JvmStatic
        fun name(e: Int) : String = names[e]
    }
}
