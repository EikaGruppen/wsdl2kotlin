package dataclasses


/**
 * This file is GENERATED. Please don't change
 */
@Suppress("unused", "useless_cast")
data class CxfMapKt(
    val entry: List<EntryKt> = emptyList(),
) {

    fun toJava(): CxfMap = CxfMap().also {
        it.entry = entry.map { elem -> elem.toJava() }
    }

    companion object {
        internal val factory = ObjectFactory()

        fun fromJava(javaClass: CxfMap): CxfMapKt = CxfMapKt(
            entry = javaClass.entry?.map { EntryKt.fromJava(it) as EntryKt } ?: emptyList(),
        )

    }

    data class EntryKt(
        val key: String? = null,
        val value: DoorKt? = null,
    ) {

        fun toJava(): CxfMap.Entry = CxfMap.Entry().also {
            it.key = key
            it.value = value?.toJava()
        }

        companion object {
            internal val factory = ObjectFactory()

            fun fromJava(javaClass: CxfMap.Entry): EntryKt = EntryKt(
                key = javaClass.key,
                value = javaClass.value?.let { DoorKt.fromJava(it) as DoorKt },
            )

        }
    }
}