package inheritance.openclasses


/**
 * This file is GENERATED. Please don't change
 */
interface ALevelEmptyKt {

    fun toJava(): ALevelEmpty = ALevelEmpty()
        .also {
        }

    companion object {

        internal val factory = ObjectFactory()

        fun fromJava(javaClass: ALevelEmpty): ALevelEmptyKt = when (javaClass) {
            is BLevelEmpty -> BLevelEmptyKt.fromJava(javaClass)
            else -> ALevelEmptyImplKt(
            )
        }

    }
}

@Suppress("unused", "useless_cast")
class ALevelEmptyImplKt(
) : ALevelEmptyKt