package inheritance.openclasses


/**
 * This file is GENERATED. Please don't change
 */
interface ALevelKt {
    val a: Int
    val aa: WindowKt?

    fun toJava(): ALevel = ALevel()
        .also {
            it.a = a
            it.aa = aa?.let { elem -> ALevelKt.factory.createAa(elem.toJava()) }
        }

    companion object {

        internal val factory = ObjectFactory()

        fun fromJava(javaClass: ALevel): ALevelKt = when (javaClass) {
            is BLevel -> BLevelKt.fromJava(javaClass)
            else -> ALevelImplKt(
                a = javaClass.a,
                aa = javaClass.aa?.value?.let { WindowKt.fromJava(it) as WindowKt },
            )
        }

    }
}

@Suppress("unused", "useless_cast")
data class ALevelImplKt(
    override val a: Int,
    override val aa: WindowKt? = null,
) : ALevelKt