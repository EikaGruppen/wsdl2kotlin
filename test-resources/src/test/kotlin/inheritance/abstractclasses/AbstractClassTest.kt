package inheritance.abstractclasses

import org.assertj.core.api.Assertions
import org.junit.jupiter.api.Test
import java.math.BigDecimal

class AbstractClassTest {

    private val kotlinCLevel = CLevelKt(1, 2, BigDecimal.TEN, 3)
    private val javaCLevel = CLevel().apply {
        a = 1
        b = 2
        bb = BigDecimal.TEN
        c = 3
    }

    private fun superResponse(): BLevel = javaCLevel

    private fun superRequest(): BLevelKt = kotlinCLevel

    @Test
    fun `Convert supertype to kotlin`() {

        val response = superResponse()

        val converted = BLevelKt.fromJava(response)

        Assertions.assertThat(converted).isInstanceOf(CLevelKt::class.java)
        Assertions.assertThat(converted as CLevelKt).isEqualTo(kotlinCLevel)
    }

    @Test
    fun `Convert supertype to java`() {

        val request = superRequest()

        val converted = request.toJava()
        Assertions.assertThat(converted).isInstanceOf(CLevel::class.java)
        val t = converted as CLevel
        Assertions.assertThat(t.a).isEqualTo(javaCLevel.a)
        Assertions.assertThat(t.b).isEqualTo(javaCLevel.b)
        Assertions.assertThat(t.bb).isEqualTo(javaCLevel.bb)
        Assertions.assertThat(t.c).isEqualTo(javaCLevel.c)
    }

}