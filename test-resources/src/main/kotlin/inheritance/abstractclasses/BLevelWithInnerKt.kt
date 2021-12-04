package inheritance.abstractclasses

import java.math.BigDecimal

/**
 * This file is GENERATED. Please don't change
 */
interface BLevelWithInnerKt : ALevelWithInnerKt {
    val b: Int
    val bb: BigDecimal?

    override fun toJava(): BLevelWithInner

    companion object {

        internal val factory = ObjectFactory()

        fun fromJava(javaClass: BLevelWithInner): BLevelWithInnerKt = when (javaClass) {
            else -> throw IllegalStateException("Not able to find implementation for class '${javaClass.javaClass.name}'")
        }

    }
}