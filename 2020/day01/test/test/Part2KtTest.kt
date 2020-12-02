package test


import main.runPart2
import org.junit.jupiter.api.Test

import org.junit.jupiter.api.Assertions.*

internal class Part2KtTest {

    @Test
    fun should_test_basic_example_part2() {
        val expected = 241861950
        val result = runPart2(listOf(
            1721,
            979,
            366,
            299,
            675,
            1456))
        assertEquals(expected, result)
    }
}