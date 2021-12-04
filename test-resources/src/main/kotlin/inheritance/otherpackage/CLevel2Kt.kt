package inheritance.otherpackage

import dataclasses.DoorKt
import inheritance.openclasses.BLevelKt
import inheritance.openclasses.WindowKt
import java.math.BigDecimal
import inheritance.openclasses.ALevelKt

/**
 * This file is GENERATED. Please don't change
 */
@Suppress("unused", "useless_cast")
data class CLevel2Kt(
    override val a: Int,
    override val aa: WindowKt? = null,
    override val b: Int,
    override val bb: BigDecimal? = null,
    override val bbb: DoorKt,
    override val bbbb: List<String> = emptyList(),
    override val bbbbb: Boolean,
    val c: Int,
) : BLevelKt {

    override fun toJava(): CLevel2 = CLevel2()
        .also {
            it.a = a
            it.aa = aa?.let { elem -> ALevelKt.factory.createAa(elem.toJava()) }
            it.b = b
            it.bb = bb
            it.bbb = bbb.toJava()
            it.bbbb.addAll(bbbb)
            it.isBbbbb = bbbbb
            it.c = c
        }

    companion object {

        internal val factory = ObjectFactory()

        fun fromJava(javaClass: CLevel2): CLevel2Kt = CLevel2Kt(
            a = javaClass.a,
            aa = javaClass.aa?.value?.let { WindowKt.fromJava(it) as WindowKt },
            b = javaClass.b,
            bb = javaClass.bb,
            bbb = javaClass.bbb.let { DoorKt.fromJava(it) as DoorKt },
            bbbb = javaClass.bbbb,
            bbbbb = javaClass.isBbbbb,
            c = javaClass.c,
        )

    }
}