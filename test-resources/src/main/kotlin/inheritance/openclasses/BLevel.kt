package inheritance.openclasses

import dataclasses.DoorKt
import inheritance.otherpackage.CLevel2
import inheritance.otherpackage.CLevel2Kt
import java.math.BigDecimal

/**
 * This file is GENERATED. Please don't change
 */
interface BLevelKt : ALevelKt {
    val b: Int
    val bb: BigDecimal?
    val bbb: DoorKt
    val bbbb: List<String>
    val bbbbb: Boolean

    override fun toJava(): BLevel = BLevel()
        .also {
            it.a = a
            it.aa = aa?.let { elem -> ALevelKt.factory.createAa(elem.toJava()) }
            it.b = b
            it.bb = bb
            it.bbb = bbb.toJava()
            it.bbbb = bbbb
            it.bbbbb = bbbbb
        }

    companion object {

        internal val factory = ObjectFactory()

        fun fromJava(javaClass: BLevel): BLevelKt = when (javaClass) {
            is CLevel -> CLevelKt.fromJava(javaClass)
            is CLevel2 -> CLevel2Kt.fromJava(javaClass)
            else -> BLevelImplKt(
                a = javaClass.a,
                aa = javaClass.aa?.value?.let { WindowKt.fromJava(it) as WindowKt },
                b = javaClass.b,
                bb = javaClass.bb,
                bbb = javaClass.bbb.let { DoorKt.fromJava(it) as DoorKt },
                bbbb = javaClass.bbbb ?: emptyList(),
                bbbbb = javaClass.bbbbb,
            )
        }

    }
}

@Suppress("unused", "useless_cast")
data class BLevelImplKt(
    override val a: Int,
    override val aa: WindowKt? = null,
    override val b: Int,
    override val bb: BigDecimal? = null,
    override val bbb: DoorKt,
    override val bbbb: List<String> = emptyList(),
    override val bbbbb: Boolean,
) : BLevelKt