package dataclasses

data class DoorKt(
    val someInteger: Int,
) {
    fun toJava(): Door = Door().also {
        it.someInteger = someInteger
    }

    companion object {
        fun fromJava(javaClass: Door): DoorKt = DoorKt(
            someInteger = javaClass.someInteger
        )
    }
}