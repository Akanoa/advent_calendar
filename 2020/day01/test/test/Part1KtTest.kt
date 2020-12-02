package test

import main.runPart1
import org.junit.jupiter.api.Test

import org.junit.jupiter.api.Assertions.*

internal class Part1KtTest {

    @Test
    fun should_test_basic_example_part1() {
        val expected = 514579
        val result = runPart1(listOf(
            1721,
            979,
            366,
            299,
            675,
            1456))
        assertEquals(expected, result)
    }
}