package parted

import dataclasses.CarKtParted
import dataclasses.DoorKt
import org.junit.jupiter.api.Test

class PartedTest {

    private val parted = CarKtParted(
        requiredInteger = 1,
        listOfInternalClasses = listOf(DoorKt(4)),
    )

    @Test
    fun `Should access delegated fields directly`() {
        parted.requiredInteger

        parted.copy(
            requiredInteger = 1
        )
    }
}