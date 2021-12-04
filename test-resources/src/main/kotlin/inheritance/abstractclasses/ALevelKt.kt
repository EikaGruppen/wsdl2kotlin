package inheritance.abstractclasses


/**
 * This file is GENERATED. Please don't change
 */
interface ALevelKt {
    val a: Int

    fun toJava(): ALevel

    companion object {

        internal val factory = ObjectFactory()

        fun fromJava(javaClass: ALevel): ALevelKt = when (javaClass) {
            is BLevel -> BLevelKt.fromJava(javaClass)
            else -> throw IllegalStateException("Not able to find implementation for class '${javaClass.javaClass.name}'")
        }

    }
}