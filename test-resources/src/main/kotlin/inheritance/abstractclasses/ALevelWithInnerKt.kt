package inheritance.abstractclasses


/**
 * This file is GENERATED. Please don't change
 */
interface ALevelWithInnerKt {
    val a: Int
    val inner: InnerKt

    fun toJava(): ALevelWithInner

    companion object {

        internal val factory = ObjectFactory()

        fun fromJava(javaClass: ALevelWithInner): ALevelWithInnerKt = when (javaClass) {
            is BLevelWithInner -> BLevelWithInnerKt.fromJava(javaClass)
            else -> throw IllegalStateException("Not able to find implementation for class '${javaClass.javaClass.name}'")
        }

    }

    data class InnerKt(
        val ia: String? = null,
    ) {

        fun toJava(): ALevelWithInner.Inner = ALevelWithInner.Inner().also {
            it.ia = ia
        }

        companion object {
            internal val factory = ObjectFactory()

            fun fromJava(javaClass: ALevelWithInner.Inner): InnerKt = InnerKt(
                ia = javaClass.ia,
            )

        }
    }
}