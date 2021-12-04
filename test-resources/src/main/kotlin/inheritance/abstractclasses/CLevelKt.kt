package inheritance.abstractclasses

import java.math.BigDecimal

/**
 * This file is GENERATED. Please don't change
 */
@Suppress("unused", "useless_cast")
data class CLevelKt(
    override val a: Int,
    override val b: Int,
    override val bb: BigDecimal? = null,
    val c: Int,
) : BLevelKt {

    override fun toJava(): CLevel = CLevel()
        .also {
            it.a = a
            it.b = b
            it.bb = bb
            it.c = c
        }

    companion object {

        internal val factory = ObjectFactory()

        fun fromJava(javaClass: CLevel): CLevelKt = CLevelKt(
            a = javaClass.a,
            b = javaClass.b,
            bb = javaClass.bb,
            c = javaClass.c,
        )

    }
}