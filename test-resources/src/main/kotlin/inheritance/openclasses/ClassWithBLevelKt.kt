package inheritance.openclasses


/**
 * This file is GENERATED. Please don't change
 */
@Suppress("unused", "useless_cast")
data class ClassWithBLevelKt(
    val bLevel: List<BLevelKt> = emptyList(),
) {

    fun toJava(): ClassWithBLevel = ClassWithBLevel().also {
        it.bLevel = bLevel.map { elem -> elem.toJava() }
    }

    companion object {
        internal val factory = ObjectFactory()

        fun fromJava(javaClass: ClassWithBLevel): ClassWithBLevelKt = ClassWithBLevelKt(
            bLevel = javaClass.bLevel?.map { BLevelKt.fromJava(it) as BLevelKt } ?: emptyList(),
        )

    }
}