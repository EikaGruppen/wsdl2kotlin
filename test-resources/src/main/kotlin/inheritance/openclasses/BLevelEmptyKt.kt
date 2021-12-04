package inheritance.openclasses


/**
 * This file is GENERATED. Please don't change
 */
@Suppress("unused", "useless_cast")
data class BLevelEmptyKt(
    val b: Int,
) : ALevelEmptyKt {

    override fun toJava(): BLevelEmpty = BLevelEmpty()
        .also {
            it.b = b
        }

    companion object {

        internal val factory = ObjectFactory()

        fun fromJava(javaClass: BLevelEmpty): BLevelEmptyKt = BLevelEmptyKt(
            b = javaClass.b,
        )

    }
}