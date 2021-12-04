package inheritance.abstractclasses

import java.math.BigDecimal

/**
 * This file is GENERATED. Please don't change
 */
interface BLevelKt : ALevelKt {
    val b: Int
    val bb: BigDecimal?

    override fun toJava(): BLevel

    companion object {

        internal val factory = ObjectFactory()

        fun fromJava(javaClass: BLevel): BLevelKt = when (javaClass) {
            is CLevel -> CLevelKt.fromJava(javaClass)
            else -> throw IllegalStateException("Not able to find implementation for class '${javaClass.javaClass.name}'")
        }

    }
}