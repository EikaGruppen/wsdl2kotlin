package dataclasses


private sealed interface CarKtPartedPart0 {
    val requiredInteger: Int
    val listOfInternalClasses: List<DoorKt>
}
private sealed interface CarKtPartedPart1 {
    val nullableListOfInternalClasses: List<DoorKt>
    val stringJAXBElement: String?
}
private sealed interface CarKtPartedPart2 {
    val internalClassJAXBElement: DoorKt?
}

/**
 * This file is GENERATED. Please don't change
 */
@Suppress("unused", "useless_cast")
data class CarKtParted private constructor(
    private val part0: Part0,
    private val part1: Part1,
    private val part2: Part2,
): CarKtPartedPart0 by part0, CarKtPartedPart1 by part1, CarKtPartedPart2 by part2 {

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

    fun copy(
        requiredInteger: Int = this.requiredInteger,
        listOfInternalClasses: List<DoorKt> = this.listOfInternalClasses,
        nullableListOfInternalClasses: List<DoorKt> = this.nullableListOfInternalClasses,
        stringJAXBElement: String? = this.stringJAXBElement,
        internalClassJAXBElement: DoorKt? = this.internalClassJAXBElement,
    ) = CarKtParted(
        requiredInteger,
        listOfInternalClasses,
        nullableListOfInternalClasses,
        stringJAXBElement,
        internalClassJAXBElement,
    )

    companion object {
        internal val factory = ObjectFactory()

        fun fromJava(javaClass: Car): CarKtParted = CarKtParted(
            Part0.fromJava(javaClass),
            Part1.fromJava(javaClass),
            Part2.fromJava(javaClass),
        )
    }

    private data class Part0(
        override val requiredInteger: Int,
        override val listOfInternalClasses: List<DoorKt> = emptyList(),
    ): CarKtPartedPart0 {

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
        override val nullableListOfInternalClasses: List<DoorKt> = emptyList(),
        override val stringJAXBElement: String? = null,
    ): CarKtPartedPart1 {

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
        override val internalClassJAXBElement: DoorKt? = null,
    ): CarKtPartedPart2 {

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