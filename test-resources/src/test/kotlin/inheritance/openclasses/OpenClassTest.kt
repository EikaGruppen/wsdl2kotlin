package inheritance.openclasses

import dataclasses.Door
import dataclasses.DoorKt
import inheritance.otherpackage.CLevel2Kt
import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test
import java.math.BigDecimal

class OpenClassTest {

    private val kotlinCLevel = CLevelKt(1, WindowKt(10), 2, BigDecimal.TEN, DoorKt(4), 3)
    private val javaCLevel = CLevel().apply {
        a = 1
        b = 2
        bb = BigDecimal.TEN
        bbb = Door().apply { someInteger = 4 }
        c = 3
    }

    private val kotlinBLevel = BLevelImplKt(1, WindowKt(10), 2, BigDecimal.ONE, DoorKt(4))
    private val kotlinCLevel2 = CLevel2Kt(1, WindowKt(10), 2, BigDecimal.TEN, DoorKt(4), 3)

    private fun superResponse(): BLevel = javaCLevel

    private fun superRequest(): BLevelKt = kotlinCLevel

    @Test
    fun `Convert supertype to kotlin`() {

        val response = superResponse()

        val converted = BLevelKt.fromJava(response)

        assertThat(converted).isInstanceOf(CLevelKt::class.java)
        assertThat(converted as CLevelKt).isEqualTo(kotlinCLevel)
    }

    @Test
    fun `Convert supertype to java`() {

        val request = superRequest()

        val converted = request.toJava()
        assertThat(converted).isInstanceOf(CLevel::class.java)
        val t = converted as CLevel
        assertThat(t.a).isEqualTo(javaCLevel.a)
        assertThat(t.b).isEqualTo(javaCLevel.b)
        assertThat(t.bb).isEqualTo(javaCLevel.bb)
        assertThat(t.c).isEqualTo(javaCLevel.c)
    }

    @Test
    fun `Class with open class as field should accept open both the open class, and extentions of the open class`() {
        val withOpenclass = ClassWithBLevelKt(listOf(kotlinBLevel))
        assertThat(withOpenclass.bLevel[0]).isInstanceOf(BLevelImplKt::class.java)

        val withSubclassOfOpenclass = ClassWithBLevelKt(listOf(kotlinCLevel))
        assertThat(withSubclassOfOpenclass.bLevel[0]).isInstanceOf(CLevelKt::class.java)

        val withSubclassOfOpenclassFromOtherPackage = ClassWithBLevelKt(listOf(kotlinCLevel2))
        assertThat(withSubclassOfOpenclassFromOtherPackage.bLevel[0]).isInstanceOf(CLevel2Kt::class.java)
    }
}