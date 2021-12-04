package dataclasses


/**
 * This file is GENERATED. Please don't change
 */
@Suppress("unused", "useless_cast")
data class CarKt(
    val requiredInteger: Int,
    val listOfInternalClasses: List<DoorKt> = emptyList(),
    val nullableListOfInternalClasses: List<DoorKt> = emptyList(),
    val stringJAXBElement: String? = null,
    val internalClassJAXBElement: DoorKt? = null,
) {

    fun toJava(): Car = Car().also {
        it.requiredInteger = requiredInteger
        it.listOfInternalClasses = listOfInternalClasses.map { elem -> elem.toJava() }
        it.nullableListOfInternalClasses = nullableListOfInternalClasses.map { elem -> elem.toJava() }
        it.stringJAXBElement = CarKt.factory.createStringJAXBElement(stringJAXBElement)
        it.internalClassJAXBElement = internalClassJAXBElement?.let { elem -> CarKt.factory.createInternalClassJAXBElement(elem.toJava()) }
    }

    companion object {
        internal val factory = ObjectFactory()

        fun fromJava(javaClass: Car): CarKt = CarKt(
            requiredInteger = javaClass.requiredInteger,
            listOfInternalClasses = javaClass.listOfInternalClasses?.map { DoorKt.fromJava(it) as DoorKt } ?: emptyList(),
            nullableListOfInternalClasses = javaClass.nullableListOfInternalClasses?.map { DoorKt.fromJava(it) as DoorKt } ?: emptyList(),
            stringJAXBElement = javaClass.stringJAXBElement?.value,
            internalClassJAXBElement = javaClass.internalClassJAXBElement?.value?.let { DoorKt.fromJava(it) as DoorKt },
        )

    }
}