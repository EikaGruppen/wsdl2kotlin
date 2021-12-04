package inheritance.openclasses

data class WindowKt(
    val someInteger: Int,
) {
    fun toJava(): Window = Window()
        .also {
        it.someInteger = someInteger
    }

    companion object {
        fun fromJava(javaClass: Window): WindowKt = WindowKt(
            someInteger = javaClass.someInteger
        )
    }
}