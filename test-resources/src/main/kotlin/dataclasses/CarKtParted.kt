package dataclasses


/**
 * This file is GENERATED. Please don't change
 */
@Suppress("unused", "useless_cast")
data class CarKtParted private constructor(
    private val part0: Part0,
    private val part1: Part1,
    private val part2: Part2,
) {

    constructor(
        requiredInteger: Int,
        listOfInternalClasses: List<DoorKt> = emptyList(),
        nullableListOfInternalClasses: List<DoorKt> = emptyList(),
        stringJAXBElement: String? = null,
        internalClassJAXBElement: DoorKt? = null,
    ) : this(
        Part0(
            requiredInteger,
            listOfInternalClasses,
        ),
        Part1(
            nullableListOfInternalClasses,
            stringJAXBElement,
        ),
        Part2(
            internalClassJAXBElement,
        ),
    )

    fun toJava(): Car = Car().apply {
        part0.toJava(this)
        part1.toJava(this)
        part2.toJava(this)
    }

    companion object {
        internal val factory = ObjectFactory()

        fun fromJava(javaClass: Car): CarKtParted = CarKtParted(
            Part0.fromJava(javaClass),
            Part1.fromJava(javaClass),
            Part2.fromJava(javaClass),
        )
    }

    private data class Part0(
        val requiredInteger: Int,
        val listOfInternalClasses: List<DoorKt> = emptyList(),
    ) {

        fun toJava(javaClass: Car): Car = javaClass.also {
            it.requiredInteger = requiredInteger
            it.listOfInternalClasses = listOfInternalClasses.map { elem -> elem.toJava() }
        }

        companion object {
            internal val factory = ObjectFactory()

            fun fromJava(javaClass: Car): Part0 = Part0(
                requiredInteger = javaClass.requiredInteger,
                listOfInternalClasses = javaClass.listOfInternalClasses?.map { DoorKt.fromJava(it) as DoorKt } ?: emptyList(),
            )
        }
    }

    private data class Part1(
        val nullableListOfInternalClasses: List<DoorKt> = emptyList(),
        val stringJAXBElement: String? = null,
    ) {

        fun toJava(javaClass: Car): Car = javaClass.also {
            it.nullableListOfInternalClasses = nullableListOfInternalClasses.map { elem -> elem.toJava() }
            it.stringJAXBElement = CarKt.factory.createStringJAXBElement(stringJAXBElement)
        }

        companion object {
            internal val factory = ObjectFactory()

            fun fromJava(javaClass: Car): Part1 = Part1(
                nullableListOfInternalClasses = javaClass.nullableListOfInternalClasses?.map { DoorKt.fromJava(it) as DoorKt } ?: emptyList(),
                stringJAXBElement = javaClass.stringJAXBElement?.value,
            )
        }
    }

    private data class Part2(
        val internalClassJAXBElement: DoorKt? = null,
    ) {

        fun toJava(javaClass: Car): Car = javaClass.also {
            it.internalClassJAXBElement = internalClassJAXBElement?.let { elem -> CarKt.factory.createInternalClassJAXBElement(elem.toJava()) }
        }

        companion object {
            internal val factory = ObjectFactory()

            fun fromJava(javaClass: Car): Part2 = Part2(
                internalClassJAXBElement = javaClass.internalClassJAXBElement?.value?.let { DoorKt.fromJava(it) as DoorKt },
            )
        }
    }
}