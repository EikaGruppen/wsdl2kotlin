package inheritance.openclasses

import dataclasses.DoorKt
import java.math.BigDecimal

/**
 * This file is GENERATED. Please don't change
 */
@Suppress("unused", "useless_cast")
data class CLevelKt(
    override val a: Int,
    override val aa: WindowKt? = null,
    override val b: Int,
    override val bb: BigDecimal? = null,
    override val bbb: DoorKt,
    override val bbbb: List<String> = emptyList(),
    override val bbbbb: Boolean,
    val c: Int,
) : BLevelKt {

    override fun toJava(): CLevel = CLevel()
        .also {
            it.a = a
            it.aa = aa?.let { elem -> ALevelKt.factory.createAa(elem.toJava()) }
            it.b = b
            it.bb = bb
            it.bbb = bbb.toJava()
            it.bbbb = bbbb
            it.bbbbb = bbbbb
            it.c = c
        }

    companion object {

        internal val factory = ObjectFactory()

        fun fromJava(javaClass: CLevel): CLevelKt = CLevelKt(
            a = javaClass.a,
            aa = javaClass.aa?.value?.let { WindowKt.fromJava(it) as WindowKt },
            b = javaClass.b,
            bb = javaClass.bb,
            bbb = javaClass.bbb.let { DoorKt.fromJava(it) as DoorKt },
            bbbb = javaClass.bbbb ?: emptyList(),
            bbbbb = javaClass.bbbbb,
            c = javaClass.c,
        )

    }
}